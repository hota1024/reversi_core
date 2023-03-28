use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console;

use crate::tile::{color_eq, flip_color, Color};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Board {
    pub width: i32,
    pub height: i32,
    tiles: Vec<Color>,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen]
    pub fn new_10x10() -> Self {
        let width = 10;
        let height = 10;

        let mut board = Self {
            width,
            height,
            tiles: vec![Color::Empty; (width * height) as usize],
        };

        for y in 0..board.height {
            for x in 0..board.width {
                if board.is_edge(x, y) {
                    board.set(x, y, Color::Wall)
                }
            }
        }

        board.set(4, 4, Color::White);
        board.set(5, 5, Color::White);
        board.set(4, 5, Color::Black);
        board.set(5, 4, Color::Black);

        board
    }

    pub fn set(&mut self, x: i32, y: i32, tile: Color) {
        let index = self.get_index(x, y);
        self.tiles[index as usize] = tile;
    }

    pub fn get(&self, x: i32, y: i32) -> Color {
        let index = self.get_index(x, y);

        self.tiles[index as usize].clone()
    }

    pub fn put(&mut self, x: i32, y: i32, color: Color) -> bool {
        let result = self.run(x, y, color.clone(), true);

        if result {
            self.set(x, y, color.clone());
        }

        result
    }

    pub fn is_puttable(&mut self, x: i32, y: i32, color: Color) -> bool {
        self.run(x, y, color, false)
    }

    pub fn has_puttable(&mut self, color: Color) -> bool {
        let mut result = false;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_puttable(x, y, color.clone()) {
                    result = true;
                }
            }
        }

        result
    }

    pub fn is_end(&mut self) -> bool {
        !self.has_puttable(Color::White) && !self.has_puttable(Color::Black)
    }

    pub fn clone_for_search(&self) -> Board {
        self.clone()
    }

    pub fn count_color(&self, color: Color) -> i32 {
        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if color_eq(self.get(x, y), color.clone()) {
                    count += 1;
                }
            }
        }

        count
    }

    fn run(&mut self, x: i32, y: i32, color: Color, flip: bool) -> bool {
        let results = vec![
            self.run_angle(x, y, -1, -1, color.clone(), flip),
            self.run_angle(x, y, 0, -1, color.clone(), flip),
            self.run_angle(x, y, 1, -1, color.clone(), flip),
            self.run_angle(x, y, 1, 0, color.clone(), flip),
            self.run_angle(x, y, 1, 1, color.clone(), flip),
            self.run_angle(x, y, 0, 1, color.clone(), flip),
            self.run_angle(x, y, -1, 1, color.clone(), flip),
            self.run_angle(x, y, -1, 0, color.clone(), flip),
        ];

        results.contains(&true)
    }

    fn run_angle(&mut self, x: i32, y: i32, ax: i32, ay: i32, color: Color, flip: bool) -> bool {
        if flip {
            if !self.run_angle(x, y, ax, ay, color.clone(), false) {
                return false;
            }
        }

        if !color_eq(self.get(x, y), Color::Empty) {
            return false;
        }

        let flipped_color = flip_color(color.clone());
        let mut cx = x as i32 + ax;
        let mut cy = y as i32 + ay;

        if !color_eq(self.get(cx, cy), flipped_color.clone()) {
            return false;
        }

        while color_eq(self.get(cx, cy), flipped_color.clone()) {
            if flip {
                self.set(cx, cy, color.clone())
            }

            cx += ax;
            cy += ay;
        }

        color_eq(self.get(cx, cy), color.clone())
    }

    fn get_index(&self, x: i32, y: i32) -> i32 {
        x + y * self.width
    }

    fn is_edge(&self, x: i32, y: i32) -> bool {
        x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1
    }
}
