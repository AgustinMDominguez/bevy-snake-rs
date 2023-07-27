use crate::Game;
use crate::grid::{GRID_SIZE, Grid, CellPos};

use bevy::prelude::*;

const BLOCK_SIZE_PX: f32 = 10.0;
const TOP_LEFT_OFFSET: f32 = (BLOCK_SIZE_PX / 2.0) - (BLOCK_SIZE_PX * (GRID_SIZE as f32 / 2.0));

fn translate_grid_pos_to_screen(cell: &CellPos) -> Vec2 {
    let x: f32 = BLOCK_SIZE_PX * (cell.x as f32) + TOP_LEFT_OFFSET;
    let y: f32 = BLOCK_SIZE_PX * (cell.y as f32) + TOP_LEFT_OFFSET;
    Vec2 { x, y }
}

pub fn render_game(game: ResMut<Game>, mut gizmos: Gizmos) {
    gizmos.rect_2d(
        Vec2 { x: 0.0, y: 0.0 },
        0.0,
        Vec2 { x: -BLOCK_SIZE_PX * (GRID_SIZE as f32), y: -BLOCK_SIZE_PX * (GRID_SIZE as f32) },
        Color::WHITE
    );
    game.grid.get_occupied_cells().iter().for_each(| cell_pos | {
        if let Some(cell) = game.grid.get_cell(cell_pos.x, cell_pos.y) {
            render_block(cell_pos, cell.get_color(), &mut gizmos)
        }
    });
    render_block(
        &game.head_pos,
        Color::Rgba { red: 0.3, green: 0.3, blue: 0.3, alpha: 1.0 },
        &mut gizmos
    );
}

fn render_block(cell_pos: &CellPos, color: Color, gizmos: &mut Gizmos) {
    gizmos.rect_2d(
        translate_grid_pos_to_screen(cell_pos),
        0.0,
        Vec2 { x: BLOCK_SIZE_PX, y: BLOCK_SIZE_PX },
        color
    );
}
