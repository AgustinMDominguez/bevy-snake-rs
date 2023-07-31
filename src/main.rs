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
    game::Sim,
    text::SnakeTexts,
    utils::Direction
};

struct CommandKey {
    keycode: KeyCode,
    text: &'static str
}

const BOOST_GAME_KEY: CommandKey = CommandKey { keycode: KeyCode::Space, text: "Space" };
const START_GAME_KEY: CommandKey = CommandKey { keycode: KeyCode::M, text: "M" };
const RESTART_KEY: CommandKey = CommandKey { keycode: KeyCode::R, text: "R" };

#[derive(Resource)]
struct GameOverSound(Handle<AudioSource>);

#[derive(Resource)]
struct BiteSound(Handle<AudioSource>);

pub type Sze = u32;

#[derive(Resource)]
struct StepTimers {
    boost_timer: Timer,
    tick_timer: Timer
}


#[derive(Resource)]
struct PlayerInput {
    input_direction: Direction,
    boost_active: bool
}

#[derive(Event, Debug)]
pub struct GameOver{
    win: bool
}

#[derive(Event)]
pub struct FoodEaten {
    pub pieces_eaten: Sze,
    pub new_score: Sze
}

pub struct SnakePlugin;

#[derive(Resource)]
struct Game {
    pub state: GameState
}

pub enum GameState {
    StartMenu,
    SimulationRunning,
    GameOverMenu,
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texts: ResMut<SnakeTexts>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(get_background_shape().into()).into(),
        material: materials.add(ColorMaterial::from(Color::Rgba { red: 0.9, green: 0.9, blue: 0.9, alpha: 1.0 })),
        ..default()
    });
    commands.insert_resource(GameOverSound(asset_server.load("audio/gameover.ogg")));
    commands.insert_resource(BiteSound(asset_server.load("audio/bite.ogg")));
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
    input.boost_active = keyboard_input.pressed(BOOST_GAME_KEY.keycode);
}

fn start_menu_update(
    commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut texts: ResMut<SnakeTexts>,
) {
    let on_start_menu = matches!(game.state, GameState::StartMenu);
    if on_start_menu && keyboard_input.just_pressed(START_GAME_KEY.keycode) {
        texts.despawn_start_menu(commands);
        game.state = GameState::SimulationRunning;
    }
}

fn game_running_update(
    time: Res<Time>,
    menu: ResMut<Game>,
    mut simulation: ResMut<Sim>,
    mut step_timers: ResMut<StepTimers>,
    player_input: ResMut<PlayerInput>,
    score_writer: EventWriter<FoodEaten>,
    game_over_writer: EventWriter<GameOver>,
) {
    if matches!(menu.state, GameState::SimulationRunning) {
        let boost_timer_finished = step_timers.boost_timer.tick(time.delta()).just_finished();
        let boost_active = boost_timer_finished && player_input.boost_active;
        let tick_timer_finished = step_timers.tick_timer.tick(time.delta()).just_finished();

        if simulation.is_game_running() && (boost_active || tick_timer_finished) {
            simulation.run_next_step(player_input.input_direction, score_writer, game_over_writer);
        }
    }
}

fn game_over_update(
    commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut simulation: ResMut<Sim>,
    mut texts: ResMut<SnakeTexts>,
) {
    let on_game_over_menu = matches!(game.state, GameState::GameOverMenu);
    if on_game_over_menu && keyboard_input.just_pressed(RESTART_KEY.keycode) {
        texts.despawn_game_over_text(commands);
        simulation.set_random_initial_state();
        game.state = GameState::SimulationRunning;
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

fn handle_bite_sound_event(
    mut commands: Commands,
    bite_sound: Res<BiteSound>,
    mut events: EventReader<FoodEaten>,
) {
    if events.iter().next().is_some() {
        commands.spawn(AudioBundle {
            source: bite_sound.0.clone(),
            settings: PlaybackSettings::DESPAWN
        });
    }
}

fn speed_update(
    mut events: EventReader<FoodEaten>,
    mut step_timers: ResMut<StepTimers>,
) {
    if let Some(event) = events.iter().next() {
        if event.pieces_eaten % 5 == 0 {
            increase_speed(&mut step_timers.tick_timer);
        }
    }
}

fn increase_speed(tick_timer: &mut Timer) {
    let min_duration = Duration::from_secs_f32(0.1);
    let new_duration = tick_timer.duration() - Duration::from_secs_f32(0.06);
    if new_duration > min_duration {
        tick_timer.set_duration(new_duration)
    }
}

fn handle_game_over_event(
    mut menu: ResMut<Game>,
    mut commands: Commands,
    mut texts: ResMut<SnakeTexts>,
    game_over_sound: Res<GameOverSound>,
    mut game_over_event: EventReader<GameOver>,
    asset_server: Res<AssetServer>
) {
    if let Some(event) = game_over_event.iter().next() {
        menu.state = GameState::GameOverMenu;
        commands.spawn(AudioBundle {
            source: game_over_sound.0.clone(),
            settings: PlaybackSettings::DESPAWN
        });
        texts.spawn_game_over_text(commands, asset_server, event.win);
    }
}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Sim::new_game())
            .insert_resource(Game { state: GameState::StartMenu })
            .insert_resource(StepTimers{
                boost_timer: Timer::from_seconds(0.08, TimerMode::Repeating),
                tick_timer: Timer::from_seconds(0.5, TimerMode::Repeating)
            })
            .insert_resource(SnakeTexts::new())
            .add_event::<FoodEaten>()
            .add_event::<GameOver>()
            .add_systems(Startup, setup)
            .add_systems(Update, (
                start_menu_update,
                game_running_update,
                game_over_update,
                input_update,
                score_update,
                speed_update,
                handle_bite_sound_event,
                handle_game_over_event,
                render_game.after(game_running_update)
            )
        );
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SnakePlugin))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .insert_resource(PlayerInput { input_direction: Direction::Right, boost_active: false })
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
