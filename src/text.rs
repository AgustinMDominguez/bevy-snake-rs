use bevy::{
    sprite::Anchor,
    ecs::entity::Entity,
    prelude::{Commands, Resource, AssetServer, Res, Color},
    text::{Text2dBundle, Text, TextAlignment, BreakLineOn, TextSection, TextStyle}
};

use crate::render::get_score_transform;
use crate::game::START_SNAKE_LENGHT;

#[derive(Resource)]
pub struct SnakeTexts {
    pub score: Entity,
    pub game_over: Entity,
    pub start: Entity
}

impl SnakeTexts {
    pub fn new() -> Self {
        SnakeTexts {
            score: Entity::PLACEHOLDER,
            game_over: Entity::PLACEHOLDER,
            start: Entity::PLACEHOLDER
        }
    }

    pub fn initialize(&mut self, mut commands: Commands, asset_server: Res<AssetServer>) {
        self.score = commands.spawn(Text2dBundle {
            text: Text {
                sections: vec!(TextSection {
                    value: (START_SNAKE_LENGHT * 100).to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                }),
                alignment: TextAlignment::Right,
                linebreak_behavior: BreakLineOn::AnyCharacter,
            },
            transform: get_score_transform(),
            text_anchor: Anchor::BottomRight,
            ..Default::default()
        }).id();
    }
}
