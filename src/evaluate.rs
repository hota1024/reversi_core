use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    board::Board,
    tile::{color_eq, flip_color, Color},
};

#[cfg_attr(rustfmt, rustfmt_skip)]
const WEIGHTS: [i32; 100] = [
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0, 30,-12,  0, -1, -1,  0,-12, 30,  0,
    0,-12,-15, -3, -3, -3, -3,-15,-12,  0,
    0,  0, -3,  0, -1, -1, 0,  -3,  0,  0,
    0, -1, -3, -1, -1, -1, -1, -1, -3,  0,
    0, -1, -3, -1, -1, -1, -1, -1, -3,  0,
    0,  0, -3,  0, -1, -1, 0,  -3,  0,  0,
    0,-12,-15, -3, -3, -3, -3,-15,-12,  0,
    0, 30,-12,  0, -1, -1,  0,-12, 30,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
];

#[wasm_bindgen]
pub fn evaluate_board(board: &Board, turn: Color) -> i32 {
    let mut score = 0;
    let flipped_color = flip_color(turn.clone());

    for y in 0..10 {
        for x in 0..10 {
            let tile = board.get(x, y);
            let weight = WEIGHTS[(x + y * 10) as usize];

            if color_eq(tile.clone(), turn.clone()) {
                score += weight;
            } else if color_eq(tile.clone(), flipped_color.clone()) {
                score -= weight;
            }
        }
    }

    score
}
