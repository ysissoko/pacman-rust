use crate::constants::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};
use crate::enums::{Fruit, Direction};

pub struct Pacman {
    pub name: String,
    pub pos: (i32, i32),
    pub direction: Direction,
    lives: i32,
    score: i32,
}

impl Pacman {
    pub fn new(name: String, pos: (i32, i32), lives: i32, direction: Direction, score: i32) -> Self {
        Pacman { name, pos, direction, lives, score }
    }

    pub fn move_around(&mut self) {
        println!("{} is at position {:?}", self.name, self.pos);
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
        println!("{} moved to position {:?}", self.name, self.pos);
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.pos
    }
    pub fn set_pos(&mut self, new_pos: (i32, i32)) {
        self.pos = new_pos;
    }
    pub fn get_pixels_x(&self) -> i32 {
        self.pos.0 * CELL_SIZE
    }
    pub fn get_pixels_y(&self) -> i32 {
        self.pos.1 * CELL_SIZE
    }
    pub fn eat_pellet(&mut self) {
        self.score += 10;
    }
    pub fn eat_power_pellet(&mut self) {
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
    pub fn eat_ghost(&mut self, multiplier: i32) {
        self.score += multiplier;
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
