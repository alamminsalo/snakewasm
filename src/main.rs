extern crate termion;
extern crate rand;

mod game;
mod snake;
mod util;

use game::Game;

use std::{thread, time};

fn main() {
    let mut game = Game::new(32,32);

    let interval = time::Duration::from_millis(100);

    print!("{}{}", termion::clear::All, termion::cursor::Hide);

    {
        let snake = game.snakes.get_mut(0).unwrap();
        snake.dir(snake::Direction::Left);
        snake.grow();
    }
    game.tick();
    game.tick();
    {
        let snake = game.snakes.get_mut(0).unwrap();
        snake.dir(snake::Direction::Bottom);
        snake.grow();
    }
    game.tick();
    game.tick();
    {
        let snake = game.snakes.get_mut(0).unwrap();
        snake.dir(snake::Direction::Left);
        snake.grow();
    }
    game.tick();
    game.tick();

    {
        let snake = game.snakes.get_mut(0).unwrap();
        snake.dir(snake::Direction::Top);
        snake.grow();
    }
    game.tick();
    game.tick();

    print!("{}", termion::cursor::Restore);
}

