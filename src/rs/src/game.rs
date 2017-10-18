extern crate rand;
use rand::{Rng};

use snake::{Snake, Direction};
use util;

#[derive(PartialEq)]
pub struct Game {
    pub w: u16,
    pub h: u16,
    pub snakes: Vec<Snake>,
    pub food: Option<(i16,i16)>
}

impl Game {

    // Creates a new game
    pub fn new(width: u16, height: u16) -> Game {
        Game {
            w: width,
            h: height,
            snakes: vec![Snake::new()],
            food: None
        }
    }

    // Handle border-crossing and translates coordinates when needed
    fn translate(dim: (u16, u16), pos: (i16,i16)) -> (i16,i16) {
        (util::wrap(2, pos.0, dim.0 as i16 - 1), util::wrap(2, pos.1, dim.1 as i16 - 1))
    }

    pub fn reset(&mut self, w: u16, h: u16) {
    	self.w = w;
	self.h = h;
        self.snakes = vec![Snake::new()];
	self.food = None;
    }

    // Ticks game state
    pub fn tick(&mut self) {
        let dim = (self.w, self.h);
        for snake in self.snakes.iter_mut() {
            let peeked = snake.peek();
            let translated = Game::translate(dim, peeked);

            let h0 = snake.head();
            let d0 = snake.d();

            let head = snake.goto(translated);
            let d1 = snake.d();

            // Food grows snake
            if self.food != None && head == self.food.unwrap() {
                snake.grow();

                // Remove food
                self.food = None;
            }
        }

//        if self.food == None {
//            self.add_food();
//        }
    }

    fn grid(&self) -> Vec<(i16,i16)> {
        let mut g = vec![];
        for y in 0..self.h as i16 {
            for x in 0..self.w as i16 {
                g.push((x, y));
            }
        }
        g
    }

    fn add_food(&mut self) {
        let mut grid = self.grid();
        for snake in self.snakes.iter() {
            grid.retain(|x: &(i16,i16)| !snake.body.contains(x));
        }

        // Grid now contains only positions that are free
        self.food = Some(*rand::thread_rng().choose(&grid).unwrap());
    }
}

#[test]
fn test_tick() {
    let mut game = Game::new(32, 32);

    {
        let snake = game.snakes.get_mut(0).unwrap();
        snake.goto((0,0));
        snake.dir(Direction::Left);
    }

    game.tick();
    game.tick();
    game.tick();

    {
        let snake = game.snakes.get_mut(0).unwrap();
        assert_eq!((29, 0), snake.head());
        snake.dir(Direction::Right);
    }

    game.tick();
    game.tick();

    {
        let snake = game.snakes.get_mut(0).unwrap();
        assert_eq!((27, 0), snake.head());
    }
}


