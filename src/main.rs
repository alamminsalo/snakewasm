extern crate termion;
extern crate rand;

mod game;
mod snake;
mod util;

use game::Game;
use snake::Direction;

use std::{thread, time};
use rand::Rng;

fn main() {
    let mut game = Game::new(32,32);
    let interval = time::Duration::from_millis(33);


    let clear = || print!("{}{}", termion::cursor::Goto(1,1), termion::clear::All);
    clear();
    game.draw_borders();

    print!("{}", termion::cursor::Hide);

    while true {
        game.tick();

        {
            let snake = game.snakes.get_mut(0).unwrap();
            snake.dir(rand::thread_rng().choose(&[Direction::Left, Direction::Right, Direction::Bottom, Direction::Top]).unwrap().clone());
            snake.grow();
        }

        thread::sleep(interval);
    }
//
//    {
        let snake = game.snakes.get_mut(0).unwrap();
        snake.dir(snake::Direction::Left);
        snake.grow();
//    }
//    game.tick();
//    game.tick();
//    {
//        let snake = game.snakes.get_mut(0).unwrap();
//        snake.dir(snake::Direction::Bottom);
//        snake.grow();
//    }
//    game.tick();
//    game.tick();
//    {
//        let snake = game.snakes.get_mut(0).unwrap();
//        snake.dir(snake::Direction::Left);
//        snake.grow();
//    }
//    game.tick();
//
//    {
//        let snake = game.snakes.get_mut(0).unwrap();
//        snake.dir(snake::Direction::Top);
//        snake.grow();
//    }
//    game.tick();
//    game.tick();

//    print!("{}", termion::cursor::Restore);
}

