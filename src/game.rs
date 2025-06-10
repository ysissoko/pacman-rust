// grid.rs
use piston_window::*;

use crate::ghost::Ghost;
use crate::grid::{Grid, TileType};
use crate::player::Pacman;
use crate::enums::Direction;
use crate::utils::get_speed_for_level;
use crate::states::{
    State,
    ChaseState,
    ScatterState,
    FrightenedState,
    EatenState,
};
use crate::pathfinding::{
    AStar,
};
use crate::constants::{
    GRID_WIDTH,
    GRID_HEIGHT,
    CELL_SIZE,
    PACMAN_INITIAL_LIVES,
    PACMAN_INITIAL_POS,
    BLINKY_INITIAL_POS,
    PINKY_INITIAL_POS,
    CLYDE_INITIAL_POS,
    INKY_INITIAL_POS,
    PACMAN_INITIAL_SCORE,
    BASE_PACMAN_SPEED,
    BASE_GHOST_SPEED,
    BASE_PACMAN_MIN_SPEED,
    BASE_GHOST_MIN_SPEED,
};
pub struct Game {
    ghosts: Vec<Ghost>,
    pacman: Pacman,
    grid: Grid,
    level: usize,
    pacman_timer: f64,
    ghost_timer: f64,
}

impl Game {
    pub fn new() -> Self {
        let grid = Grid::new("grid.map", CELL_SIZE, GRID_WIDTH, GRID_HEIGHT);

        let mut ghosts: Vec<Ghost> = Vec::new();
        let pacman = Pacman::new(String::from("Pacman"), PACMAN_INITIAL_POS, PACMAN_INITIAL_LIVES, Direction::Left, PACMAN_INITIAL_SCORE);
        
        ghosts.push(Ghost::new(
            String::from("Blinky"),
            BLINKY_INITIAL_POS,
            Box::new(ChaseState::new()),
            AStar::new()
        ));

        // ghosts.push(Ghost::new(
        //     String::from("Pinky"),
        //     PINKY_INITIAL_POS,
        //     Box::new(ChaseState::new()),
        //     AStar::new()
        // ));
        
        // ghosts.push(Ghost::new(
        //     String::from("Clyde"),
        //     CLYDE_INITIAL_POS,
        //     Box::new(ChaseState::new()),
        //     AStar::new()
        // ));

        // ghosts.push(Ghost::new(
        //     String::from("Inky"),
        //     INKY_INITIAL_POS,
        //     Box::new(ChaseState::new()),
        //     AStar::new()
        // ));

        Game {
            pacman,
            ghosts,
            grid,
            level: 1,
            pacman_timer: 0.0,
            ghost_timer: 0.0,
        }
    }

    fn move_ghosts(&mut self) {
        for ghost in &mut self.ghosts {
            ghost.move_around(self.pacman.get_pos());
        }
    }
    
    pub fn render(&self, c: Context, graphics: &mut G2d) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        // Clear the screen.
        clear(BLACK, graphics);
        // Draw the grid
        for tile in self.grid.get_tiles() {
            let square = rectangle::square(tile.get_pixels_x() as f64, tile.get_pixels_y() as f64, tile.get_size().0 as f64);
            let color = match tile.get_type() {
                TileType::Wall => [0.0, 0.0, 1.0, 1.0], // Blue
                TileType::Pellet => [1.0, 1.0, 1.0, 1.0], // White
                TileType::PowerPellet => [1.0, 1.0, 0.0, 1.0], // Yellow
                TileType::GhostGate => [0.5, 0.5, 0.5, 1.0], // Gray
                _ => [0.0, 0.0, 0.0, 1.0], // Black for floor or unknown
            };

            rectangle(color, square, c.transform, graphics);
        }

        // Draw the ghosts
        for ghost in &self.ghosts {
            let square = rectangle::square(ghost.get_pixels_x() as f64, ghost.get_pixels_y() as f64, CELL_SIZE as f64);
            rectangle([1.0, 0.0, 0.0, 1.0], square, c.transform, graphics);
        }

        // Draw Pacman
        let square = rectangle::square(self.pacman.get_pixels_x() as f64, self.pacman.get_pixels_y() as f64, CELL_SIZE as f64);
        rectangle([1.0, 1.0, 0.0, 1.0], square, c.transform, graphics);
    }

    pub fn handle_input(&mut self, input: &Input) {
        // Handle input events here
        match *input {
            Input::Button(ButtonArgs { button: Button::Keyboard(Key::Left), .. }) => {
                self.pacman.direction = Direction::Left;
            }
            Input::Button(ButtonArgs { button: Button::Keyboard(Key::Up), .. }) => {
                self.pacman.direction = Direction::Up;
            }
            Input::Button(ButtonArgs { button: Button::Keyboard(Key::Right), .. }) => {
                self.pacman.direction = Direction::Right;
            }
            Input::Button(ButtonArgs { button: Button::Keyboard(Key::Down), .. }) => {
                self.pacman.direction = Direction::Down;
            }
            _ => {}
        }
    }

    pub fn update(&mut self, _dt: f64) {
        let pacman_interval = get_speed_for_level(BASE_GHOST_SPEED, self.level, BASE_PACMAN_MIN_SPEED);
        let ghost_interval = get_speed_for_level(BASE_GHOST_SPEED, self.level, BASE_GHOST_MIN_SPEED);
        self.pacman_timer += _dt;
        self.ghost_timer += _dt;
        if self.pacman_timer >= pacman_interval {
            self.pacman_timer = 0.0;
            // Pacman moves every pacman_interval seconds
            self.pacman.move_around();
        }

        if self.ghost_timer >= ghost_interval {
            self.ghost_timer = 0.0;
            // Ghosts move every ghost_interval seconds
            self.move_ghosts();
        }
    }
}
