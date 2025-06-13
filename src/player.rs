use crate::constants::{CELL_SIZE, EATEN_GHOSTS_MULTIPLIERS, GRID_HEIGHT, GRID_WIDTH};
use crate::enums::{Fruit, Direction};
use crate::grid::Grid;

pub struct Pacman {
    pub name: String,
    pub pos: (i32, i32),
    pub direction: Direction,
    pub expected_direction: Option<Direction>,
    pub lives: i32,
    pub score: i32,
    pub eaten_pellets: Vec<(i32, i32)>
}

impl Pacman {
    pub fn new(name: String, pos: (i32, i32), lives: i32, direction: Direction, score: i32) -> Self {
        Pacman { name, pos, direction, lives, score, expected_direction: None, eaten_pellets: Vec::new() }
    }

    pub fn move_around(&mut self, grid: &Grid) {
        if let Some(expected_direction) = self.expected_direction {
            if self.can_move(self.expected_direction.unwrap(), grid) {
                self.direction = expected_direction;
            }
        }

        if self.can_move(self.direction, grid) {
            self.update_position();
        }
    }

    fn update_position(&mut self) {
         match self.direction {
            Direction::Up => {
                self.pos.1 -= 1;
                if self.pos.1 < 0 {
                    self.pos.1 = 0;
                }
            },
            Direction::Down => {
                self.pos.1 += 1;
                if self.pos.1 > GRID_HEIGHT - 1 {
                    self.pos.1 = 0;
                }
            },
            Direction::Left => {
                self.pos.0 -= 1;
                if self.pos.0 < 0 {
                    self.pos.0 = GRID_WIDTH - 1;
                }
            },
            Direction::Right => {
                self.pos.0 += 1;
                if self.pos.0 > GRID_WIDTH - 1 {
                    self.pos.0 = 0;
                }
            },
        }
    }

    fn can_move(&self, direction: Direction, grid: &Grid) -> bool {
        // Check if the next tile in the current direction is valid
        let (x, y) = self.pos;

        let new_pos = match direction {
            Direction::Up => (x, if y - 1 < 0 { GRID_HEIGHT - 1 } else { y - 1 }),
            Direction::Down => (x, if y + 1 >= GRID_HEIGHT { 0 } else { y + 1 }),
            Direction::Left => (if x - 1 < 0 { GRID_WIDTH - 1 } else { x - 1}, y),
            Direction::Right => (if x + 1 >= GRID_WIDTH { 0 } else { x + 1 }, y),
        };

        let next_tile = grid.get_tile(new_pos);

        if let Some(next_tile) = next_tile {
            return next_tile.is_walkable_for_pacman();
        }

        false
    }

    pub fn get_pixels_x(&self) -> i32 {
        self.pos.0 * CELL_SIZE
    }
    pub fn get_pixels_y(&self) -> i32 {
        self.pos.1 * CELL_SIZE
    }
    pub fn eat_pellet(&mut self) {
        self.eaten_pellets.push(self.pos);
        self.score += 10;
    }
    pub fn eat_power_pellet(&mut self) {
        self.eaten_pellets.push(self.pos);
        self.score += 50;
    }
    pub fn lose_life(&mut self) {
        self.lives -= 1;
    }
    pub fn get_lives(&self) -> i32 {
        self.lives
    }
    pub fn get_score(&self) -> i32 {
        self.score
    }
    pub fn eat_ghost(&mut self, multiplier: usize) {
        self.score += EATEN_GHOSTS_MULTIPLIERS[multiplier];
    }
    pub fn eat_fruit(&mut self, fruit: Fruit) {
        match fruit {
            Fruit::Cherry => self.score += 100,
            Fruit::Strawberry => self.score += 300,
            Fruit::Orange => self.score += 500,
            Fruit::Apple => self.score += 700,
            Fruit::Melon => self.score += 1000,
            Fruit::Galaxian => self.score += 2000,
            Fruit::Bell => self.score += 3000,
            Fruit::Key => self.score += 5000,
        }
    }
}
