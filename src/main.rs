mod constants;
mod enums;
mod game;
mod ghost;
mod grid;
mod player;
mod utils; 
mod states;
mod pathfinding;

use game::Game;
use piston_window::*;

use constants::{
    GRID_WIDTH,
    GRID_HEIGHT,
    CELL_SIZE,
};

fn main() {

        // Create a Glutin window.
    let mut window: PistonWindow = WindowSettings::new("Pacman RS By Yasuke", [(GRID_WIDTH * CELL_SIZE) as f64, (GRID_HEIGHT * CELL_SIZE) as f64])
                                .exit_on_esc(true)
                                .vsync(true)
                                .build()
                                .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    // Create a new game and run it.
    let mut game = Game::new();
    
    while let Some(event) = window.next() {
        if let Some(args) = event.update_args() {
            game.update(args.dt);
        }

        // if let Some(input) = event.press_args() {
        //     game.handle_input(&input);
        // }

        // Handle render events
        window.draw_2d(&event, |context, graphics, _device| {
            game.render(context, graphics);
        });
    }
}
