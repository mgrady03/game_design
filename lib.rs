mod client;
mod crypto;
mod error;
mod packet;
mod serialize;
mod server;
mod token;

pub use client::{DisconnectReason, NetcodeClient};
pub use crypto::generate_random_bytes;
pub use error::NetcodeError;
pub use server::{NetcodeServer, ServerResult};
pub use token::{ConnectToken, TokenGenerationError};

use std::time::Duration;

type ClientID = u64;

const NETCODE_VERSION_INFO: &[u8; 13] = b"NETCODE 1.02\0";
const NETCODE_MAX_CLIENTS: usize = 1024;
const NETCODE_MAX_PENDING_CLIENTS: usize = NETCODE_MAX_CLIENTS * 4;

const NETCODE_ADDRESS_NONE: u8 = 0;
const NETCODE_ADDRESS_IPV4: u8 = 1;
const NETCODE_ADDRESS_IPV6: u8 = 2;

const NETCODE_CONNECT_TOKEN_PRIVATE_BYTES: usize = 1024;
/// The maximum number of bytes that a netcode packet can contain.
pub const NETCODE_MAX_PACKET_BYTES: usize = 1400;
/// The maximum number of bytes that a payload can have when generating a payload packet.
pub const NETCODE_MAX_PAYLOAD_BYTES: usize = 1300;

/// The number of bytes in a private key;
pub const NETCODE_KEY_BYTES: usize = 32;
const NETCODE_MAC_BYTES: usize = 16;
/// The number of bytes that an user data can contain in the ConnectToken.
pub const NETCODE_USER_DATA_BYTES: usize = 256;
const NETCODE_CHALLENGE_TOKEN_BYTES: usize = 300;
const NETCODE_CONNECT_TOKEN_XNONCE_BYTES: usize = 24;

const NETCODE_ADDITIONAL_DATA_SIZE: usize = 13 + 8 + 8;

const NETCODE_TIMEOUT_SECONDS: i32 = 15;

const NETCODE_SEND_RATE: Duration = Duration::from_millis(250);
