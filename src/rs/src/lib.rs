extern crate rand;
use ndarray::{Array2, ArrayBase};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

pub mod game;
pub mod snake;
mod util;

#[cfg(feature = "pyo3")]
mod py;

use self::game::Game;
use self::snake::Direction;
use std::sync::Mutex;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new(100, 100));
}

#[wasm_bindgen]
pub fn is_ended() -> bool {
    GAME.lock().unwrap().is_ended()
}

#[wasm_bindgen]
pub fn tick() {
    GAME.lock().unwrap().tick();
}

#[wasm_bindgen]
pub fn reset(w: u16, h: u16) {
    GAME.lock().unwrap().reset(w, h);
}

#[wasm_bindgen]
pub fn snake_len() -> usize {
    GAME.lock().unwrap().get_snake().body().len()
}

#[wasm_bindgen]
pub fn snake_x_at(idx: usize) -> i16 {
    GAME.lock().unwrap().get_snake().body()[idx].0
}

#[wasm_bindgen]
pub fn snake_y_at(idx: usize) -> i16 {
    GAME.lock().unwrap().get_snake().body()[idx].1
}

#[wasm_bindgen]
pub fn snake_up() {
    GAME.lock().unwrap().get_snake().dir(Direction::Top);
}

#[wasm_bindgen]
pub fn snake_left() {
    GAME.lock().unwrap().get_snake().dir(Direction::Left);
}

#[wasm_bindgen]
pub fn snake_right() {
    GAME.lock().unwrap().get_snake().dir(Direction::Right);
}

#[wasm_bindgen]
pub fn snake_turn_left() {
    GAME.lock().unwrap().get_snake().turn_left();
}

#[wasm_bindgen]
pub fn snake_turn_right() {
    GAME.lock().unwrap().get_snake().turn_right();
}

#[wasm_bindgen]
pub fn snake_dir() -> u8 {
    GAME.lock().unwrap().snake_dir()
}

#[wasm_bindgen]
pub fn snake_down() {
    GAME.lock().unwrap().get_snake().dir(Direction::Bottom);
}

pub fn food() -> (i16, i16) {
    GAME.lock().unwrap().get_food().unwrap_or((-1, -1))
}

pub fn head() -> (i16, i16) {
    GAME.lock().unwrap().get_snake().head()
}

#[wasm_bindgen]
pub fn food_x() -> i16 {
    GAME.lock().unwrap().get_food().unwrap_or((-1, -1)).0
}

#[wasm_bindgen]
pub fn food_y() -> i16 {
    GAME.lock().unwrap().get_food().unwrap_or((-1, -1)).1
}

#[wasm_bindgen]
pub fn game_height() -> u16 {
    GAME.lock().unwrap().height()
}

#[wasm_bindgen]
pub fn game_width() -> u16 {
    GAME.lock().unwrap().width()
}

#[wasm_bindgen]
pub fn tick_count() -> usize {
    GAME.lock().unwrap().tick_count()
}

pub fn snake_body() -> Vec<(i16, i16)> {
    GAME.lock().unwrap().get_snake().body().clone()
}

pub fn state() -> Vec<Vec<i8>> {
    let mut game = GAME.lock().unwrap();
    let width = game.width() as usize;
    let height = game.height() as usize;

    let (food_x, food_y) = game.get_food().unwrap();
    let mut snake_iter = game.get_snake().body().iter();

    // empty board with 0
    let mut state = vec![vec![2; width]; height];

    // food
    state[food_y as usize][food_x as usize] = 3;

    // head
    if let Some((head_x, head_y)) = snake_iter.next() {
        state[*head_y as usize][*head_x as usize] = 1;
    }

    // body
    for (snake_x, snake_y) in snake_iter {
        state[*snake_y as usize][*snake_x as usize] = 0;
    }

    state
}

// Returns compatible state for neural network input
pub fn state_model() -> Vec<Vec<f32>> {
    let mut game = GAME.lock().unwrap();
    let width = game.width() as usize;
    let height = game.height() as usize;

    let body_all = game.get_snake().body().clone();
    let head = &body_all[..1][0].clone();
    let body = &body_all[1..];

    // get food distance matrix, rolled to food position
    let food = game.get_food().unwrap();
    let center = (width as i16 / 2, height as i16 / 2);
    let mut m_state = util::roll_2d(util::dist_2d(width, height), util::dist_coord(food, center));

    // add body weight matrix, replacing cell values with body weights (negative)
    m_state = util::add_weight_matrix(m_state, body);

    // center matrix to head
    m_state = util::roll_2d(m_state, util::dist_coord(center, *head));

    // set head position to 0
    *m_state
        .get_mut((center.0 as usize, center.1 as usize))
        .unwrap() = 0.0;

    // rotate array to face always into current direction of the snake (upwards)
    m_state = util::rot90(m_state, 4 - game.snake_dir() as usize);

    let mut result = vec![];
    for row in m_state.rows() {
        result.push(row.to_vec());
    }

    result
}

#[wasm_bindgen]
pub fn state_js() -> JsValue {
    JsValue::from_serde(&state()).unwrap()
}

#[wasm_bindgen]
pub fn state_model_js() -> JsValue {
    JsValue::from_serde(&state_model()).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dir() {
        reset(16, 16);
        snake_up();
        tick();
        assert!(GAME.lock().unwrap().get_snake().direction() == Direction::Top);

        snake_left();
        tick();
        assert!(GAME.lock().unwrap().get_snake().direction() == Direction::Left);

        snake_down();
        tick();
        assert!(GAME.lock().unwrap().get_snake().direction() == Direction::Bottom);

        snake_right();
        tick();
        assert!(GAME.lock().unwrap().get_snake().direction() == Direction::Right);
    }
}
