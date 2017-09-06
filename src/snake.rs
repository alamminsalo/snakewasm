// Snake object

#[derive(Clone, PartialEq)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right
}

pub struct Snake {
    pub body: Vec<(i16,i16)>,
    pub d: Direction,
    pub autopilot: bool
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![(0,5),(1,5),(2,5),(3,5)],
            d: Direction::Left,
            autopilot: false
        }
    }

    // Handles dir
    fn dir(&self, mut dir: Direction) -> Direction {
        // Head can't go backwards
        if dir == Direction::Top && self.d == Direction::Bottom
            || dir == Direction::Bottom && self.d == Direction::Top
                || dir == Direction::Right && self.d == Direction::Left
                || dir == Direction::Left && self.d == Direction::Right {
                    dir = self.d.clone();
                }
        dir
    }

    // Moves snake
    pub fn mv(&mut self, dir: Direction) -> (i16, i16) {
        self.body.pop();
        self.d = self.dir(dir);
        
        // Insert to new pos
        let head = self.head();
        self.body.insert(0, match self.d {
            Direction::Top => (head.0, head.1 - 1),
            Direction::Bottom => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1)
        });

        // Return new head pos
        self.head()
    }

    // Peeks a place this direction would end up the snake
    pub fn peek(&self, dir: Direction) -> (i16,i16) {
        // Head can't go backwards
        let d = self.dir(dir);

        let head = self.head();
        match d {
            Direction::Top => (head.0, head.1 - 1),
            Direction::Bottom => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1)
        }
    }

    // Forces new head location
    pub fn goto(&mut self, pos: (i16,i16), dir: Option<Direction>) -> (i16,i16) {
        self.body.pop();
        self.body.insert(0, pos);
        if dir != None {
            self.d = dir.unwrap();
        }
        self.head()
    }

    pub fn head(&self) -> (i16,i16) {
        *self.body.first().unwrap()
    }

    pub fn tail(&self) -> (i16, i16) {
        *self.body.last().unwrap()
    }

    pub fn grow(&mut self) {
        let tail = self.tail();
        self.body.push(tail);
    }
}

/**======== TESTS BEGIN ==========**/
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
fn test_mv() {
    let mut snake = Snake::new();
    let pos0 = snake.head();

    snake.mv(Direction::Top);
    snake.mv(Direction::Left);
    snake.mv(Direction::Bottom);
    let pos1 = snake.mv(Direction::Top);

    assert_eq!((pos0.0 - 1, pos0.1 + 1) , pos1);
}

#[test]
fn test_goto() {
    let mut snake = Snake::new();
    let pos0 = snake.head();
    let pos1 = snake.goto((99,99), None);

    assert_eq!((pos0.0 + 99, pos0.1 + 94) , pos1);
}

#[test]
fn test_goto_mv() {
    let mut snake = Snake::new();
    snake.goto((99,99), None);
    snake.mv(Direction::Bottom);
    snake.mv(Direction::Bottom);

    assert_eq!((99,100) , snake.body[1]);
}

#[test]
fn test_peek() {
    let mut snake = Snake::new();
    snake.goto((99,99), None);
    snake.mv(Direction::Bottom);
    snake.mv(Direction::Bottom);

    assert_eq!((100,101) , snake.peek(Direction::Right));
    assert_eq!((99,101) , snake.head());
}

