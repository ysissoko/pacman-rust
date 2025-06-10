use crate::constants::{GRID_WIDTH, GRID_HEIGHT};

pub fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> f32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as f32
}

fn check_bounds(pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < GRID_WIDTH && pos.1 >= 0 && pos.1 < GRID_HEIGHT
}

pub fn find_neighbors(pos: (i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                // Skip the current position
                continue;
            }

            let new_pos = (pos.0 + i, pos.1 + j);
            if check_bounds(new_pos) {
                neighbors.push(new_pos);
            } else if new_pos.0 < 0 {
                // Wrap around horizontally
                neighbors.push((GRID_WIDTH - 1, new_pos.1));
            } else if new_pos.0 >= GRID_WIDTH {
                // Wrap around horizontally
                neighbors.push((0, new_pos.1));
            } else if new_pos.1 < 0 {
                // Wrap around vertically
                neighbors.push((new_pos.0, GRID_HEIGHT - 1));
            } else if new_pos.1 >= GRID_HEIGHT {
                // Wrap around vertically
                neighbors.push((new_pos.0, 0));
            }
        }
    }

    neighbors
}

pub fn get_speed_for_level(base_speed: f64, level: usize, min_speed: f64) -> f64 {
    let scale = 1.0 - (level as f64 * 0.03);
    (base_speed * scale).max(min_speed)
}
