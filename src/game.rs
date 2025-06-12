use std::collections::VecDeque;

use rand::Rng;
use piston_window::ellipse::circle;
// grid.rs
use piston_window::*;

use crate::ghost::{self, get_target_clyde, get_target_inky, get_target_pinky, Ghost};
use crate::grid::{Grid, TileType};
use crate::player::Pacman;
use crate::enums::{Direction, GhostState};
use crate::utils::get_speed_for_level;

use crate::constants::{
    BASE_GHOST_MIN_SPEED, BASE_GHOST_SPEED, BASE_PACMAN_MIN_SPEED, BLINKY_COLOR, BLINKY_INITIAL_POS, BLINKY_NAME, BOTTOM_LEFT_CORNER, BOTTOM_RIGHT_CORNER, CELL_SIZE, CLYDE_COLOR, CLYDE_INITIAL_POS, CLYDE_NAME, GHOSTS_HOUSE_POS, GHOST_EATEN_COLOR, GHOST_FRIGHTENED_COLOR, GHOST_GATE_COLOR, GRID_HEIGHT, GRID_WIDTH, INKY_COLOR, INKY_INITIAL_POS, INKY_NAME, PACMAN_COLOR, PACMAN_INITIAL_LIVES, PACMAN_INITIAL_POS, PACMAN_INITIAL_SCORE, PELLET_COLOR, PINKY_COLOR, PINKY_INITIAL_POS, PINKY_NAME, POWER_PELLET_COLOR, TOP_LEFT_CORNER, TOP_RIGHT_CORNER, WALL_COLOR
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
    frightened_mode: bool,
    frightened_timer: f64,
    ghosts_state: GhostState,
}

