use std::collections::HashMap;
use bevy::prelude::{Resource, KeyCode, ResMut, Res, Input};

use crate::utils::Direction;

pub const BOOST_GAME_KEY: CommandKey = CommandKey { keycode: KeyCode::Space, str: "Space" };
pub const START_GAME_KEY: CommandKey = CommandKey { keycode: KeyCode::M, str: "M" };
pub const RESTART_GAME_KEY: CommandKey = CommandKey { keycode: KeyCode::R, str: "R" };
pub const PAUSE_GAME_KEY: CommandKey = CommandKey { keycode: KeyCode::P, str: "P" };

pub struct CommandKey {
    pub keycode: KeyCode,
    pub str: &'static str
}

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub input_direction: DirectionQueue,
    pub is_boost_active: bool,
}

#[derive(Default)]
pub struct DirectionQueue {
    next: Option<Direction>,
    queued_1: Option<Direction>,
    queued_2: Option<Direction>
}

impl DirectionQueue {
    pub fn push(&mut self, dir: Direction) {
        if self.next.is_none() {
            self.next = Some(dir);
        } else if self.queued_1.is_none() {
            if let Some(prev_dir) = self.next {
                if prev_dir != dir {
                    self.queued_1 = Some(dir);
                }
            }
        } else if self.queued_2.is_none() {
            if let Some(prev_dir) = self.queued_1 {
                if prev_dir != dir {
                    self.queued_2 = Some(dir);
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<Direction> {
        if let Some(dir) = self.next {
            self.next = self.queued_1;
            self.queued_1 = self.queued_2;
            self.queued_2 = None;
            Some(dir)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.next = None;
        self.queued_1 = None;
        self.queued_2 = None;
    }
}

pub fn handle_player_input(mut input: ResMut<PlayerInput>, keyboard_input: Res<Input<KeyCode>>) {
    let dir_map = HashMap::from([
        (KeyCode::Up, Direction::Up),
        (KeyCode::Down, Direction::Down),
        (KeyCode::Right, Direction::Right),
        (KeyCode::Left, Direction::Left)
    ]);
    if keyboard_input.any_just_pressed(dir_map.iter().map(| (&k, _) | k)) {
        for (&key_code, &direction) in dir_map.iter() {
            if keyboard_input.just_pressed(key_code) {
                input.input_direction.push(direction);
                break;
            }
        }
    }
    input.is_boost_active = keyboard_input.pressed(BOOST_GAME_KEY.keycode);
}