use std::collections::VecDeque;

use rand::Rng;
use piston_window::ellipse::circle;
// grid.rs
use piston_window::*;

use crate::ghost::{get_target_clyde, get_target_inky, get_target_pinky, Ghost};
use crate::grid::{Grid, TileType};
use crate::player::Pacman;
use crate::enums::{Direction, GhostState};
use crate::utils::get_speed_for_level;

use crate::constants::{
    BASE_GHOST_MIN_SPEED, BASE_GHOST_SPEED, BASE_PACMAN_MIN_SPEED, BASE_PACMAN_SPEED, BLINKY_COLOR, BLINKY_INITIAL_POS, BLINKY_NAME, BOTTOM_LEFT_CORNER, BOTTOM_RIGHT_CORNER, CELL_SIZE, CLYDE_COLOR, CLYDE_INITIAL_POS, CLYDE_NAME, GHOST_GATE_COLOR, GRID_HEIGHT, GRID_WIDTH, INKY_COLOR, INKY_INITIAL_POS, INKY_NAME, PACMAN_COLOR, PACMAN_INITIAL_LIVES, PACMAN_INITIAL_POS, PACMAN_INITIAL_SCORE, PELLET_COLOR, PINKY_COLOR, PINKY_INITIAL_POS, PINKY_NAME, POWER_PELLET_COLOR, TOP_LEFT_CORNER, TOP_RIGHT_CORNER, WALL_COLOR
};
pub struct Game {
    ghosts: Vec<Ghost>,
    pacman: Pacman,
    grid: Grid,
    level: usize,
    pacman_timer: f64,
    ghost_timer: f64,
    state_timer: f64,
    switch_state_interval: Option<f64>,
    state_intervals: VecDeque<f64>,
}

impl Game {
    pub fn new() -> Self {
        let grid = Grid::new("grid.map", CELL_SIZE, GRID_WIDTH, GRID_HEIGHT);

        let mut ghosts: Vec<Ghost> = Vec::new();
        let pacman = Pacman::new(String::from("Pacman"), PACMAN_INITIAL_POS, PACMAN_INITIAL_LIVES, Direction::Left, PACMAN_INITIAL_SCORE);
        
        ghosts.push(Ghost::new(
            BLINKY_NAME.to_string(),
            BLINKY_INITIAL_POS,
            TOP_RIGHT_CORNER,
            BLINKY_COLOR
        ));

        ghosts.push(Ghost::new(
            PINKY_NAME.to_string(),
            PINKY_INITIAL_POS,
            TOP_LEFT_CORNER,
            PINKY_COLOR
        ));
        
        ghosts.push(Ghost::new(
            CLYDE_NAME.to_string(),
            CLYDE_INITIAL_POS,
            BOTTOM_LEFT_CORNER,
            CLYDE_COLOR
        ));

        ghosts.push(Ghost::new(
            INKY_NAME.to_string(),
            INKY_INITIAL_POS,
            BOTTOM_RIGHT_CORNER,
            INKY_COLOR
        ));

        Game {
            pacman,
            ghosts,
            grid,
            level: 1,
            pacman_timer: 0.0,
            ghost_timer: 0.0,
            state_timer: 0.0,
            state_intervals: vec![20., 7., 20., 5.].into(), // Initialize with 4 intervals for each ghost
            switch_state_interval: Some(7.)
        }
    }

    fn move_ghosts(&mut self) {
        // Find Blinky's position before the loop to avoid borrow conflicts
        let blinky_pos = self.ghosts.iter().find(|g| g.name == BLINKY_NAME).map(|g| g.pos);

        for ghost in &mut self.ghosts {
            let target = match ghost.state {
                GhostState::Chase => match ghost.name.as_str() {
                    PINKY_NAME => get_target_pinky(self.pacman.direction, self.pacman.pos),
                    INKY_NAME => get_target_inky(
                        self.pacman.direction,
                        self.pacman.pos,
                        blinky_pos.unwrap_or((0, 0))
                    ),
                    BLINKY_NAME => self.pacman.pos,
                    CLYDE_NAME => get_target_clyde(ghost.pos, self.pacman.pos),
                    _ => (0, 0), // Default case, should not happen
                },
                GhostState::Scatter => ghost.scatter_pos,
                GhostState::Frightened => {
                    // In frightened state, ghosts move randomly
                    let mut rng = rand::rng();
                    let random_x = rng.random_range(0..GRID_WIDTH as i32);
                    let random_y = rng.random_range(0..GRID_HEIGHT as i32);
                    (random_x, random_y)
                },
                _ => {
                    // Default case, should not happen
                    (0, 0)
                }
            };

            ghost.move_around(target, &self.grid);
        }
    }
    
