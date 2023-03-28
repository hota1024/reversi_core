use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone)]
pub enum Color {
    Empty = 0,
    White = 1,
    Black = 2,
    Wall = 3,
}

#[wasm_bindgen]
pub fn flip_color(color: Color) -> Color {
    match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
        _ => color.clone(),
    }
}

#[wasm_bindgen]
pub fn color_eq(a: Color, b: Color) -> bool {
    match (a, b) {
        (Color::Empty, Color::Empty) => true,
        (Color::White, Color::White) => true,
        (Color::Black, Color::Black) => true,
        (Color::Wall, Color::Wall) => true,
        _ => false,
    }
}
