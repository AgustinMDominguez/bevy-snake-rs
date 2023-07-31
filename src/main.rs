mod game;
mod cell;
mod grid;
mod text;
mod utils;
mod render;

use std::collections::HashMap;
use std::time::Duration;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy::sprite::MaterialMesh2dBundle;

use crate::{
    render::{render_game, get_background_shape},
    game::Game,
    text::SnakeTexts,
    utils::Direction
};

pub type Sze = u32;

#[derive(Resource)]
struct StepTimer(Timer);

#[derive(Resource)]
struct BoostTimer(Timer);

#[derive(Resource)]
struct PlayerInput {
    input_direction: Direction
}

#[derive(Event)]
pub struct FoodEaten {
    pub pieces_eaten: Sze,
    pub new_score: Sze
}

pub struct SnakePlugin;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texts: ResMut<SnakeTexts>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(get_background_shape().into()).into(),
        material: materials.add(ColorMaterial::from(Color::Rgba { red: 0.9, green: 0.9, blue: 0.9, alpha: 1.0 })),
        ..default()
    });
    texts.initialize(commands, asset_server);
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
    mut tick_timer: ResMut<StepTimer>,
    score_writer: EventWriter<FoodEaten>,
    mut boost_timer: ResMut<BoostTimer>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let boost_timer_finished = boost_timer.0.tick(time.delta()).just_finished();
    let boost_active = boost_timer_finished && keyboard_input.pressed(KeyCode::Space);
    let tick_timer_finished = tick_timer.0.tick(time.delta()).just_finished();

    if game.is_game_running() && (boost_active || tick_timer_finished) {
        game.run_next_step(input.input_direction, score_writer);
    }
}

fn score_update(
    texts: ResMut<SnakeTexts>,
    mut events: EventReader<FoodEaten>,
    mut score_text_query: Query<(Entity, &mut Text)>
) {
    if !events.is_empty() {
        if let Ok(mut text) = score_text_query.get_component_mut::<Text>(texts.score) {
            for event in events.iter() {
                text.sections[0].value = event.new_score.to_string();
            }
        }
    }
}

fn speed_update(
    mut events: EventReader<FoodEaten>,
    tick_timer: ResMut<StepTimer>
) {
    if let Some(event) = events.iter().next() {
        if event.pieces_eaten % 5 == 0 {
            increase_speed(tick_timer);
        }
    }
}

fn increase_speed(mut tick_timer: ResMut<StepTimer>) {
    let min_duration = Duration::from_secs_f32(0.1);
    let new_duration = tick_timer.0.duration() - Duration::from_secs_f32(0.06);
    if new_duration > min_duration {
        tick_timer.0.set_duration(new_duration)
    }
}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Game::new_game())
            .insert_resource(StepTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .insert_resource(BoostTimer(Timer::from_seconds(0.08, TimerMode::Repeating)))
            .insert_resource(SnakeTexts::new())
            .add_event::<FoodEaten>()
            .add_systems(Startup, setup)
            .add_systems(Update, (
                game_update,
                input_update,
                score_update,
                speed_update,
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
