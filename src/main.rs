mod grid;
mod render;
mod utils;

use std::collections::HashMap;
use rand::{rngs::ThreadRng, Rng, seq::SliceRandom};

use crate::{
    utils::min,
    render::render_game,
    grid::{
        Cell,
        CellPos,
        Grid,
        ArrGrid,
        GRID_SIZE
    }
};

use bevy::{
    prelude::*,
    DefaultPlugins
};

pub type Sze = u32;

const START_SNAKE_LENGHT: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction { Left, Right, Up, Down }

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
pub struct Game {
    grid: ArrGrid,
    head_pos: CellPos,
    tail_pos: CellPos,
    food_pos: CellPos,
    score: Sze,
    input: Direction,
    neck_direction: Direction,
    game_ended: Option<GameConclution>
}

#[derive(Resource)]
struct StepTimer(Timer);

pub struct SnakePlugin;

#[derive(Debug)]
enum GameConclution { Win, Loss }

fn get_random_head_pos(rng: &mut ThreadRng) -> CellPos {
    let x = rng.gen_range(START_SNAKE_LENGHT..(GRID_SIZE / 2));
    let y = rng.gen_range(0..GRID_SIZE);
    CellPos { x, y }
}

impl Game {
    fn new_game() -> Game {
        let mut rng = rand::thread_rng();
        let head_pos = get_random_head_pos(&mut rng);
        let tail_pos = CellPos {
            x: head_pos.x + 1 - START_SNAKE_LENGHT,
            y: head_pos.y
        };
        let mut grid = ArrGrid::new_empty_grid();
        for offset in 0..START_SNAKE_LENGHT {
            let body = Cell::SnakeBody { age: (START_SNAKE_LENGHT - offset) as Sze };
            grid.set_cell(body, tail_pos.x + offset, tail_pos.y);
        }
        let food_pos = CellPos { x: (GRID_SIZE - head_pos.x)/2, y: rng.gen_range(0..GRID_SIZE) };
        grid.set_cell(Cell::Food, food_pos.x, food_pos.y);
        let game_ended = None;
        Game {
            grid,
            head_pos,
            tail_pos,
            food_pos,
            score: START_SNAKE_LENGHT as Sze,
            input: Direction::Right,
            neck_direction: Direction::Right,
            game_ended
        }
    }

    fn get_state(&self) -> String {
        let mut ret = String::new();
        ret.push_str(format!("(head={:?})",self.head_pos).as_str());
        ret.push_str(format!("(tail={:?})",self.tail_pos).as_str());
        ret.push_str(format!("(neck_dir={:?})",self.neck_direction).as_str());
        ret.push_str(format!("(input_dir={:?})",self.input).as_str());
        ret.push_str(format!("(game_ended={:?})",self.game_ended).as_str());
        let occupied_cells = self.grid.get_occupied_cells().iter().fold(String::new(), | str, &val | {
            if let Some(Cell::SnakeBody { age }) = self.grid.get_cell(val.x, val.y) {
                let mut new_str = str.clone();
                new_str.push_str(format!("({})", age).as_str());
                new_str
            } else {
                str
            }
        });
        ret.push_str(format!("(occupied_cells = {:?}", occupied_cells).as_str());
        ret
    }
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
                game.input = direction;
                break;
            }
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn game_update(mut game: ResMut<Game>, time: Res<Time>, mut timer: ResMut<StepTimer>) {
    if timer.0.tick(time.delta()).just_finished() && game.game_ended.is_none() {
        println!("Game = {}", game.get_state());
        move_snake_head(&mut game);
        if was_food_eaten(&game) {
            let could_spawn_food = spawn_food(&mut game);
            game.score += 1;
            if !could_spawn_food {
                game.game_ended = Some(GameConclution::Win);
            }
        }
        if let Some(game_ended) = &game.game_ended {
            println!("{:?}!", game_ended);
        }
        move_snake_tail(&mut game);
    }
}

fn was_food_eaten(game: &Game) -> bool { game.food_pos == game.head_pos }

