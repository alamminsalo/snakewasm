use snake::{Snake, Direction};

use std::cmp;

struct Game {
    w: u16,
    h: u16,
    snake: Snake,
    d: Direction,
    food: Option<(i16,i16)>
}

// Clamps value between bounds
fn clamp(min: i16, val: i16, max: i16) -> i16 {
    cmp::max(0, cmp::min(max, val))
}

impl Game {

    // Creates a new game
    pub fn new(width: u16, height: u16) -> Game {
        Game {
            w: width,
            h: height,
            snake: Snake::new(),
            d: Direction::Left,
            food: None

        }
    }

    // Handlers border-crossing and translates coordinates when needed
    fn translate(&self, mut pos: (i16,i16)) -> (i16,i16) {
        (clamp(0, pos.0, self.w as i16 - 1), clamp(0, pos.1, self.h as i16 - 1))
    }

    // Ticks game state
    pub fn tick(&mut self) {
        let peeked = self.snake.peek(self.d.clone());
        let translated = self.translate(peeked);
        self.snake.goto(translated, None);
    }
}

