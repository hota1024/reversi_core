use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    board::Board,
    evaluate::evaluate_board,
    tile::{flip_color, Color},
};

#[wasm_bindgen]
pub fn nega_alpha(
    board: Board,
    turn: Color,
    depth: i32,
    passed: bool,
    alpha: i32,
    beta: i32,
) -> i32 {
    if depth == 0 {
        return evaluate_board(&board, turn.clone());
    }

    let mut max_score = std::i32::MIN;
    let mut board_clone = board.clone();
    let mut internal_alpha = alpha;

    for y in 0..board.height {
        for x in 0..board.width {
            if board_clone.is_puttable(x, y, turn.clone()) {
                let mut put_board = board_clone.clone();
                put_board.put(x, y, turn.clone());
                let score = -nega_alpha(
                    put_board,
                    flip_color(turn.clone()),
                    depth - 1,
                    false,
                    -beta,
                    -internal_alpha,
                );

                if score >= beta {
                    return score;
                }

                internal_alpha = std::cmp::max(internal_alpha, score);
                max_score = std::cmp::max(max_score, score);
            }
        }
    }

    if max_score == std::i32::MIN {
        if passed {
            return evaluate_board(&board, turn.clone());
        }

        return -nega_alpha(
            board_clone.clone(),
            flip_color(turn.clone()),
            depth - 1,
            true,
            -beta,
            -alpha,
        );
    }

    max_score
}

#[wasm_bindgen]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
pub fn search_with_nega_alpha(board: Board, turn: Color, depth: i32) -> Coord {
    let mut result = Coord { x: -1, y: -1 };
    let mut board_clone = board.clone();
    let mut alpha = std::i32::MIN + 1;
    let beta = std::i32::MAX;

    for y in 0..board.height {
        for x in 0..board.width {
            if board_clone.is_puttable(x, y, turn.clone()) {
                let mut put_board = board_clone.clone();
                put_board.put(x, y, turn.clone());

                let score = -nega_alpha(
                    put_board,
                    flip_color(turn.clone()),
                    depth - 1,
                    false,
                    -beta,
                    -alpha,
                );

                if alpha < score {
                    alpha = score;
                    result = Coord { x, y };
                }
            }
        }
    }

    result
}