fn move_snake_head(game: &mut Game) {
    game.grid.get_occupied_cells().iter().for_each(| cell_pos | {
        match game.grid.get_cell(cell_pos.x, cell_pos.y) {
            Some(Cell::SnakeBody { age }) => {
                game.grid.set_cell(Cell::SnakeBody { age: age + 1 }, cell_pos.x, cell_pos.y)
            },
            _ => {}
        }
    });
    let move_dir = if game.input == game.neck_direction.opposite() {
        game.neck_direction
    } else {
        game.input
    };
    let dir_vector = match move_dir {
        Direction::Up => [0, 1],
        Direction::Down => [0, -1],
        Direction::Left => [-1, 0],
        Direction::Right => [1, 0]
    };
    let ix = (game.head_pos.x as i64) + dir_vector[0];
    let iy = (game.head_pos.y as i64) + dir_vector[1];
    [ix, iy].iter().for_each(| &coord | {
        if coord < 0 || coord >= (GRID_SIZE as i64) {
            game.game_ended = Some(GameConclution::Loss);
            return;
        }
    });
    let x = ix as usize;
    let y = iy as usize;
    if let Some(Cell::SnakeBody { .. }) = game.grid.get_cell(x, y) {
        game.game_ended = Some(GameConclution::Loss);
        return;
    }
    game.grid.set_cell(Cell::SnakeBody { age: 1 }, x, y);
    game.neck_direction = move_dir;
    game.head_pos = CellPos { x, y};
}

fn move_snake_tail(game: &mut Game) {
    if let Some(Cell::SnakeBody { age }) = game.grid.get_cell(game.tail_pos.x, game.tail_pos.y) {
        if game.score < age {
            if let Some((new_tail_pos, _)) = get_oldest_tail_neighbor(game) {
                game.grid.clear_cell(game.tail_pos.x, game.tail_pos.y);
                game.tail_pos = new_tail_pos;
            } else {
                panic!() // :/
            }
        }
    } else {
        panic!() // Not ideal :/ maybe I did something wrong but idk what
    }
}

fn get_oldest_tail_neighbor(game: &Game) -> Option<(CellPos, Sze)> {
    game.tail_pos
        .get_valid_neighbors()
        .iter()
        .fold(None, | prev, &neighbor | {
            if let Some(Cell::SnakeBody { age }) = game.grid.get_cell(neighbor.x, neighbor.y) {
                match prev {
                    Some((_, prev_age)) => {
                        if prev_age < age {
                            Some((CellPos { x: neighbor.x, y: neighbor.y}, age))
                        } else {
                            prev
                        }
                    },
                    None => Some((CellPos { x: neighbor.x, y: neighbor.y}, age))
                }
            } else {
                prev
            }
        }
    )
}

fn spawn_food(game: &mut Game) -> bool {
    let mut rng = rand::thread_rng();
    let mut is_food_spawned = false;
    for _ in 1..20 {
        let x = rng.gen_range(0..GRID_SIZE);
        let y = rng.gen_range(0..GRID_SIZE);
        if let None = game.grid.get_cell(x, y) {
            game.grid.set_cell(Cell::Food, x, y);
            is_food_spawned = true;
            break;
        }
    }
    if !is_food_spawned {
        let empty_cells = get_empty_cells_around_tail(&game.grid, &game.tail_pos);
        if let Some(cell_pos) = empty_cells.choose(&mut rng) {
            game.grid.set_cell(Cell::Food, cell_pos.x, cell_pos.y);
            is_food_spawned = true;
        };
    }
    is_food_spawned
}

const AV_SPACE_SEARCH_AREA: usize = 10;

fn get_empty_cells_around_tail(grid: &ArrGrid, tail_pos: &CellPos) -> Vec<CellPos> {
    let mut empty_cells = vec!();
    let low_x = tail_pos.x - min(tail_pos.x, AV_SPACE_SEARCH_AREA);
    let high_x = min(GRID_SIZE, tail_pos.x + AV_SPACE_SEARCH_AREA);
    let low_y = tail_pos.y - min(tail_pos.y, AV_SPACE_SEARCH_AREA);
    let high_y = min(GRID_SIZE, tail_pos.y + AV_SPACE_SEARCH_AREA);
    for x in low_x..high_x {
        for y in low_y..high_y {
            if let None = grid.get_cell(x, y)  {
                empty_cells.push(CellPos { x, y })
            }
        }
    }
    empty_cells
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
