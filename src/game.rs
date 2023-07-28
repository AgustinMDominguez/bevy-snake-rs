use rand::{rngs::ThreadRng, Rng, seq::SliceRandom};
use bevy::prelude::Resource;

use crate::{
    Sze,
    Direction,
    utils::min,
    grid::{Grid, GRID_SIZE},
    cell::{Cell, CellPos, CellContent}
};

const START_SNAKE_LENGHT: usize = 3;

#[derive(Debug, PartialEq)]
enum GameState { Running, Win, Loss }

#[derive(Resource)]
pub struct Game {
    score: Sze,
    neck_direction: Direction,
    game_state: GameState,
    grid: Grid,
    head_pos: CellPos,
    tail_pos: CellPos,
    food_pos: CellPos
}

struct SnakeBody {
    pos: CellPos,
    age: Sze
}

impl Game {
    pub fn new_game() -> Self {
        let mut rng = rand::thread_rng();
        let head_pos = get_random_head_pos(&mut rng);
        let tail_pos = CellPos {
            x: head_pos.x + 1 - START_SNAKE_LENGHT,
            y: head_pos.y
        };
        let mut grid = Grid::new_empty_grid();
        for offset in 0..START_SNAKE_LENGHT {
            grid.set_cell(Cell {
                pos: CellPos { x: tail_pos.x + offset, y: tail_pos.y },
                content: CellContent::SnakeBody { age: (START_SNAKE_LENGHT - offset) as Sze }
            });
        }
        let food_pos = CellPos {
            x: (GRID_SIZE - head_pos.x) / 2,
            y: rng.gen_range(0..GRID_SIZE)
        };
        grid.set_cell(Cell { pos: food_pos, content: CellContent::Food });
        Game {
            grid,
            head_pos,
            tail_pos,
            food_pos,
            score: START_SNAKE_LENGHT as Sze,
            neck_direction: Direction::Right,
            game_state: GameState::Running
        }
    }

    pub fn run_next_step(&mut self, input_direction: Direction) {
        self.age_snake_body();
        self.move_snake_head(input_direction);
        if !self.is_game_running() {
            println!("{:?}!", self.game_state);
            return;
        }
        if self.was_food_eaten() {
            let could_spawn_food = self.spawn_food();
            self.score += 1;
            if !could_spawn_food {
                self.game_state= GameState::Win;
            }
        }
        self.move_snake_tail();
        self.log_game_state_if_finished();
    }

    fn age_snake_body(&mut self) {
        self.get_occupied_cells().iter().for_each( | &cell | {
            match cell.content {
                CellContent::Food => {},
                CellContent::SnakeBody { age } => {
                    self.grid.set_cell(Cell {
                        pos: cell.pos,
                        content: CellContent::SnakeBody { age: age + 1 }
                    });
                }
            }
        });
    }

    fn move_snake_head(&mut self, input_direction: Direction) {
        let move_dir = if input_direction == self.neck_direction.opposite() {
            self.neck_direction
        } else {
            input_direction
        };
        let dir_vector = match move_dir {
            Direction::Up => [0, 1],
            Direction::Down => [0, -1],
            Direction::Left => [-1, 0],
            Direction::Right => [1, 0]
        };
        let ix = (self.head_pos.x as i64) + dir_vector[0];
        let iy = (self.head_pos.y as i64) + dir_vector[1];
        let grid_size = GRID_SIZE as i64;
        if ix < 0 || iy < 0 || ix >= grid_size || iy >= grid_size {
            self.game_state = GameState::Loss;
            return;
        }

        let x = ix as usize;
        let y = iy as usize;

        let head_pos = CellPos { x, y };
        if self.is_position_occupied_by_snake(head_pos) {
            self.game_state = GameState::Loss;
            return;
        }
        self.grid.set_cell(Cell { pos: head_pos, content: CellContent::SnakeBody { age: 1 } });
        self.neck_direction = move_dir;
        self.head_pos = head_pos;
    }