    pub fn render(&self, c: Context, graphics: &mut G2d) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        // Clear the screen.
        clear(BLACK, graphics);
        
        // Draw the grid
        for tile in self.grid.get_tiles() {
            let tile_x = tile.get_pixels_x() as f64;
            let tile_y = tile.get_pixels_y() as f64;
            let tile_size = tile.get_size().0 as f64;
            
            match tile.get_type() {
                TileType::Pellet => {
                    // Draw small circle for regular pellets
                    let pellet_size = tile_size * 0.2; // 20% of cell size
                    let pellet_x = tile_x + (tile_size - pellet_size) / 2.0;
                    let pellet_y = tile_y + (tile_size - pellet_size) / 2.0;
                    let pellet_circle = circle(pellet_x, pellet_y, pellet_size / 2.0);
                    ellipse(PELLET_COLOR, pellet_circle, c.transform, graphics);
                },
                TileType::PowerPellet => {
                    // Draw larger circle for power pellets
                    let power_size = tile_size * 0.5; // 50% of cell size
                    let power_x = tile_x + (tile_size - power_size) / 2.0;
                    let power_y = tile_y + (tile_size - power_size) / 2.0;
                    let power_circle = circle(power_x, power_y, power_size / 2.0);
                    ellipse(POWER_PELLET_COLOR, power_circle, c.transform, graphics);
                },
                // Draw rectangles for walls and other elements
                TileType::Wall | TileType::GhostGate => {
                    let square = rectangle::square(tile_x, tile_y, tile_size);
                    let color = match tile.get_type() {
                        TileType::Wall => WALL_COLOR,
                        TileType::GhostGate => GHOST_GATE_COLOR,
                        _ => unreachable!(),
                    };
                    rectangle(color, square, c.transform, graphics);
                },
                // Don't draw anything for floor tiles (leave them black)
                _ => {},
            }
        }

        // Draw the ghosts
        for ghost in &self.ghosts {
            let square = rectangle::square(ghost.get_pixels_x() as f64, ghost.get_pixels_y() as f64, CELL_SIZE as f64);
            rectangle(ghost.color, square, c.transform, graphics);
        }

        // Draw Pacman
        let square = rectangle::square(self.pacman.get_pixels_x() as f64, self.pacman.get_pixels_y() as f64, CELL_SIZE as f64);
        rectangle(PACMAN_COLOR, square, c.transform, graphics);
    }

    pub fn handle_input(&mut self, button: &Button) {
        // Handle input events here
        match *button {
            Button::Keyboard(Key::Left) => {
                self.pacman.expected_direction = Some(Direction::Left);
            }
            Button::Keyboard(Key::Up) => {
                self.pacman.expected_direction = Some(Direction::Up);
            }
            Button::Keyboard(Key::Right) => {
                self.pacman.expected_direction = Some(Direction::Right);
            }
            Button::Keyboard(Key::Down) => {
                self.pacman.expected_direction = Some(Direction::Down);
            }
            _ => {}
        }
    }

    pub fn update(&mut self, _dt: f64) {
        let pacman_interval = get_speed_for_level(BASE_GHOST_SPEED, self.level, BASE_PACMAN_MIN_SPEED);
        let ghost_interval = get_speed_for_level(BASE_GHOST_SPEED, self.level, BASE_GHOST_MIN_SPEED);

        self.pacman_timer += _dt;
        self.ghost_timer += _dt;
        self.state_timer += _dt;

        if self.pacman_timer >= pacman_interval {
            self.pacman_timer = 0.0;
            // Pacman moves every pacman_interval seconds
            self.pacman.move_around(&self.grid);
        }

        if self.ghost_timer >= ghost_interval {
            self.ghost_timer = 0.0;
            // Ghosts move every ghost_interval seconds
            self.move_ghosts();
        }

        if let Some(interval) = self.switch_state_interval {
            if self.state_timer >= interval {
                self.switch_state_interval = self.state_intervals.pop_front();
                // Change ghost states every scatter_interval seconds
                for ghost in &mut self.ghosts {
                    println!("Switching state for ghost: {} in {} s", ghost.name, self.state_timer);
                    ghost.switch_state();
                }
                self.state_timer = 0.0;
            }
        }
    }
}
