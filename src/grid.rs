use bevy::prelude::Component;

use crate::cell::{Cell, CellPos, CellContent};

pub const GRID_SIZE: usize = 15;

#[derive(Component)]
pub struct Grid {
    arr: [[Option<CellContent>; GRID_SIZE]; GRID_SIZE]
}

impl Grid {
    pub fn new_empty_grid() -> Self { Grid { arr: [[None; GRID_SIZE]; GRID_SIZE] }}

    pub fn get_cell_content(&self, pos: CellPos) -> Option<CellContent> { self.arr[pos.x][pos.y] }

    pub fn is_cell_empty(&self, pos: CellPos) -> bool { self.arr[pos.x][pos.y].is_none() }

    pub fn set_cell(&mut self, cell: Cell) {
        self.arr[cell.position.x][cell.position.y] = Some(cell.content);
    }

    pub fn clear_cell(&mut self, pos: CellPos) {
        self.arr[pos.x][pos.y] = None;
    }

    pub fn clear_grid(&mut self) {
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                self.arr[x][y] = None;
            }
        }
    }

    pub fn get_occupied_cells(&self) -> Vec<Cell> {
        let mut ret_cells = vec!();
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                if let Some(content) = self.arr[x][y] {
                    ret_cells.push(Cell { position: CellPos { x, y }, content });
                }
            }
        }
        ret_cells
    }
}
