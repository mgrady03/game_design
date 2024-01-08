use std::{
    collections::{btree_map, BTreeMap, BTreeSet, HashMap},
    time::Duration,
};

use bytes::Bytes;

use super::SliceConstructor;
use crate::{
    error::ChannelError,
    packet::{Packet, Slice, SLICE_SIZE},
};

#[derive(Debug)]
enum UnackedMessage {
    Small {
        message: Bytes,
        last_sent: Option<Duration>,
    },
    Sliced {
        message: Bytes,
        num_slices: usize,
        num_acked_slices: usize,
        next_slice_to_send: usize,
        acked: Vec<bool>,
        last_sent: Vec<Option<Duration>>,
    },
}

#[derive(Debug)]
pub struct SendChannelReliable {
    channel_id: u8,
    unacked_messages: BTreeMap<u64, UnackedMessage>,
    next_reliable_message_id: u64,
    resend_time: Duration,
    max_memory_usage_bytes: usize,
    memory_usage_bytes: usize,
}

#[derive(Debug)]
enum ReliableOrder {
    Ordered,
    Unordered {
        most_recent_message_id: u64,
        received_messages: BTreeSet<u64>,
    },
}

#[derive(Debug)]
pub struct ReceiveChannelReliable {
    slices: HashMap<u64, SliceConstructor>,
    messages: BTreeMap<u64, Bytes>,
    oldest_pending_message_id: u64,
    reliable_order: ReliableOrder,
    memory_usage_bytes: usize,
    max_memory_usage_bytes: usize,
}

impl UnackedMessage {
    fn new_sliced(payload: Bytes) -> Self {
        let num_slices = (payload.len() + SLICE_SIZE - 1) / SLICE_SIZE;

        Self::Sliced {
            message: payload,
            num_slices,
            num_acked_slices: 0,
            next_slice_to_send: 0,
            acked: vec![false; num_slices],
            last_sent: vec![None; num_slices],
        }
    }
}

impl SendChannelReliable {
    pub fn new(channel_id: u8, resend_time: Duration, max_memory_usage_bytes: usize) -> Self {
        Self {
            channel_id,
            unacked_messages: BTreeMap::new(),
            next_reliable_message_id: 0,
            resend_time,
            max_memory_usage_bytes,
            memory_usage_bytes: 0,
        }
    }

    pub fn get_packets_to_send(&mut self, packet_sequence: &mut u64, available_bytes: &mut u64, current_time: Duration) -> Vec<Packet> {
        if self.unacked_messages.is_empty() {
            return vec![];
        }

        let mut packets: Vec<Packet> = vec![];

        let mut small_messages: Vec<(u64, Bytes)> = vec![];
        let mut small_messages_bytes = 0;

        'messages: for (&message_id, unacked_message) in self.unacked_messages.iter_mut() {
            match unacked_message {
                UnackedMessage::Small { message, last_sent } => {
                    if *available_bytes < message.len() as u64 {
                        // Skip message, no bytes available to send this message
                        continue;
                    }

                    if let Some(last_sent) = last_sent {
                        if current_time - *last_sent < self.resend_time {
                            continue;
                        }
                    }

                    *available_bytes -= message.len() as u64;

                    // Generate packet with small messages if you cannot fit
                    let serialized_size = message.len() + octets::varint_len(message.len() as u64) + octets::varint_len(message_id);
                    if small_messages_bytes + serialized_size > SLICE_SIZE {
                        packets.push(Packet::SmallReliable {
                            sequence: *packet_sequence,
                            channel_id: self.channel_id,
                            messages: std::mem::take(&mut small_messages),
                        });
                        small_messages_bytes = 0;
                        *packet_sequence += 1;
                    }

                    small_messages_bytes += serialized_size;
                    small_messages.push((message_id, message.clone()));
                    *last_sent = Some(current_time);

                    continue;
                }
                UnackedMessage::Sliced {
                    message,
                    num_slices,
                    acked,
                    last_sent,
                    next_slice_to_send,
                    ..
                } => {
                    let start_index = *next_slice_to_send;
                    for i in 0..*num_slices {
                        if *available_bytes < SLICE_SIZE as u64 {
                            // Skip message, no bytes available to send a slice
                            continue 'messages;
                        }

                        let i = (start_index + i) % *num_slices;
                        if acked[i] {
                            continue;
                        }

                        if let Some(last_sent) = last_sent[i] {
                            if current_time - last_sent < self.resend_time {
                                continue;
                            }
                        }

                        let start = i * SLICE_SIZE;
                        let end = if i == *num_slices - 1 { message.len() } else { (i + 1) * SLICE_SIZE };

                        let payload = message.slice(start..end);
                        *available_bytes -= payload.len() as u64;

                        let slice = Slice {
                            message_id,
                            slice_index: i,
                            num_slices: *num_slices,
                            payload,
                        };

                        packets.push(Packet::ReliableSlice {
                            sequence: *packet_sequence,
                            channel_id: self.channel_id,
                            slice,
                        });

                        *packet_sequence += 1;
                        last_sent[i] = Some(current_time);
                        *next_slice_to_send = i + 1 % *num_slices;
                    }
                }
            }
        }

        // Generate final packet for remaining small messages
        if !small_messages.is_empty() {
            packets.push(Packet::SmallReliable {
                sequence: *packet_sequence,
                channel_id: self.channel_id,
                messages: std::mem::take(&mut small_messages),
            });
            *packet_sequence += 1;
        }

