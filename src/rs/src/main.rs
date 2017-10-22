
extern crate rand;

#[macro_use]
extern crate lazy_static;

pub mod game;
pub mod snake;
mod util;

use std::sync::Mutex;
use self::game::Game;
use self::snake::Direction;

lazy_static! {
  static ref GAME: Mutex<Game> = Mutex::new(Game::new(100,100));
}

fn main() {
  println!("rust code running..");
}

#[no_mangle]
pub fn is_ended() -> bool {
  GAME.lock().unwrap().is_ended()
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
  GAME.lock().unwrap().get_snake().body().len()
}

#[no_mangle]
pub fn snake_x_at(idx: usize) -> i16 {
  GAME.lock().unwrap().get_snake().body()[idx].0
}

#[no_mangle]
pub fn snake_y_at(idx: usize) -> i16 {
  GAME.lock().unwrap().get_snake().body()[idx].1
}

#[no_mangle]
pub fn snake_up() {
  GAME.lock().unwrap().get_snake().dir(Direction::Top);
}

#[no_mangle]
pub fn snake_left() {
  GAME.lock().unwrap().get_snake().dir(Direction::Left);
}

#[no_mangle]
pub fn snake_right() {
  GAME.lock().unwrap().get_snake().dir(Direction::Right);
}

#[no_mangle]
pub fn snake_down() {
  GAME.lock().unwrap().get_snake().dir(Direction::Bottom);
}

#[no_mangle]
pub fn food_x() -> i16 {
  GAME.lock().unwrap().get_food().unwrap_or((-1,-1)).0
}

#[no_mangle]
pub fn food_y() -> i16 {
  GAME.lock().unwrap().get_food().unwrap_or((-1,-1)).1
}

#[no_mangle]
pub fn game_height() -> u16 {
  GAME.lock().unwrap().height()
}

#[no_mangle]
pub fn game_width() -> u16 {
  GAME.lock().unwrap().width()
}

#[cfg(test)]
mod tests {
#[test]
fn test_dir() {
  reset(16,16);
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

