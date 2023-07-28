mod game;
mod cell;
mod grid;
mod utils;
mod render;

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy::sprite::MaterialMesh2dBundle;

use crate::render::{render_game, get_background_shape};
use crate::game::Game;

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

#[derive(Resource)]
struct PlayerInput {
    input_direction: Direction
}

pub struct SnakePlugin;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(get_background_shape().into()).into(),
        material: materials.add(ColorMaterial::from(Color::Rgba { red: 0.9, green: 0.9, blue: 0.9, alpha: 1.0 })),
        ..default()
    });
}

fn input_update(mut input: ResMut<PlayerInput>, keyboard_input: Res<Input<KeyCode>>) {
    let dir_map = HashMap::from([
        (KeyCode::Up, Direction::Up),
        (KeyCode::Down, Direction::Down),
        (KeyCode::Right, Direction::Right),
        (KeyCode::Left, Direction::Left)
    ]);
    if keyboard_input.any_just_pressed(dir_map.iter().map(| (&k, _) | k)) {
        for (&key_code, &direction) in dir_map.iter() {
            if keyboard_input.just_pressed(key_code) {
                input.input_direction = direction;
                break;
            }
        }
    }
}

fn game_update(
    mut game: ResMut<Game>,
    input: ResMut<PlayerInput>,
    time: Res<Time>,
    mut timer: ResMut<StepTimer>
) {
    if timer.0.tick(time.delta()).just_finished() && game.is_game_running() {
        game.run_next_step(input.input_direction)
    }
}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Game::new_game())
            .insert_resource(StepTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
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
        .insert_resource(PlayerInput { input_direction: Direction::Right })
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
