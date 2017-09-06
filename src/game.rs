use snake::{Snake, Direction};

use std::cmp;

struct Game {
    w: u16,
    h: u16,
    d: Direction,
    pub snake: Snake,
    pub food: Option<(i16,i16)>
}

// Clamps value between bounds
fn wrap(min: i16, val: i16, max: i16) -> i16 {
    if val < min {
        return max;
    }
    if val > max {
        return min;
    }
    val
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
    fn translate(&self, pos: (i16,i16)) -> (i16,i16) {
        (wrap(0, pos.0, self.w as i16 - 1), wrap(0, pos.1, self.h as i16 - 1))
    }

    // Ticks game state
    pub fn tick(&mut self) {
        let peeked = self.snake.peek(self.d.clone());
        let translated = self.translate(peeked);
        self.snake.goto(translated, None);
    }
}

#[test]
fn test_tick() {
    let mut game = Game::new(32, 32);
    game.tick();
    game.tick();
    game.tick();

    game.d = Direction::Bottom;
    game.tick();
    game.tick();
    game.tick();
    game.tick();

    game.d = Direction::Top;
    game.tick();
    game.tick();
    game.tick();
    game.tick();

    assert_eq!((29, 13), game.snake.head());
}


