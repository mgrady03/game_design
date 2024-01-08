use bevy::prelude::*;

use crate::{GameStates,components::menus::*,resources::menus::*};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub fn main_menu(mut commands: Commands,asset_server: Res<AssetServer>) {
    let bg_image = commands.spawn(SpriteBundle {
        texture: asset_server.load("menu.png"),
        transform: Transform::from_xyz(0., 0., -100.),
        ..default()
    }).id();
    let host_button = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px((720.*0.1)-(65./2.)),
                left: Val::Px((1280.*0.9)-(150./2.)),
                display: Display::Flex,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Host",
                        TextStyle {
                            font: asset_server.load("AovelSansRounded-rdDL.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                }).insert(HostButtonMarker);
        }).id();
        let join_button = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px((720.*0.25)-(65./2.)),
                left: Val::Px((1280.*0.9)-(150./2.)-150.),
                display: Display::Flex,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Join",
                        TextStyle {
                            font: asset_server.load("AovelSansRounded-rdDL.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                }).insert(JoinButtonMarker);
        }).id();

        commands.insert_resource(MainMenuData {
            join_button,
            host_button,
            bg_image,
        });
}

pub fn clear_main_menu(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.host_button).despawn_recursive();
    commands.entity(menu_data.join_button).despawn_recursive();
    commands.entity(menu_data.bg_image).despawn_recursive();
}

pub fn main_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            (Option<&JoinButtonMarker>, Option<&HostButtonMarker>),
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    for (interaction,(join,host), mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if host.is_some() {
                    next_state.set(GameStates::HostMenu);
                }
                else if join.is_some() {
                    next_state.set(GameStates::JoinMenu);
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn join_menu(mut commands: Commands,asset_server: Res<AssetServer>) {
    let bg_image = commands.spawn(SpriteBundle {
        texture: asset_server.load("menu.png"),
        transform: Transform::from_xyz(0., 0., -100.),
        ..default()
    }).id();
    let play_button = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px((720.*0.75)-(65./2.)),
                left: Val::Px((1280.*0.5)-(150./2.)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Select",
                        TextStyle {
                            font: asset_server.load("AovelSansRounded-rdDL.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                }).insert(PlayButtonMarker);
        }).id();

        commands.insert_resource(JoinMenuData {
            play_button,
            bg_image,
        });
}

pub fn clear_join_menu(mut commands: Commands, menu_data: Res<JoinMenuData>) {
    commands.entity(menu_data.play_button).despawn_recursive();
    commands.entity(menu_data.bg_image).despawn_recursive();
}

pub fn join_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            Option<&PlayButtonMarker>,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    for (interaction,play, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if play.is_some() {
                    next_state.set(GameStates::Game);
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn host_menu(mut commands: Commands,asset_server: Res<AssetServer>) {
    let bg_image = commands.spawn(SpriteBundle {
        texture: asset_server.load("menu.png"),
        transform: Transform::from_xyz(0., 0., -100.),
        ..default()
    }).id();
    let play_button = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px((720.*0.75)-(65./2.)),
                left: Val::Px((1280.*0.5)-(150./2.)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Select",
                        TextStyle {
                            font: asset_server.load("AovelSansRounded-rdDL.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                }).insert(PlayButtonMarker);
        }).id();

        commands.insert_resource(HostMenuData {
            play_button,
            bg_image,
        });
}

pub fn clear_host_menu(mut commands: Commands, menu_data: Res<HostMenuData>) {
    commands.entity(menu_data.play_button).despawn_recursive();
    commands.entity(menu_data.bg_image).despawn_recursive();
}

pub fn host_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            Option<&PlayButtonMarker>,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameStates>>,
) { 
    for (interaction,play, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if play.is_some() {
                    next_state.set(GameStates::Game);
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn select_menu(mut commands: Commands,asset_server: Res<AssetServer>) {
    let bg_image = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.72157, 0.56471, 0.53725),
                custom_size: Some(Vec2::new(1280.,720.)),
                ..default()
            },
            transform: Transform::from_xyz(0.,0.,-101.),
            ..default()
        }).id();
    let play_button = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px((720.*0.75)-(65./2.)),
                left: Val::Px((1280.*0.5)-(150./2.)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("AovelSansRounded-rdDL.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                }).insert(PlayButtonMarker);
        }).id();

        let up_button = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px((720.*0.4)-(65./2.)),
                left: Val::Px((1280.*0.5)-(150./2.)),
                display: Display::Flex,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage::new(asset_server.load("arrow_buttons.png")),
                    // border_color: BorderColor(Color::BLACK),
                    // background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "1",
                        TextStyle {
                            font: asset_server.load("AovelSansRounded-rdDL.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                }).insert(SelectButtonMarker {selection: 0});
        }).id();

        let right_button = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px((720.*0.5)-(65./2.)),
                left: Val::Px((1280.*0.6)-(150./2.)),
                display: Display::Flex,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "2",
                        TextStyle {
                            font: asset_server.load("AovelSansRounded-rdDL.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                }).insert(SelectButtonMarker {selection: 1});
        }).id();

        let down_button = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px((720.*0.6)-(65./2.)),
                left: Val::Px((1280.*0.5)-(150./2.)),
                display: Display::Flex,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "3",
                        TextStyle {
                            font: asset_server.load("AovelSansRounded-rdDL.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                }).insert(SelectButtonMarker {selection: 2});
        }).id();

        let left_button = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px((720.*0.5)-(65./2.)),
                left: Val::Px((1280.*0.4)-(150./2.)),
                display: Display::Flex,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "4",
                        TextStyle {
                            font: asset_server.load("AovelSansRounded-rdDL.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                }).insert(SelectButtonMarker {selection: 3});
        }).id();

        commands.insert_resource(SelectMenuData {
            play_button,
            up_button,
            right_button,
            down_button,
            left_button,
            bg_image,
        });
}

pub fn select_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            Option<&PlayButtonMarker>,
            Option<&SelectButtonMarker>,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    for (interaction,play,select, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if play.is_some() {
                    next_state.set(GameStates::Game);
                }
                if select.is_some() {

                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                if select.is_some() {
                    *color = bevy::prelude::BackgroundColor(Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. });
                    border_color.0 = Color::Rgba { red: 0., green: 0., blue: 0., alpha: 1. };
                }
                else {
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }
}

pub fn clear_select_menu(mut commands: Commands, menu_data: Res<SelectMenuData>) {
    commands.entity(menu_data.play_button).despawn_recursive();
    commands.entity(menu_data.up_button).despawn_recursive();
    commands.entity(menu_data.right_button).despawn_recursive();
    commands.entity(menu_data.down_button).despawn_recursive();
    commands.entity(menu_data.left_button).despawn_recursive();
    commands.entity(menu_data.bg_image).despawn_recursive();
}