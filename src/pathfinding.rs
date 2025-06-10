use std::collections::VecDeque;

use crate::utils::{manhattan_distance, find_neighbors};
use ndarray::Array2;
use crate::constants::{GRID_WIDTH, GRID_HEIGHT};
struct Node {
    // Define the structure of your node here
    g: f32,
    h: f32,
    f: f32,
    pos: (i32, i32),
    neighbors: Vec<(i32, i32)>,
}

impl Node {
    pub fn new(pos: (i32, i32)) -> Self {
        Node {
            g: 0.0,
            h: 0.0,
            f: 0.0,
            pos,
            neighbors: find_neighbors(pos),
        }
    }
}

pub struct AStar {
    grid: Array2<Node>,
    open_list: Vec<(i32, i32)>,
    closed_list: Vec<(i32, i32)>,
    current_goal: Option<(i32, i32)>,  // Current target position
}

impl AStar {
    pub fn new() -> Self {
        let grid: Array2<Node> = Array2::from_shape_fn(
            (GRID_WIDTH as usize, GRID_HEIGHT as usize),
            |(col, row)| {
                Node::new((col as i32, row as i32))
            },
        );

        AStar {
            open_list: Vec::new(),
            closed_list: Vec::new(),
            grid,
            current_goal: None
        }
    }
    
    pub fn set_goal(&mut self, goal: (i32, i32)) {
        self.current_goal = Some(goal);
        
        // Update heuristics for all nodes in the open list
        let snap_open_positions: Vec<(i32, i32)> = self.open_list.clone();
        for pos in snap_open_positions.iter() {
            self.grid.iter_mut().find(|node| {
                node.pos == *pos
            }).map(|node: &mut Node| {
                if let Some(current_goal) = self.current_goal {
                    node.h = manhattan_distance(node.pos, current_goal);
                    node.f = node.g + node.h; // Update f score
                } 
            });
        }
    }

    fn get_node_mut(&mut self, pos: (i32, i32)) -> Option<&mut Node> {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < GRID_WIDTH && pos.1 < GRID_HEIGHT {
            Some(&mut self.grid[(pos.0 as usize, pos.1 as usize)])
        } else {
            None
        }
    }

    fn get_node(&self, pos: (i32, i32)) -> Option<&Node> {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < GRID_WIDTH && pos.1 < GRID_HEIGHT {
            Some(&self.grid[(pos.0 as usize, pos.1 as usize)])
        } else {
            None
        }
    }

    pub fn find_path(&mut self, start: (i32, i32), initial_goal: (i32, i32)) -> Option<Vec<(i32, i32)>> {
        // Reset the pathfinding state
        self.reset_path();
        // Initialize the start node
        self.open_list.push(start);
        // Set the initial goal
        self.set_goal(initial_goal);

        // Keep a separate data structure for parent tracking
        let mut came_from: std::collections::HashMap<(i32, i32), (i32, i32)> = std::collections::HashMap::new();
        
        while !self.open_list.is_empty() {
            let snapshot_open_list = self.open_list.clone();
            // Find the node with lowest f score
            let (min_index, _) = snapshot_open_list
                .iter()
                .enumerate()
                .min_by(|&(_, &a), &(_, &b)| {
                    let a_node = self.get_node(a).unwrap();
                    let b_node = self.get_node(b).unwrap();
                    a_node.f.partial_cmp(&b_node.f).unwrap()
                })
                .unwrap();

            let current = self.open_list.remove(min_index);
            // Check if we reached the goal (which may have changed)
            let goal: (i32, i32) = self.current_goal.unwrap();
            // println!("Current position: {:?}, Goal: {:?}", current, goal);
            if current == goal {
                // println!("Path found from {:?} to {:?}", start, goal);
                // Reconstruct the path
                return Some(reconstruct_path(&came_from, start, goal));
            }
            
            // Add to closed list
            self.closed_list.push(current);
            
            // Get neighbors one at a time and process them
            let neighbor_positions = {
                let current_node = self.get_node(current).unwrap();
                current_node.neighbors.clone()
            };
            
            for neighbor_pos in neighbor_positions {
                // Skip if in closed list
                if self.closed_list.contains(&neighbor_pos) {
                    continue;
                }
                
                // Calculate the tentative g score
                let tentative_g = {
                    let current_node = self.get_node(current).unwrap();
                    current_node.g + 1.0 // Using uniform cost for simplicity
                };
                
                let is_in_open_list = self.open_list.contains(&neighbor_pos);
                let is_better = {
                    if let Some(neighbor_node) = self.get_node(neighbor_pos) {
                        !is_in_open_list || tentative_g < neighbor_node.g
                    } 
                    else {
                        false
                    }
                };
                
                if is_better {
                    // Update the neighbor with the current goal
                    let goal = self.current_goal.unwrap();
                    {
                        let neighbor_node: &mut Node = self.get_node_mut(neighbor_pos).unwrap();
                        neighbor_node.g = tentative_g;
                        neighbor_node.h = manhattan_distance(neighbor_pos, goal);
                        neighbor_node.f = neighbor_node.g + neighbor_node.h;
                    }
                    
                    // Record where this node came from for path reconstruction
                    came_from.insert(neighbor_pos, current);
                    
                    // Add to open list if not there
                    if !self.open_list.contains(&neighbor_pos) {
                        // println!("Adding neighbor {:?} to open list with f: {}", neighbor_pos, self.get_node(neighbor_pos).unwrap().f);
                        self.open_list.push(neighbor_pos);
                    }
                }
            }
            // println!("open list length: {}, closed list length: {}", self.open_list.len(), self.closed_list.len());
        }
        // println!("No path found from {:?} to {:?}", start, initial_goal);
        None // No path found
    }

    fn reset_path(&mut self) {
        self.open_list.clear();
        self.closed_list.clear();
    }
}

// Helper function to reconstruct path
fn reconstruct_path(
    came_from: &std::collections::HashMap<(i32, i32), (i32, i32)>, 
    start: (i32, i32), 
    goal: (i32, i32)
) -> Vec<(i32, i32)> {
    let mut path = Vec::new();
    let mut current = goal;
    
    while current != start {
        path.push(current);
        current = *came_from.get(&current).unwrap();
    }
    path.push(start);
    path.reverse();
    path
}
