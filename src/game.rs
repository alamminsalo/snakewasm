use snake::{Snake, Direction};
use util;

struct Game {
    w: u16,
    h: u16,
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
        (util::wrap(0, pos.0, dim.0 as i16 - 1), util::wrap(0, pos.1, dim.1 as i16 - 1))
    }

    // Ticks game state
    pub fn tick(&mut self) {
        let dim = (self.w, self.h);
        for snake in self.snakes.iter_mut() {
            let peeked = snake.peek();
            let translated = Game::translate(dim, peeked);

            let pos = snake.goto(translated);

            // Food grows snake
            if self.food != None && pos == self.food.unwrap() {
                snake.grow();

                // Remove food
                self.food = None;
            }
        }

        if self.food == None {
            self.addFood();
        }
    }

    fn addFood(&mut self) {
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


