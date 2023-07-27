use bevy::{prelude::{
    App,
    Update,
    Component,
    Commands,
    Startup,
    Plugin,
    Res,
    Resource,
    Timer,
    Query,
    Time,
    With, ResMut
}, DefaultPlugins, time::TimerMode};

const GRID_SIZE: u32 = 50;

enum Cell {
    Food,
    Snake { age: String }
}

trait Grid {
    fn x_size(&self) -> u32;
    fn y_size(&self) -> u32;
    fn get_cell(&self) -> Option<Cell>;
}



#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name(String::from("Elaina Proctor"))));
    commands.spawn((Person, Name(String::from("Renzo Hume"))));
    commands.spawn((Person, Name(String::from("Zayna Nieves"))));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin)) // Default plugins mean things like ui, window manager, visual renderer etc
        .run();
}