        packets
    }

    pub fn send_message(&mut self, message: Bytes) -> Result<(), ChannelError> {
        if self.memory_usage_bytes + message.len() > self.max_memory_usage_bytes {
            return Err(ChannelError::ReliableChannelMaxMemoryReached);
        }

        self.memory_usage_bytes += message.len();
        let unacked_message = if message.len() > SLICE_SIZE {
            UnackedMessage::new_sliced(message)
        } else {
            UnackedMessage::Small { message, last_sent: None }
        };

        self.unacked_messages.insert(self.next_reliable_message_id, unacked_message);
        self.next_reliable_message_id += 1;

        Ok(())
    }

    pub fn process_message_ack(&mut self, message_id: u64) {
        if self.unacked_messages.contains_key(&message_id) {
            let unacked_message = self.unacked_messages.remove(&message_id).unwrap();
            let UnackedMessage::Small { message: payload, .. } = unacked_message else {
                unreachable!("called ack on small message but found sliced");
            };
            self.memory_usage_bytes -= payload.len();
        }
    }

    pub fn process_slice_message_ack(&mut self, message_id: u64, slice_index: usize) {
        let Some(unacked_message) = self.unacked_messages.get_mut(&message_id) else {
            return;
        };

        let UnackedMessage::Sliced { message, num_slices, num_acked_slices, acked, .. } = unacked_message else {
            unreachable!("called ack on sliced message but found small");
        };

        if acked[slice_index] {
            return;
        }

        acked[slice_index] = true;
        *num_acked_slices += 1;

        if *num_acked_slices == *num_slices {
            self.memory_usage_bytes -= message.len();
            self.unacked_messages.remove(&message_id);
        }
    }
}

impl ReceiveChannelReliable {
    pub fn new(max_memory_usage_bytes: usize, ordered: bool) -> Self {
        let reliable_order = match ordered {
            true => ReliableOrder::Ordered,
            false => ReliableOrder::Unordered {
                most_recent_message_id: 0,
                received_messages: BTreeSet::new(),
            },
        };
        Self {
            slices: HashMap::new(),
            messages: BTreeMap::new(),
            oldest_pending_message_id: 0,
            reliable_order,
            memory_usage_bytes: 0,
            max_memory_usage_bytes,
        }
    }

    pub fn process_message(&mut self, message: Bytes, message_id: u64) -> Result<(), ChannelError> {
        if message_id < self.oldest_pending_message_id {
            // Discard old message already received
            return Ok(());
        }

        match &mut self.reliable_order {
            ReliableOrder::Ordered => {
                if let btree_map::Entry::Vacant(entry) = self.messages.entry(message_id) {
                    self.memory_usage_bytes += message.len();
                    if self.max_memory_usage_bytes < self.memory_usage_bytes {
                        return Err(ChannelError::ReliableChannelMaxMemoryReached);
                    }

                    entry.insert(message);
                }
            }
            ReliableOrder::Unordered {
                most_recent_message_id,
                received_messages,
            } => {
                if *most_recent_message_id < message_id {
                    *most_recent_message_id = message_id;
                }

                if !received_messages.contains(&message_id) {
                    self.memory_usage_bytes += message.len();
                    if self.max_memory_usage_bytes < self.memory_usage_bytes {
                        return Err(ChannelError::ReliableChannelMaxMemoryReached);
                    }

                    received_messages.insert(message_id);
                    self.messages.insert(message_id, message);
                }
            }
        }

        Ok(())
    }

    pub fn process_slice(&mut self, slice: Slice) -> Result<(), ChannelError> {
        if self.messages.contains_key(&slice.message_id) || slice.message_id < self.oldest_pending_message_id {
            // Message already assembled
            return Ok(());
        }

        if !self.slices.contains_key(&slice.message_id) {
            self.memory_usage_bytes += slice.num_slices * SLICE_SIZE;
            if self.max_memory_usage_bytes < self.memory_usage_bytes {
                return Err(ChannelError::ReliableChannelMaxMemoryReached);
            }
        }

        let slice_constructor = self
            .slices
            .entry(slice.message_id)
            .or_insert_with(|| SliceConstructor::new(slice.message_id, slice.num_slices));

        if let Some(message) = slice_constructor.process_slice(slice.slice_index, &slice.payload)? {
            // Memory usage is re-added with the exactly message size
            self.memory_usage_bytes -= slice.num_slices * SLICE_SIZE;
            self.process_message(message, slice.message_id)?;
            self.slices.remove(&slice.message_id);
        }

        Ok(())
    }

    pub fn receive_message(&mut self) -> Option<Bytes> {
        match &mut self.reliable_order {
            ReliableOrder::Ordered => {
                let Some(message) = self.messages.remove(&self.oldest_pending_message_id) else {
                    return None;
                };

                self.oldest_pending_message_id += 1;
                self.memory_usage_bytes -= message.len();
                Some(message)
            }
            ReliableOrder::Unordered { received_messages, .. } => {
                let Some((message_id, message)) = self.messages.pop_first() else {
                    return None;
                };

                if self.oldest_pending_message_id == message_id {
                    // Remove all next items that could have been received out of order,
                    // until we find an message that was not received
                    while received_messages.contains(&self.oldest_pending_message_id) {
                        received_messages.remove(&self.oldest_pending_message_id);
                        self.oldest_pending_message_id += 1;
                    }
                }

                self.memory_usage_bytes -= message.len();
                Some(message)
            }
        }
    }
}