extern crate rand;

use crate::snake::{Direction, Snake};
use crate::util;
use ndarray::Array;
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;

#[derive(PartialEq)]
#[wasm_bindgen]
pub struct Game {
    w: u16,
    h: u16,
    snakes: Vec<Snake>,
    food: Option<(i16, i16)>,
    ended: bool,
    tick_count: usize,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u16, height: u16) -> Game {
        Game {
            w: width,
            h: height,
            snakes: vec![Snake::new()],
            food: None,
            ended: false,
            tick_count: 0,
        }
    }

    pub fn width(&self) -> u16 {
        self.w
    }

    pub fn height(&self) -> u16 {
        self.h
    }

    pub fn input(&mut self, cmd: u8) {
        match cmd {
            0 => self.mut_snake().dir(Direction::Top),
            1 => self.mut_snake().dir(Direction::Bottom),
            2 => self.mut_snake().dir(Direction::Left),
            _ => self.mut_snake().dir(Direction::Right),
        };
    }

    pub fn input_turn(&mut self, cmd: u8) {
        match cmd {
            1 => self.mut_snake().turn_left(),
            2 => self.mut_snake().turn_right(),
            _ => {}
        }
    }

    fn snake(&self) -> Snake {
        self.snakes[0].clone()
    }

    fn mut_snake(&mut self) -> &mut Snake {
        &mut self.snakes[0]
    }

    fn food(&self) -> Option<(i16, i16)> {
        self.food
    }

    pub fn score(&self) -> usize {
        self.snake().body().len() - 4
    }

    pub fn set_food(&mut self, x: i16, y: i16) {
        let free = self.free_grid();
        let xy = (x, y);
        if free.contains(&xy) {
            self.food = Some(xy);
        }
    }

    pub fn is_ended(&self) -> bool {
        self.ended
    }

    //fn in_bounds(dim: (u16, u16), pos: (i16, i16)) -> bool {
    //    pos.0 >= 0 && (pos.0 as u16) < dim.0 && pos.1 >= 0 && (pos.1 as u16) < dim.1
    //}

    // Handle border-crossing and translates coordinates when needed
    fn translate(dim: (u16, u16), pos: (i16, i16)) -> (i16, i16) {
        (
            util::wrap(0, pos.0, dim.0 as i16 - 1),
            util::wrap(0, pos.1, dim.1 as i16 - 1),
        )
    }

    pub fn reset(&mut self) {
        self.snakes = vec![Snake::new()];
        self.food = None;
        self.ended = false;
        self.tick_count = 0;
    }

    // Ticks game state
    pub fn tick(&mut self) {
        if !self.ended {
            self.tick_count += 1;
            let dim = (self.w, self.h);
            for snake in self.snakes.iter_mut() {
                let peeked = snake.peek();
                // hitting self ends game, trim last bit of tail off when considering hitting self
                if snake.body()[1..snake.body().len() - 1].contains(&peeked) {
                    self.ended = true;
                } else {
                    // no wall, move to location
                    let translated = Game::translate(dim, peeked);
                    let head = snake.goto(translated);

                    // Food grows snake
                    if self.food != None && head == self.food.unwrap() {
                        snake.grow();

                        // Remove food
                        self.food = None;
                    }
                }
            }
            if self.food == None {
                self.add_food();
            }
        }
    }

    fn grid(&self) -> Vec<(i16, i16)> {
        let mut g = vec![];
        for y in 0..self.h as i16 {
            for x in 0..self.w as i16 {
                g.push((x, y));
            }
        }
        g
    }

    fn free_grid(&self) -> Vec<(i16, i16)> {
        let mut grid = self.grid();
        for snake in self.snakes.iter() {
            grid.retain(|x: &(i16, i16)| !snake.body().contains(x));
        }
        grid
    }

    fn add_food(&mut self) {
        // select food position randomly from list of positions that are free
        let grid = self.free_grid();
        self.food = grid.choose(&mut rand::thread_rng()).cloned()
    }

    pub fn tick_count(&self) -> usize {
        self.tick_count
    }

    pub fn snake_dir(&self) -> u8 {
        match self.snakes[0].direction() {
            Direction::Top => 0,
            Direction::Right => 1,
            Direction::Bottom => 2,
            Direction::Left => 3,
        }
    }

    pub fn state(&self) -> Vec<i8> {
        let width = self.width() as usize;
        let height = self.height() as usize;

        let (food_x, food_y) = self.food().unwrap();
        let snake = self.snake();
        let mut body_iter = snake.body().iter();

        // empty board with 0
        let mut state = vec![vec![2; width]; height];

        // food
        state[food_y as usize][food_x as usize] = 3;

        // head
        if let Some((head_x, head_y)) = body_iter.next() {
            state[*head_y as usize][*head_x as usize] = 1;
        }

        // body
        for (snake_x, snake_y) in body_iter {
            state[*snake_y as usize][*snake_x as usize] = 0;
        }

        state.into_iter().flatten().collect()
    }

    pub fn state_model(&self) -> Vec<f32> {
        let width = self.width() as usize;
        let height = self.height() as usize;

        let body_all = self.snake().body().clone();
        let head = &body_all[..1][0].clone();
        let body = &body_all[1..];

        // get food distance matrix, rolled to food position
        let food = self.food().unwrap();
        let center = (width as i16 / 2, height as i16 / 2);
        let mut m_state =
            util::roll_2d(util::dist_2d(width, height), util::dist_coord(food, center));

        // add body weight matrix, replacing cell values with body weights (negative)
        m_state = util::add_weight_matrix(m_state, body);

        // center matrix to head
        m_state = util::roll_2d(m_state, util::dist_coord(center, *head));

        // set head position to 0
        *m_state
            .get_mut((center.0 as usize, center.1 as usize))
            .unwrap() = 0.0;

        // rotate array to face always into current direction of the snake (upwards)
        m_state = util::rot90(m_state, 4 - self.snake_dir() as usize);

        // return as 1d vec
        Array::from_iter(m_state.iter().cloned()).to_vec()
    }

    pub fn state_params(&self) -> Vec<u8> {
        let head = self.snake().head();
        vec![head.0 as u8, head.1 as u8, self.snake_dir()]
    }

    // Javascript API for getting structs
    #[cfg(feature = "js")]
    pub fn state_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.state()).unwrap()
    }

    #[cfg(feature = "js")]
    pub fn state_model_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.state_model()).unwrap()
    }

    #[cfg(feature = "js")]
    pub fn state_params_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.state_params()).unwrap()
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_tick() {
//         let mut game = Game::new(32, 32);
//
//         {
//             let snake = game.snakes.get_mut(0).unwrap();
//             snake.goto((0, 0));
//             snake.dir(Direction::Left);
//         }
//
//         game.tick();
//         game.tick();
//         game.tick();
//
//         {
//             let snake = game.snakes.get_mut(0).unwrap();
//             assert_eq!((29, 0), snake.head());
//             snake.dir(Direction::Right);
//         }
//
//         game.tick();
//         game.tick();
//
//         {
//             let snake = game.snakes.get_mut(0).unwrap();
//             assert_eq!((27, 0), snake.head());
//         }
//     }
//
//     #[test]
//     fn test_grid() {
//         let mut game = Game::new(10, 10);
//         {
//             let snake = game.snakes.get_mut(0).unwrap();
//             snake.goto((0, 0));
//             snake.dir(Direction::Left);
//         }
//
//         game.tick();
//         game.tick();
//
//         let grid = game.grid();
//         let free = game.free_grid();
//         assert!(free.len() < game.grid().len());
//
//         {
//             let snake = game.snakes.get_mut(0).unwrap();
//             assert!(free.len() + snake.body().len() == grid.len());
//             for part in &snake.body() {
//                 assert!(!free.contains(&part));
//             }
//         }
//     }
// }
