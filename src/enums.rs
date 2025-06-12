
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub enum Fruit {
    Cherry,
    Strawberry,
    Orange,
    Apple,
    Melon,
    Galaxian,
    Bell,
    Key,
}

#[derive(PartialEq)]
pub enum GhostState {
    Chase,
    Scatter,
    Frightened,
    Eaten,
}
