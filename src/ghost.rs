use crate::enums::{Direction, GhostState};
use crate::grid::Grid;
use crate::constants::{BOTTOM_LEFT_CORNER, BOTTOM_RIGHT_CORNER, CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, TOP_LEFT_CORNER, TOP_RIGHT_CORNER};
use crate::player::Pacman;
use crate::utils::manhattan_distance;

pub fn calculate_inky_vec(ahead: (i32, i32), blinky_pos: (i32, i32)) -> (i32, i32) {
    // Inky's target is two tiles in front of Pacman plus the vector from Inky to Blinky
    let dx = ahead.0 - blinky_pos.0;
    let dy = ahead.1 - blinky_pos.1;
    (ahead.0 + 2 * dx, ahead.1 + 2* dy)
}

pub fn get_target_inky(pacman_dir: Direction, pacman_pos: (i32, i32), blinky_pos: (i32, i32)) -> (i32, i32) {
    let (pacman_x, pacman_y) = pacman_pos;
    // Inky's target is two tiles in front of Pacman
    let dx = match pacman_dir {
        Direction::Left => -1,
        Direction::Right => 1,
        _ => 0,
    };
    let dy = match pacman_dir {
        Direction::Up => -1,
        Direction::Down => 1,
        _ => 0,
    };

    let ahead = (pacman_x + 2 * dx, pacman_y + 2 * dy);
    calculate_inky_vec(ahead, blinky_pos)
}

pub fn get_target_pinky(pacman_dir:Direction, pacman_pos: (i32, i32)) -> (i32, i32) {
    let dx = match pacman_dir {
        Direction::Left => -1,
        Direction::Right => 1,
        _ => 0,
    };
    let dy = match pacman_dir {
        Direction::Up => -1,
        Direction::Down => 1,
        _ => 0,
    };

    (pacman_pos.0 + 4 * dx, pacman_pos.0 + 4 * dy)
}

pub fn get_target_clyde(pacman_pos: (i32, i32), clyde_pos: (i32, i32)) -> (i32, i32) {
    // If Clyde is far from Pacman, chase Pacman
    if manhattan_distance(pacman_pos, clyde_pos) > 8. {
        pacman_pos
    } else {
        // Otherwise, scatter to the bottom left corner
        BOTTOM_LEFT_CORNER
    }
}

pub struct Ghost {
    pub name: String,
    pub pos: (i32, i32),
    pub direction: Direction,
    pub state: GhostState,
    pub scatter_pos: (i32, i32),
    pub color: [f32; 4], // RGBA color
}

impl Ghost {
    // Option 1: Accept a boxed state directly
    pub fn new(name: String, pos: (i32, i32), scatter_pos: (i32, i32), color: [f32; 4]) -> Self {
        Ghost { 
            name, 
            pos, 
            state: GhostState::Scatter,
            direction: Direction::Left, // Default direction
            scatter_pos,
            color
        }
    }
    
    pub fn move_around(&mut self, target: (i32, i32),grid: &Grid) {
        if let Some(new_pos) = self.move_to_target(target, grid) {
            self.pos = new_pos; // Move to the next node in the path
        }
    }
    
    pub fn get_pixels_x(&self) -> i32 {
        self.pos.0 * CELL_SIZE
    }
    
    pub fn get_pixels_y(&self) -> i32 {
        self.pos.1 * CELL_SIZE
    }

    fn determine_direction(&mut self, new_pos: (i32, i32)) -> Direction {
        match self.pos.0 - new_pos.0 {
            0 => match self.pos.1 - new_pos.1 {
                0 => Direction::Right, // Moving right
                1 => Direction::Down, // Moving down
                -1 => Direction::Up, // Moving up
                _ => Direction::Up, // Default to left if no movement
            }, // Moving down
            1 => Direction::Right, // Moving right
            -1 => Direction::Left, // Moving left
            _ => Direction::Right, 
        }
    }

    pub fn move_to_target(&mut self, target:(i32, i32), game_grid: &Grid) -> Option<(i32, i32)> {
        let (ghost_x, ghost_y) = self.pos;
        let up = (ghost_x, if ghost_y - 1 < 0 { GRID_HEIGHT - 1 } else { ghost_y - 1 });
        let down = (ghost_x, if ghost_y + 1 >= GRID_HEIGHT { 0 } else { ghost_y + 1 });
        let left = (if ghost_x - 1 < 0 { GRID_WIDTH - 1 } else { ghost_x - 1}, ghost_y);
        let right = (if ghost_x + 1 >= GRID_WIDTH { 0 } else { ghost_x + 1 }, ghost_y);

        let mut possible_moves = vec![
            up, // Up
            down, // Down
            left, // Left
            right, // Right
        ];

        let oposite_dir = self.direction.opposite();
        match oposite_dir {
            Direction::Up => {
                // If the ghost is moving up, it cannot move down
                possible_moves.retain(|&pos| pos != down);
            },
            Direction::Down => {
                // If the ghost is moving down, it cannot move up
                possible_moves.retain(|&pos| pos != up);
            },
            Direction::Left => {
                // If the ghost is moving left, it cannot move right
                possible_moves.retain(|&pos| pos != right);
            },
            Direction::Right => {
                // If the ghost is moving right, it cannot move left
                possible_moves.retain(|&pos| pos != left);
            },
        }
        
        possible_moves.iter().filter(|position| {
            // Check if the position is within bounds and walkable
            if let Some(tile) = game_grid.get_tile(**position) {
                tile.is_walkable_for_ghost()
            } else {
                false
            }
        }).map(|&pos| {
            // Calculate the distance to the player
            let distance = manhattan_distance(pos, target);
            
            (pos, distance, self.determine_direction(target) as i32)
        }).min_by(|a, b| {
            // First compare by distance
            match a.1.partial_cmp(&b.1) {
                Some(std::cmp::Ordering::Equal) => {
                    // If distances are equal, compare by direction priority
                    a.2.cmp(&b.2)
                },
                Some(order) => order,
                None => std::cmp::Ordering::Equal,
            }
        }).map(|(best_move, _, _)| {
            self.direction = self.determine_direction(best_move);
            best_move
        })
    }

}
