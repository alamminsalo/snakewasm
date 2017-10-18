
extern crate rand;

#[macro_use]
extern crate lazy_static;

pub mod game;
pub mod snake;
mod util;

use std::sync::Mutex;
use self::game::Game;

lazy_static! {
	static ref GAME: Mutex<Game> = Mutex::new(Game::new(100,100));
}

fn main() {
	println!("rust code running..");
}

#[no_mangle]
pub fn tick() {
	GAME.lock().unwrap().tick();
}

#[no_mangle]
pub fn reset(w: u16, h: u16) {
	GAME.lock().unwrap().reset(w, h);
}

#[no_mangle]
pub fn snake_len() -> usize {
	GAME.lock().unwrap().snakes[0].body.len()
}

#[no_mangle]
pub fn snake_x_at(idx: usize) -> i16 {
	GAME.lock().unwrap().snakes[0].body[idx].0
}

#[no_mangle]
pub fn snake_y_at(idx: usize) -> i16 {
	GAME.lock().unwrap().snakes[0].body[idx].1
}

#[no_mangle]
pub fn food_x() -> i16 {
	GAME.lock().unwrap().food.unwrap_or((-1,-1)).0
}

#[no_mangle]
pub fn food_y() -> i16 {
	GAME.lock().unwrap().food.unwrap_or((-1,-1)).1
}

#[no_mangle]
pub fn game_height() -> u16 {
	GAME.lock().unwrap().h
}

#[no_mangle]
pub fn game_width() -> u16 {
	GAME.lock().unwrap().w
}

