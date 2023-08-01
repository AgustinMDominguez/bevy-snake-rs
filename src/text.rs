use bevy::{
    sprite::Anchor,
    ecs::entity::Entity,
    prelude::{Commands, Resource, AssetServer, Res, Color, Transform, Font, Handle},
    text::{Text2dBundle, Text, TextAlignment, BreakLineOn, TextSection, TextStyle}
};

use crate::{render::get_score_transform, input::PAUSE_GAME_KEY};
use crate::input::{RESTART_GAME_KEY, START_GAME_KEY, BOOST_GAME_KEY};
use crate::simulation::START_SNAKE_LENGHT;

#[derive(Resource)]
pub struct SnakeTexts {
    pub score: Entity,
    pub game_over: Entity,
    pub start: Entity,
    pub pause: Entity,
}

impl SnakeTexts {
    pub fn new() -> Self {
        SnakeTexts {
            score: Entity::PLACEHOLDER,
            game_over: Entity::PLACEHOLDER,
            start: Entity::PLACEHOLDER,
            pause: Entity::PLACEHOLDER,
        }
    }

    pub fn initialize(&mut self, mut commands: Commands, asset_server: Res<AssetServer>) {
        let font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
        self.score = commands.spawn(Text2dBundle {
            text: Text {
                sections: vec!(TextSection {
                    value: (START_SNAKE_LENGHT * 100).to_string(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                }),
                alignment: TextAlignment::Right,
                linebreak_behavior: BreakLineOn::AnyCharacter,
            },
            transform: get_score_transform(),
            text_anchor: Anchor::BottomRight,
            ..Default::default()
        }).id();

        self.game_over = commands.spawn(Text2dBundle {
            text: Text {
                sections: vec!(TextSection {
                    value: format!(
                        "\nMove with arrows keys\nPress {} for boost\nPress {} to start\nPress {} to pause",
                        BOOST_GAME_KEY.text,
                        START_GAME_KEY.text,
                        PAUSE_GAME_KEY.text
                    ),
                    style: TextStyle {
                        font,
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                }),
                alignment: TextAlignment::Center,
                linebreak_behavior: BreakLineOn::WordBoundary,
            },
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            text_anchor: Anchor::Center,
            ..Default::default()
        }).id()
    }

    pub fn despawn_start_menu(&mut self, mut commands: Commands) {
        if self.game_over != Entity::PLACEHOLDER {
            commands.entity(self.game_over).despawn();
        }
    }

    pub fn spawn_paused_text(&mut self, mut commands: Commands, asset_server: Res<AssetServer>) {
        let font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
        self.pause = commands.spawn(Text2dBundle {
            text: Text {
                sections: vec!(
                    TextSection {
                        value: "Paused\n".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 50.0,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: format!("Press {} to unpause", PAUSE_GAME_KEY.text),
                        style: TextStyle {
                            font,
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    }
                ),
                alignment: TextAlignment::Center,
                linebreak_behavior: BreakLineOn::WordBoundary,
            },
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            text_anchor: Anchor::Center,
            ..Default::default()
        }).id()
    }

    pub fn despawn_paused_text(&mut self, mut commands: Commands) {
        if self.game_over != Entity::PLACEHOLDER {
            commands.entity(self.pause).despawn();
        }
    }

    pub fn spawn_game_over_text(&mut self, mut commands: Commands, asset_server: Res<AssetServer>, is_win: bool) {
        let font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
        self.game_over = commands.spawn(Text2dBundle {
            text: Text {
                sections: vec!(
                    TextSection {
                        value: "GAME OVER\n".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 50.0,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: (if is_win {"WIN"} else {"LOSE"}).to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: format!("\nPress {} to restart\nPress ESC to exit", RESTART_GAME_KEY.text),
                        style: TextStyle {
                            font,
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    }
                ),
                alignment: TextAlignment::Center,
                linebreak_behavior: BreakLineOn::WordBoundary,
            },
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            text_anchor: Anchor::Center,
            ..Default::default()
        }).id();
    }

    pub fn despawn_game_over_text(&mut self, mut commands: Commands) {
        if self.game_over != Entity::PLACEHOLDER {
            commands.entity(self.game_over).despawn();
        }
    }
}
