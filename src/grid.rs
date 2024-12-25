use rustc_hash::FxHashSet;

use crate::config::*;

pub struct SpatialGrid {
    pub grid: Vec<Vec<FxHashSet<usize>>>,
    pub food_grid: Vec<Vec<FxHashSet<usize>>>,
}

impl SpatialGrid {
    pub fn new() -> Self {
        let mut grid = Vec::new();
        let mut food_grid = Vec::new();

        for _ in 0..(GAME_SIZE / GAME_SQUARE_SIZE) {
            let mut row = Vec::new();
            let mut food_row = Vec::new();

            for _ in 0..(GAME_SIZE / GAME_SQUARE_SIZE) {
                row.push(FxHashSet::default());
                food_row.push(FxHashSet::default());
            }

            grid.push(row);
            food_grid.push(food_row);
        }

        Self { grid, food_grid }
    }

    fn calculate_grid_position(x: f32, y: f32) -> (usize, usize) {
        let x = x as usize / GAME_SQUARE_SIZE as usize;
        let y = y as usize / GAME_SQUARE_SIZE as usize;

        (x, y)
    }

    pub fn add_cell(&mut self, x: f32, y: f32, id: usize) {
        let (x, y) = Self::calculate_grid_position(x, y);

        self.grid[x][y].insert(id);
    }

    pub fn remove_cell(&mut self, x: f32, y: f32, id: usize) {
        let (x, y) = Self::calculate_grid_position(x, y);

        self.grid[x][y].retain(|&i| i != id);
    }

    pub fn move_cell(&mut self, old_x: f32, old_y: f32, new_x: f32, new_y: f32, id: usize) {
        self.remove_cell(old_x, old_y, id);
        self.add_cell(new_x, new_y, id);
    }

    pub fn add_food(&mut self, x: f32, y: f32, id: usize) {
        let (x, y) = Self::calculate_grid_position(x, y);

        self.food_grid[x][y].insert(id);
    }

    pub fn remove_food(&mut self, x: f32, y: f32, id: usize) {
        let (x, y) = Self::calculate_grid_position(x, y);

        self.food_grid[x][y].retain(|&i| i != id);
    }

    fn get_neighbors_indices(x: usize, y: usize, grid_size: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let new_x = x as isize + i;
                let new_y = y as isize + j;

                if new_x >= 0
                    && new_x < grid_size as isize
                    && new_y >= 0
                    && new_y < grid_size as isize
                {
                    neighbors.push((new_x as usize, new_y as usize));
                }
            }
        }

        neighbors
    }

    pub fn get_cell_neighbors(&self, x: f32, y: f32, id: usize) -> Vec<usize> {
        let (x, y) = Self::calculate_grid_position(x, y);

        let mut neighbors = Vec::new();
        for (nx, ny) in Self::get_neighbors_indices(x, y, self.grid.len()) {
            neighbors.extend(
                self.grid[nx][ny]
                    .iter()
                    .filter(|&&neighbor_id| neighbor_id != id),
            );
        }
        neighbors
    }

    pub fn get_food_neighbors(&self, x: f32, y: f32) -> Vec<usize> {
        let (x, y) = Self::calculate_grid_position(x, y);

        let mut neighbors = Vec::new();
        for (nx, ny) in Self::get_neighbors_indices(x, y, self.food_grid.len()) {
            neighbors.extend(self.food_grid[nx][ny].iter());
        }
        neighbors
    }
}
