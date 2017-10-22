// Snake object

use util;

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right
}

#[derive(PartialEq)]
pub struct Snake {
    pub body: Vec<(i16,i16)>,
    pub d: Direction,
    pub autopilot: bool
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![(5,5),(6,5),(7,5),(8,5)],
            d: Direction::Left,
            autopilot: false
        }
    }

    // Sets snake dir, if valid
    // Returns new dir
    pub fn dir(&mut self, dir: Direction) -> Direction {
        if dir != self.d
            && ![&dir, &self.d].iter().all(|&d| [Direction::Top, Direction::Bottom].contains(d)) 
            && ![&dir, &self.d].iter().all(|&d| [Direction::Left, Direction::Right].contains(d)) {

                    // Fix dir change
                    let h = self.head();

                    self.d = dir;
                }

        self.d.clone()
    }

    pub fn d(&self) -> Direction {
        self.d.clone()
    }

    // Peeks and returns the next place snake head will be
    pub fn peek(&self) -> (i16,i16) {
        let head = self.head();

        match self.d {
            Direction::Top => (head.0, head.1 - 1),
            Direction::Bottom => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1)
        }
    }

    // Moves snake to direction
    // Returns new head pos
    pub fn mv(&mut self) -> (i16, i16) {
        self.body.pop();
        let peeked = self.peek();
        self.body.insert(0, peeked);

        self.head()
    }

    // Moves head to a new location
    // Returns new head pos
    pub fn goto(&mut self, pos: (i16,i16)) -> (i16,i16) {
        self.body.pop();
        self.body.insert(0, pos);
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
    snake.goto((0,0));

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
    let pos1 = snake.goto((99,99));

    assert_eq!((pos0.0 + 99, pos0.1 + 94) , pos1);
}

#[test]
fn test_goto_mv() {
    let mut snake = Snake::new();
    snake.goto((99,99));
    snake.dir(Direction::Bottom);
    snake.mv();
    snake.mv();

    assert_eq!((99,100) , snake.body[1]);
}

#[test]
fn test_peek() {
    let mut snake = Snake::new();
    snake.goto((99,99));
    snake.dir(Direction::Bottom);
    snake.mv();
    snake.mv();

    snake.dir(Direction::Right);
    assert_eq!((100,101) , snake.peek());
    assert_eq!((99,101) , snake.head());
}

