use crate::pathfinding::{AStar};
use crate::states::{State};
use crate::constants::{CELL_SIZE};

// grid.rs
pub struct Ghost {
    pub name: String,
    pub pos: (i32, i32),
    state: Box<dyn State>,
    pathfinding: AStar,
}

impl Ghost {
    // Option 1: Accept a boxed state directly
    pub fn new(name: String, pos: (i32, i32), state: Box<dyn State>, pathfinding: AStar) -> Self {
        Ghost { 
            name, 
            pos, 
            state, 
            pathfinding 
        }
    }
    
    
    pub fn move_around(&mut self, target: (i32, i32)) {
        println!("{} is at position {:?}", self.name, self.pos);
        // Pass mutable reference to pathfinding
        self.state.move_around(&self.name, &mut self.pos, target, &mut self.pathfinding);
    }
    
    pub fn get_pixels_x(&self) -> i32 {
        self.pos.0 * CELL_SIZE
    }
    
    pub fn get_pixels_y(&self) -> i32 {
        self.pos.1 * CELL_SIZE
    }
    
    // Helper method to change state
    pub fn set_state(&mut self, new_state: Box<dyn State>) {
        self.state = new_state;
    }

}
