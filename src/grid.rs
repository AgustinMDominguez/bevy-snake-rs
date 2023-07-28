use std::fmt::Debug;
use crate::Sze;
use bevy::prelude::{Component, Color};

pub const GRID_SIZE: usize = 50;

#[derive(Debug)]
pub struct OutOfBoundsError;

#[derive(Component)]
pub struct ArrGrid {
    arr: [[Option<Cell>; GRID_SIZE]; GRID_SIZE]
}

pub trait Grid {
    fn x_size(&self) -> usize;
    fn y_size(&self) -> usize;
    fn get_cell(&self, x: usize, y: usize) -> Option<Cell>;
    fn get_occupied_cells(&self) -> Vec<CellPos>;
    fn set_cell(&mut self, value: Cell, x: usize, y: usize);
    fn clear_cell(&mut self, x: usize, y: usize);
}


impl ArrGrid {
    pub fn new_empty_grid() -> ArrGrid {
        ArrGrid { arr: [[None; GRID_SIZE]; GRID_SIZE] }
    }
}

impl Grid for ArrGrid {
    fn x_size(&self) -> usize { GRID_SIZE }

    fn y_size(&self) -> usize { GRID_SIZE }

    fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        self.arr[x][y]
    }

    fn get_occupied_cells(&self) -> Vec<CellPos> {
        let mut ret_cells = vec!();
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                if self.arr[x][y].is_some() {
                    ret_cells.push(CellPos { x, y });
                }
            }
        }
        ret_cells
    }

    fn set_cell(&mut self, value: Cell, x: usize, y: usize) {
        self.arr[x][y] = Some(value);
    }

    fn clear_cell(&mut self, x: usize, y: usize) {
        self.arr[x][y] = None;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Food,
    SnakeBody { age: Sze }
}

impl Cell {
    pub fn get_color(&self) -> Color {
        match self {
            Self::Food => Color::RED,
            Self::SnakeBody { .. } => Color::BLACK
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CellPos {
    pub x: usize,
    pub y: usize
}

impl CellPos {
    pub fn get_valid_neighbors(&self) -> Vec<CellPos> {
        let mut neighbors = vec!();
        if self.x > 0             { neighbors.push(CellPos { x: self.x - 1, y: self.y }) };
        if self.x < GRID_SIZE - 1 { neighbors.push(CellPos { x: self.x + 1, y: self.y }) };
        if self.y > 0             { neighbors.push(CellPos { x: self.x, y: self.y - 1 }) };
        if self.y < GRID_SIZE - 1 { neighbors.push(CellPos { x: self.x, y: self.y + 1 }) };
        neighbors
    }
}
