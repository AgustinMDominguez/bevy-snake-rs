mod game;
mod grid;
mod render;
mod utils;

use std::collections::HashMap;

use crate::{
    render::render_game,
    game::{Game, move_snake}
};

use bevy::{
    prelude::*,
    DefaultPlugins
};

pub type Sze = u32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction { Left, Right, Up, Down }

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

#[derive(Resource)]
struct StepTimer(Timer);

pub struct SnakePlugin;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn input_update(mut game: ResMut<Game>, keyboard_input: Res<Input<KeyCode>>) {
    let dir_map = HashMap::from([
        (KeyCode::Up, Direction::Up),
        (KeyCode::Down, Direction::Down),
        (KeyCode::Right, Direction::Right),
        (KeyCode::Left, Direction::Left)
    ]);
    if keyboard_input.any_just_pressed(dir_map.iter().map(| (&k, _) | k)) {
        for (&key_code, &direction) in dir_map.iter() {
            if keyboard_input.just_pressed(key_code) {
                game.input_direction = direction;
                break;
            }
        }
    }
}

fn game_update(game: ResMut<Game>, time: Res<Time>, mut timer: ResMut<StepTimer>) {
    if timer.0.tick(time.delta()).just_finished() && game.game_did_not_end() {
        move_snake(game);
    }
}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Game::new_game())
            .insert_resource(StepTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_systems(Startup, setup)
            .add_systems(Update, (
                game_update,
                input_update,
                render_game.after(game_update)
            )
        );
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SnakePlugin))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