    fn log_game_state_if_finished(&self) {
        if self.game_state != GameState::Running {
            println!("{:?}!", self.game_state);
        }
    }

    fn was_food_eaten(&self) -> bool { self.food_pos == self.head_pos }

    fn is_position_occupied_by_snake(&self, pos: CellPos) -> bool {
        matches!(self.grid.get_cell_content(pos), Some(CellContent::SnakeBody { .. }))
    }

    fn move_snake_tail(&mut self) {
        let cur_tail = self.get_tail();
        if self.score < cur_tail.age {
            let new_tail = self.get_oldest_tail_neighbor();
            self.grid.clear_cell(cur_tail.pos);
            self.tail_pos = new_tail;
        }
    }

    fn get_oldest_tail_neighbor(&self) -> CellPos {
        let oldest_neighbor: Option<(CellPos, Sze)> = self.tail_pos
            .get_neighbors()
            .iter()
            .filter_map(| &cell_pos | {
                if let Some(CellContent::SnakeBody { age }) = self.grid.get_cell_content(cell_pos) {
                    Some((cell_pos, age))
                } else {
                    None
                }
            })
            .fold(None, | prev, (cur_pos, cur_age) | {
                match prev {
                    Some((_, prev_age)) => {
                        if prev_age < cur_age { Some((cur_pos, cur_age)) } else { prev }
                    },
                    None => Some((cur_pos, cur_age))
                }
            });
        oldest_neighbor.unwrap().0
    }

    fn get_tail(&self) -> SnakeBody {
        if let Some(CellContent::SnakeBody { age }) = self.grid.get_cell_content(self.tail_pos) {
            SnakeBody { pos: self.tail_pos, age }
        } else {
            panic!()
        }
    }

    fn spawn_food(&mut self) -> bool {
        let mut rng = rand::thread_rng();
        let mut is_food_spawned = false;
        for _ in 1..20 {
            let food_pos = CellPos { x: rng.gen_range(0..GRID_SIZE), y: rng.gen_range(0..GRID_SIZE) };
            if self.grid.get_cell_content(food_pos).is_none() {
                self.grid.set_cell(Cell { pos: food_pos, content: CellContent::Food });
                self.food_pos = food_pos;
                is_food_spawned = true;
                break;
            }
        }
        if !is_food_spawned {
            if let Some(&food_pos) = self.get_empty_cells_around_tail().choose(&mut rng) {
                self.grid.set_cell(Cell { pos: food_pos, content: CellContent::Food });
                self.food_pos = food_pos;
                is_food_spawned = true;
            };
        }
        is_food_spawned
    }

    fn get_empty_cells_around_tail(&self) -> Vec<CellPos> {
        let search_area = 10;
        let tail_pos = &self.tail_pos;
        let mut empty_cells = vec!();
        let low_x = tail_pos.x - min(tail_pos.x, search_area);
        let high_x = min(GRID_SIZE, tail_pos.x + search_area);
        let low_y = tail_pos.y - min(tail_pos.y, search_area);
        let high_y = min(GRID_SIZE, tail_pos.y + search_area);
        for x in low_x..high_x {
            for y in low_y..high_y {
                let pos: CellPos = CellPos { x, y };
                if self.grid.is_cell_empty(pos) { empty_cells.push(pos); }
            }
        }
        empty_cells
    }

    pub fn get_occupied_cells(&self) -> Vec<Cell> { self.grid.get_occupied_cells() }

    pub fn get_head_position(&self) -> CellPos { self.head_pos }

    pub fn is_game_running(&self) -> bool { self.game_state == GameState::Running }
}

fn get_random_head_pos(rng: &mut ThreadRng) -> CellPos {
    let x = rng.gen_range(START_SNAKE_LENGHT..(GRID_SIZE / 2));
    let y = rng.gen_range(0..GRID_SIZE);
    CellPos { x, y }
}
