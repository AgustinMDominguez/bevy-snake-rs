use bevy::prelude::*;

use crate::{
    Game,
    grid::GRID_SIZE,
    cell::CellPos
};

const BLOCK_SIZE_PX: f32 = 16.0;
const BORDER_OFFSET: f32 = (BLOCK_SIZE_PX / 2.0) - (BLOCK_SIZE_PX * (GRID_SIZE as f32 / 2.0));
const HEAD_COLOR: Color = Color::Rgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0 };
const BLOCK_SIZE: Vec2 = Vec2 { x: BLOCK_SIZE_PX, y: BLOCK_SIZE_PX };
const BOARD_POS: Vec2 = Vec2 { x: 0.0, y: 0.0 };
const BOARD_SIZE: Vec2 = Vec2 {
    x: -BLOCK_SIZE_PX * (GRID_SIZE as f32),
    y: -BLOCK_SIZE_PX * (GRID_SIZE as f32)
};

const SCORE_BOTTOM_MARGIN: f32 = 6.0;

pub fn get_score_transform() -> Transform {
    Transform::from_translation(Vec3 {
        x: -BORDER_OFFSET,
        y: SCORE_BOTTOM_MARGIN - BORDER_OFFSET,
        z: 1.0
    })
}

pub fn get_background_shape() -> shape::Box {
    shape::Box::new(BOARD_SIZE.x, BOARD_SIZE.y, 1.0)
}

pub fn render_game(game: ResMut<Game>, mut gizmos: Gizmos) {
    game.get_occupied_cells().iter().for_each(| cell | {
        render_cell(cell.pos, cell.content.get_color(), &mut gizmos);
    });
    render_cell(game.get_head_position(), HEAD_COLOR, &mut gizmos);
    gizmos.rect_2d(BOARD_POS, 0.0, BOARD_SIZE, Color::BLACK);
}

fn render_cell(cell_pos: CellPos, color: Color, gizmos: &mut Gizmos) {
    let rect_pos = translate_grid_pos_to_screen(cell_pos);
    gizmos.rect_2d(rect_pos, 0.0, BLOCK_SIZE, color);
}

fn translate_grid_pos_to_screen(cell: CellPos) -> Vec2 {
    let x: f32 = BLOCK_SIZE_PX * (cell.x as f32) + BORDER_OFFSET;
    let y: f32 = BLOCK_SIZE_PX * (cell.y as f32) + BORDER_OFFSET;
    Vec2 { x, y }
}
