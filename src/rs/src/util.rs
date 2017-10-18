use snake::Direction;

// Clamps value between bounds
pub fn wrap(min: i16, val: i16, max: i16) -> i16 {
    if val < min {
        return max;
    }
    if val > max {
        return min;
    }
    val
}

pub fn snake_angle(pd: &Direction, d: &Direction) -> char {
    if (*pd == Direction::Left && *d == Direction::Top)
        || (*pd == Direction::Bottom && *d == Direction::Right) {
        return '┗';
    }
    else if (*pd == Direction::Right && *d == Direction::Top)
    || (*pd == Direction::Bottom && *d == Direction::Left) {
        return '┛';
    }
    else if (*pd == Direction::Right && *d == Direction::Bottom)
    || (*pd == Direction::Top && *d == Direction::Left) {
        return '┓';
    }
    else if (*pd == Direction::Left && *d == Direction::Bottom)
    || (*pd == Direction::Top && *d == Direction::Right) {
        return '┏';
    }

    snake_head(&d)
}

pub fn snake_head(d: &Direction) -> char {
    match *d {
        Direction::Left => '━',
        Direction::Right => '━',
        Direction::Top => '┃',
        Direction::Bottom => '┃'
    }
}

