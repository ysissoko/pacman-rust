mod constants;
mod enums;
mod game;
mod ghost;
mod grid;
mod player;
mod utils; 
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
    let mut window: PistonWindow = WindowSettings::new("Pacman RS By Yasuke", [(GRID_WIDTH * CELL_SIZE) as f64 + 100., (GRID_HEIGHT * CELL_SIZE) as f64])
                                .exit_on_esc(true)
                                .vsync(true)
                                .build()
                                .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = find_folder::Search::Parents(1)
            .for_folder("assets").unwrap();
    let glyphs = window.load_font(assets.join("ARCADE_N.TTF")).unwrap();
    // Create a new game and run it.
    let mut game = Game::new(glyphs);
    
    while let Some(event) = window.next() {
        if let Some(args) = event.update_args() {
            game.update(args.dt);
        }

        if let Some(input) = event.press_args() {
            game.handle_input(&input);
        }

        // Handle render events
        window.draw_2d(&event, |context, graphics, device| {
            game.render(context, graphics, device);
        });
    }
}
