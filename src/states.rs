use std::path;

use crate::pathfinding::AStar;
use crate::ghost::Ghost;

pub trait State {
    fn move_around(&self, ghost_name: &str, ghost_pos: &mut (i32, i32), player_pos: (i32, i32), pathfinding: &mut AStar);
}

pub struct ChaseState;

impl ChaseState {
    pub fn new() -> Self {
        ChaseState {}
    }
}

impl State for ChaseState {
    fn move_around(&self, ghost_name: &str, ghost_pos: &mut (i32, i32), player_pos: (i32, i32), pathfinding: &mut AStar) {
        println!("{} is chasing the player!", ghost_name);
        if let Some(path) = pathfinding.find_path(*ghost_pos, player_pos) {
            println!("{} found a path to the player: {:?}", ghost_name, path);
            *ghost_pos = path[1]; // Update ghost position to the last node in the path
        } else {
            println!("{} could not find a path to the player.", ghost_name);
        }
    }
}

pub struct ScatterState;

impl ScatterState {
    pub fn new() -> Self {
        ScatterState {}
    }
}

impl State for ScatterState {
    fn move_around(&self, ghost_name: &str, ghost_pos: &mut (i32, i32), player_pos: (i32, i32), pathfinding: &mut AStar) {
        println!("{} is scattering the player!", ghost_name);
    }
}

pub struct FrightenedState;

impl FrightenedState {
    pub fn new() -> Self {
        FrightenedState {}
    }
}

impl State for FrightenedState { 
    fn move_around(&self, ghost_name: &str, ghost_pos: &mut(i32, i32), player_pos: (i32, i32), pathfinding: &mut AStar) {
        println!("{} is frightened and running away!", ghost_name);
    }
}

pub struct EatenState;

impl EatenState {
    pub fn new() -> Self {
        EatenState {}
    }
}

impl State for EatenState {
    fn move_around(&self, ghost_name: &str, ghost_pos: &mut(i32, i32), player_pos: (i32, i32), pathfinding: &mut AStar) {
        println!("{} is eaten and out of the game!", ghost_name);
    }
}
