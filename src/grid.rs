use std::{fs::File, io::{BufRead, BufReader}};


use ndarray::Array2;

#[derive(Debug, PartialEq, Clone)]
pub enum TileType {
    Wall,
    Pellet,
    PowerPellet,
    Floor,
    GhostGate,
}

#[derive(Clone)]
pub struct Tile {
    pub pos: (i32, i32),
    size: (i32, i32),
    pub type_: TileType
}

impl Tile {
    pub fn new(pos: (i32, i32), size: (i32, i32), type_: TileType) -> Self {
        Tile { pos, size, type_ }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.pos
    }

    pub fn get_size(&self) -> (i32, i32) {
        self.size
    }

    pub fn get_pixels_x(&self) -> i32 {
        self.pos.0 * self.size.0
    }

    pub fn get_pixels_y(&self) -> i32 {
        self.pos.1 * self.size.1
    }

    pub fn get_type(&self) -> &TileType {
        &self.type_
    }

    pub fn is_walkable_for_pacman(&self) -> bool {
        match self.type_ {
            TileType::Wall => false,
            TileType::GhostGate => false,
            _ => true,
        }
    }

    pub fn is_walkable_for_ghost(&self) -> bool {
        match self.type_ {
            TileType::Wall => false,
            _ => true,
        }
    }
}

 fn char_to_tile_type(c: char) -> TileType {
    match c {
        '#' => TileType::Wall,
        '.' => TileType::Pellet,
        ' ' => TileType::Floor,
        'o' => TileType::PowerPellet,
        '=' => TileType::GhostGate, // 👈 Gate from the map
        _   => TileType::Floor,
    }
}

pub struct Grid {
    tiles: Array2<Tile>,
    num_cols: i32,
    num_rows: i32,
}

fn load_grid_from_file(path: &str, tile_size: i32, num_cols: i32, num_rows: i32) -> Array2<Tile> {
    let file = File::open(path).expect("Could not open map file");
    let reader = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();
        
    Array2::from_shape_fn(
        (num_cols as usize, num_rows as usize),
        |(col, row)| Tile::new((col as i32, row as i32), (tile_size, tile_size), char_to_tile_type(grid[row][col])),
    )  
}

impl Grid {
    pub fn new(path: &str, tile_size: i32, num_cols: i32, num_rows: i32) -> Self {
        let tiles = load_grid_from_file(path, tile_size, num_cols, num_rows);
        Grid { tiles, num_cols, num_rows }
    }

    pub fn get_tiles(&self) -> &Array2<Tile> {
        &self.tiles
    }

    pub fn get_tile(&self, pos: (i32, i32)) -> Option<&Tile> {
        self.tiles.get((pos.0 as usize, pos.1 as usize))
    }
    // Getters for pixel width and height
    // These are the pixel dimensions of the grid
    pub fn get_dim_width(&self) -> f64 {
        (self.num_cols * self.tiles[(0, 0)].size.0) as f64
    }

    pub fn get_dim_height(&self) -> f64 {
        (self.num_rows * self.tiles[(0, 0)].size.1) as f64
    }
}
