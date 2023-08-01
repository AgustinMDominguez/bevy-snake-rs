mod cell;
mod grid;
mod text;
mod utils;
mod input;
mod render;
mod timers;
mod simulation;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy::sprite::MaterialMesh2dBundle;

use crate::{
    timers::StepTimers,
    input::{
        PlayerInput,
        handle_player_input,
        RESTART_GAME_KEY, START_GAME_KEY, PAUSE_GAME_KEY
    },
    render::{render_game, get_background_shape},
    simulation::Sim,
    text::SnakeTexts,
    utils::Direction
};

#[derive(Resource)]
struct GameOverSound(Handle<AudioSource>);

#[derive(Resource)]
struct BiteSound(Handle<AudioSource>);

#[derive(Resource)]
struct WinSound(Handle<AudioSource>);

pub type Sze = u32;

#[derive(Event, Debug)]
pub struct SimulationOver {
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
    Paused,
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
        material: materials.add(ColorMaterial::from(Color::Rgba { red: 0.1, green: 0.7, blue: 0.2, alpha: 0.7 })),
        ..default()
    });
    commands.insert_resource(BiteSound(asset_server.load("audio/bite.ogg")));
    commands.insert_resource(GameOverSound(asset_server.load("audio/gameover.ogg")));
    commands.insert_resource(WinSound(asset_server.load("audio/win.ogg")));
    texts.initialize(commands, asset_server);
}

fn update_start_menu(
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

fn update_pause_menu(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut texts: ResMut<SnakeTexts>,
) {
    if keyboard_input.just_pressed(PAUSE_GAME_KEY.keycode) {
        if matches!(game.state, GameState::SimulationRunning) {
            texts.spawn_paused_text(commands, asset_server);
            game.state = GameState::Paused;
            keyboard_input.clear_just_pressed(PAUSE_GAME_KEY.keycode);
        } else if matches!(game.state, GameState::Paused) {
            texts.despawn_paused_text(commands);
            game.state = GameState::SimulationRunning;
            keyboard_input.clear_just_pressed(PAUSE_GAME_KEY.keycode);
        }
    }
}

fn update_simulation(
    time: Res<Time>,
    menu: Res<Game>,
    score_writer: EventWriter<FoodEaten>,
    game_over_writer: EventWriter<SimulationOver>,
    mut simulation: ResMut<Sim>,
    mut step_timers: ResMut<StepTimers>,
    mut player_input: ResMut<PlayerInput>,
) {
    if matches!(menu.state, GameState::SimulationRunning) {
        let boost_timer_finished = step_timers.boost_timer.tick(time.delta()).just_finished();
        let boost_active = boost_timer_finished && player_input.is_boost_active;
        let tick_timer_finished = step_timers.tick_timer.tick(time.delta()).just_finished();

        if simulation.is_game_running() && (boost_active || tick_timer_finished) {
            simulation.run_next_step(&mut player_input.input_direction, score_writer, game_over_writer);
        }
    }
}

fn update_game_over_menu(
    commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut texts: ResMut<SnakeTexts>,
    mut simulation: ResMut<Sim>,
    mut step_timers: ResMut<StepTimers>,
    mut player_input: ResMut<PlayerInput>,
) {
    let on_game_over_menu = matches!(game.state, GameState::GameOverMenu);
    if on_game_over_menu && keyboard_input.just_pressed(RESTART_GAME_KEY.keycode) {
        texts.despawn_game_over_text(commands);
        simulation.reset_new_game();
        step_timers.reset_tick_speed();
        player_input.input_direction.clear();
        player_input.input_direction.push(Direction::Right);
        game.state = GameState::SimulationRunning;
    }
}

fn update_score(
    texts: Res<SnakeTexts>,
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

fn handle_food_eaten_event(
    bite_sound: Res<BiteSound>,
    mut commands: Commands,
    mut events: EventReader<FoodEaten>,
    mut step_timers: ResMut<StepTimers>,
) {
    if let Some(event) = events.iter().next() {
        if event.pieces_eaten % 5 == 0 {
            step_timers.increase_tick_speed();
        }
        commands.spawn(AudioBundle {
            source: bite_sound.0.clone(),
            settings: PlaybackSettings::DESPAWN
        });
    }
}

fn handle_game_over_event(
    game_over_sound: Res<GameOverSound>,
    win_sound: Res<WinSound>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut menu: ResMut<Game>,
    mut texts: ResMut<SnakeTexts>,
    mut game_over_event: EventReader<SimulationOver>,
) {
    if let Some(event) = game_over_event.iter().next() {
        menu.state = GameState::GameOverMenu;
        let sound_effect = if event.win {
            win_sound.0.clone()
        } else {
            game_over_sound.0.clone()
        };
        commands.spawn(AudioBundle {
            source: sound_effect,
            settings: PlaybackSettings::DESPAWN
        });
        texts.spawn_game_over_text(commands, asset_server, event.win);
    }
}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Sim::new_simulation())
            .insert_resource(Game { state: GameState::StartMenu })
            .insert_resource(PlayerInput::default())
            .insert_resource(StepTimers::default())
            .insert_resource(SnakeTexts::default())
            .add_event::<FoodEaten>()
            .add_event::<SimulationOver>()
            .add_systems(Startup, setup)
            .add_systems(Update, (
                update_simulation,
                update_start_menu,
                update_pause_menu,
                update_game_over_menu,
                update_score,
                handle_player_input,
                handle_food_eaten_event,
                handle_game_over_event,
                render_game.after(update_simulation)
            )
        );
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SnakePlugin))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
