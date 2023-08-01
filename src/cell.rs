use bevy::prelude::Color;

use crate::Sze;
use crate::grid::GRID_SIZE;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub position: CellPos,
    pub content: CellContent
}

#[derive(Clone, Copy, Debug)]
pub enum CellContent {
    Food,
    SnakeBody { age: Sze }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CellPos {
    pub x: usize,
    pub y: usize
}

impl CellPos {
    pub fn get_neighbors(&self) -> Vec<CellPos> {
        let mut neighbors = vec!();
        if self.x > 0             { neighbors.push(CellPos { x: self.x - 1, y: self.y }) };
        if self.x < GRID_SIZE - 1 { neighbors.push(CellPos { x: self.x + 1, y: self.y }) };
        if self.y > 0             { neighbors.push(CellPos { x: self.x, y: self.y - 1 }) };
        if self.y < GRID_SIZE - 1 { neighbors.push(CellPos { x: self.x, y: self.y + 1 }) };
        neighbors
    }
}

impl CellContent {
    pub fn get_color(&self) -> Color {
        match self {
            Self::Food => Color::BLUE,
            Self::SnakeBody { .. } => Color::BLACK
        }
    }
}
