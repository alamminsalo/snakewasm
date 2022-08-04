// Snake object

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(PartialEq)]
pub struct Snake {
    body: Vec<(i16, i16)>,
    d: Direction,
    d_next: Option<Direction>,
}

impl Snake {
    pub fn direction(&self) -> Direction {
        self.d.clone()
    }

    pub fn body(&self) -> &Vec<(i16, i16)> {
        &self.body
    }

    pub fn new() -> Snake {
        Snake {
            body: vec![(5, 5), (6, 5), (7, 5), (8, 5)],
            d: Direction::Left,
            d_next: None,
        }
    }

    // Sets snake dir, if valid
    // Returns new dir
    pub fn dir(&mut self, dir: Direction) -> Direction {
        if dir != self.d
            && ![&dir, &self.d]
                .iter()
                .all(|&d| [Direction::Top, Direction::Bottom].contains(d))
            && ![&dir, &self.d]
                .iter()
                .all(|&d| [Direction::Left, Direction::Right].contains(d))
        {
            self.d_next = Some(dir);
        }

        self.direction().clone()
    }

    pub fn d(&self) -> Direction {
        self.d.clone()
    }

    // Peeks and returns the next place snake head will be
    pub fn peek(&self) -> (i16, i16) {
        let head = self.head();

        match self.d {
            Direction::Top => (head.0, head.1 - 1),
            Direction::Bottom => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        }
    }

    // Moves snake to direction
    // Returns new head pos
    pub fn mv(&mut self) -> (i16, i16) {
        if self.d_next != None {
            self.d = self.d_next.clone().unwrap();
            self.d_next = None;
        }

        self.body.pop();
        let peeked = self.peek();
        self.body.insert(0, peeked);

        self.head()
    }

    // Moves head to a new location
    // Returns new head pos
    pub fn goto(&mut self, pos: (i16, i16)) -> (i16, i16) {
        if self.d_next != None {
            self.d = self.d_next.clone().unwrap();
            self.d_next = None;
        }
        self.body.pop();
        self.body.insert(0, pos);
        self.head()
    }

    pub fn head(&self) -> (i16, i16) {
        self.body.first().unwrap().clone()
    }

    pub fn tail(&self) -> (i16, i16) {
        self.body.last().unwrap().clone()
    }

    pub fn grow(&mut self) {
        let tail = self.tail();
        self.body.push(tail.into());
    }

    pub fn turn_left(&mut self) {
        self.dir(match self.d {
            Direction::Top => Direction::Left,
            Direction::Left => Direction::Bottom,
            Direction::Bottom => Direction::Right,
            Direction::Right => Direction::Top,
        });
    }

    pub fn turn_right(&mut self) {
        self.dir(match self.d {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top,
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_grow() {
        let mut snake = Snake::new();

        let len0 = snake.body.len();

        snake.grow();
        snake.grow();
        snake.grow();

        let len1 = snake.body.len();

        assert_eq!(len1, len0 + 3);
    }

    #[test]
    fn test_dir() {
        let mut snake = Snake::new();
        assert!(snake.dir(Direction::Right) == Direction::Left);
        assert!(snake.dir(Direction::Left) == Direction::Left);

        snake.dir(Direction::Top);
        snake.mv();
        assert!(snake.dir(Direction::Bottom) == Direction::Top);
        assert!(snake.dir(Direction::Top) == Direction::Top);
    }

    #[test]
    fn test_mv() {
        let mut snake = Snake::new();
        snake.goto((0, 0));

        snake.dir(Direction::Top);
        assert_eq!((0, -1), snake.mv());
        snake.dir(Direction::Left);
        assert_eq!((-1, -1), snake.mv());
        snake.dir(Direction::Bottom);
        assert_eq!((-1, 0), snake.mv());
    }

    #[test]
    fn test_goto() {
        let mut snake = Snake::new();
        let pos0 = snake.head();
        let pos1 = snake.goto((99, 99));

        assert_eq!((pos0.0 + 99, pos0.1 + 94), pos1);
    }

    #[test]
    fn test_goto_mv() {
        let mut snake = Snake::new();
        snake.goto((99, 99));
        snake.dir(Direction::Bottom);
        snake.mv();
        snake.mv();

        assert_eq!((99, 100), snake.body[1]);
    }

    #[test]
    fn test_peek() {
        let mut snake = Snake::new();
        snake.goto((99, 99));
        snake.dir(Direction::Bottom);
        snake.mv();
        snake.mv();

        snake.dir(Direction::Right);
        assert_eq!((100, 101), snake.peek());
        assert_eq!((99, 101), snake.head());
    }
}