fn get_color_from_state(ghost: &Ghost) -> [f32; 4] {
    match ghost.state {
        GhostState::Frightened => GHOST_FRIGHTENED_COLOR,
        GhostState::Eaten => GHOST_EATEN_COLOR, // Eaten ghosts are also shown as frightened
        _ => match ghost.name.as_str() {
            BLINKY_NAME => BLINKY_COLOR,
            PINKY_NAME => PINKY_COLOR,
            INKY_NAME => INKY_COLOR,
            CLYDE_NAME => CLYDE_COLOR,
            _ => [1.0, 1.0, 1.0, 1.0], // Default color for unknown ghosts
        }
    }
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
            state_intervals: vec![20., 7., 20., 5., 1.].into(), // Initialize with 4 intervals for each ghost
            switch_state_interval: Some(7.),
            frightened_mode: false,
            frightened_timer: 0.,
            ghosts_state: GhostState::Scatter, // Start with ghosts in scatter state
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
                GhostState::Eaten => {
                    GHOSTS_HOUSE_POS // Eaten ghosts return to their home position
                }
                _ => {
                    // Default case, should not happen
                    (0, 0)
                }
            };

            if ghost.pos == GHOSTS_HOUSE_POS && ghost.state == GhostState::Eaten {
                // If the ghost is in the eaten state, it should return to its home position
                ghost.state = self.ghosts_state; // Reset to scatter state after being eaten
                continue; // Skip moving this ghost
            }
            ghost.move_around(target, &self.grid);
        }
    }

    fn check_collision(&mut self) {
        self.grid.get_tile(self.pacman.pos).map(|tile| {
            if self.pacman.eaten_pellets.contains(&tile.pos) {
                // If Pacman has already
                // eaten this pellet, skip it
                return;
            }
            if tile.type_ == TileType::Pellet {
                // If Pacman eats a pellet
                self.pacman.eat_pellet();
            } else if tile.type_ == TileType::PowerPellet {
                // If Pacman eats a power pellet
                self.pacman.eat_power_pellet();
                self.frightened_mode = true;
                self.frightened_timer = 0.0;

                // Set ghosts to frightened state
                for ghost in &mut self.ghosts {
                    if ghost.state != GhostState::Eaten {
                        println!("Pacman ate a power pellet! Ghosts are now frightened.");
                        ghost.state = GhostState::Frightened;
                    }
                }
            }
        });

    }
    
    fn check_ghosts_collision(&mut self) {
                // Check for collisions with ghosts
        for ghost in &mut self.ghosts {
            if ghost.pos == self.pacman.pos {
                if self.frightened_mode {
                    // If Pacman is in frightened mode, eat the ghost
                    println!("Pacman ate {}", ghost.name);
                    ghost.state = GhostState::Eaten; // Set ghost to eaten statee
                } else {
                    // If Pacman is not in frightened mode, lose a life
                    println!("Pacman collided with {}", ghost.name);
                    self.pacman.lose_life();
                    if self.pacman.lives <= 0 {
                        println!("Game Over! Pacman has no lives left.");
                        // Handle game over logic here
                    }
                }
            }
        }
    }
    pub fn render(&self, c: Context, graphics: &mut G2d) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        // Clear the screen.
        clear(BLACK, graphics);
        
        // Pre-calculate common values
        let transform = c.transform;
        let pellet_size = CELL_SIZE as f64 * 0.2;
        let power_size = CELL_SIZE as f64 * 0.5;
        
        // Create a lookup HashSet for eaten pellets (faster lookups)
        let eaten_pellets_set = &self.pacman.eaten_pellets;
        
        // Draw walls first (they don't change)
        for tile in self.grid.get_tiles() {
            if tile.type_ == TileType::Wall || tile.type_ == TileType::GhostGate {
                let tile_x = tile.get_pixels_x() as f64;
                let tile_y = tile.get_pixels_y() as f64;
                let tile_size = tile.get_size().0 as f64;
                
                let square = rectangle::square(tile_x, tile_y, tile_size);
                let color = match tile.type_ {
                    TileType::Wall => WALL_COLOR,
                    TileType::GhostGate => GHOST_GATE_COLOR,
                    _ => unreachable!(),
                };
                rectangle(color, square, transform, graphics);
            }
        }
        
        // Draw pellets (only if not eaten)
        for tile in self.grid.get_tiles() {
            // Skip non-pellet tiles and eaten pellets (single check)
            if (tile.type_ != TileType::Pellet && tile.type_ != TileType::PowerPellet) 
               || eaten_pellets_set.contains(&tile.pos) {
                continue;
            }

            let tile_x = tile.get_pixels_x() as f64;
            let tile_y = tile.get_pixels_y() as f64;
            let tile_size = tile.get_size().0 as f64;
            
            match tile.type_ {
                TileType::Pellet => {
                    // Regular pellet
                    let pellet_x = tile_x + (tile_size - pellet_size) / 2.0;
                    let pellet_y = tile_y + (tile_size - pellet_size) / 2.0;
                    let pellet_circle = circle(pellet_x, pellet_y, pellet_size / 2.0);
                    ellipse(PELLET_COLOR, pellet_circle, transform, graphics);
                },
                TileType::PowerPellet => {
                    // Power pellet
                    let power_x = tile_x + (tile_size - power_size) / 2.0;
                    let power_y = tile_y + (tile_size - power_size) / 2.0;
                    let power_circle = circle(power_x, power_y, power_size / 2.0);
                    ellipse(POWER_PELLET_COLOR, power_circle, transform, graphics);
                },
                _ => {}
            }
        }

        // Draw the ghosts and Pacman (unchanged)
        for ghost in &self.ghosts {
            let square = rectangle::square(ghost.get_pixels_x() as f64, ghost.get_pixels_y() as f64, CELL_SIZE as f64);
            rectangle(get_color_from_state(ghost), square, transform, graphics);
        }

        let square = rectangle::square(self.pacman.get_pixels_x() as f64, self.pacman.get_pixels_y() as f64, CELL_SIZE as f64);
        rectangle(PACMAN_COLOR, square, transform, graphics);
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

    pub fn get_frightened_duration(&self) -> i32 {
        // Duration for which ghosts are frightened after eating a power pellet
        if self.level > 6 { 0 } else { 7 - self.level as i32 }// 10 seconds
    }
    pub fn update(&mut self, _dt: f64) {
        let pacman_interval = get_speed_for_level(BASE_GHOST_SPEED, self.level, BASE_PACMAN_MIN_SPEED);
        let ghost_interval = get_speed_for_level(BASE_GHOST_SPEED, self.level, BASE_GHOST_MIN_SPEED);

        self.pacman_timer += _dt;
        self.ghost_timer += _dt;
        self.state_timer += _dt;


        if self.frightened_mode {
            self.frightened_timer += _dt;
            if self.frightened_timer >= self.get_frightened_duration().into() {
                // Reset frightened mode after 10 seconds
                self.frightened_mode = false;
                self.frightened_timer = 0.0;
                for ghost in &mut self.ghosts {
                    if ghost.state == GhostState::Frightened {
                        ghost.state = self.ghosts_state;
                    }
                }
            }
        }

        // Move Pacman and ghosts based on their timers
        if self.pacman_timer >= pacman_interval {
            self.pacman_timer = 0.0;
            // Pacman moves every pacman_interval seconds
            self.pacman.move_around(&self.grid);
            // Check for collisions with pellets or power pellets
            self.check_collision();
            // Add ghost collision check here too!
            self.check_ghosts_collision();
        }

        if self.ghost_timer >= ghost_interval {
            self.ghost_timer = 0.0;
            // Ghosts move every ghost_interval seconds
            self.move_ghosts();
            // Check for collisions with ghosts
            self.check_ghosts_collision();
        }
        
        if let Some(interval) = self.switch_state_interval {
            if self.state_timer >= interval {
                self.switch_state_interval = self.state_intervals.pop_front();

                if GhostState::Chase == self.ghosts_state {
                    println!("Switching from Chase to Scatter");
                    self.ghosts_state = GhostState::Scatter;
                } else if GhostState::Scatter == self.ghosts_state {
                    println!("Switching from Scatter to Chase");
                    self.ghosts_state = GhostState::Chase;
                }
                // Change ghost states every scatter_interval seconds
                for ghost in &mut self.ghosts.iter_mut().filter(|g| g.state != GhostState::Eaten && g.state != GhostState::Frightened) {
                    println!("Switching state for ghost: {} in {} s", ghost.name, self.state_timer);
                    ghost.state = self.ghosts_state;
                }
                self.state_timer = 0.0;
            }
        }
    }
}
