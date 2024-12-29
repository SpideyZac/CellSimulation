use rand::{thread_rng, Rng};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::cell::Cell;
use crate::config::*;
use crate::id::IdManager;

pub struct CellManager {
    cells: FxHashMap<u64, Cell>,
    food: FxHashMap<u64, (f32, f32, f32)>,
    cell_grid: Vec<FxHashSet<u64>>,
    food_grid: Vec<FxHashSet<u64>>,
    cell_id_manager: IdManager,
    food_id_manager: IdManager,
    _num_cells: usize,
}

impl CellManager {
    pub fn new() -> Self {
        Self {
            cells: FxHashMap::default(),
            food: FxHashMap::default(),
            cell_grid: vec![
                FxHashSet::default();
                (GAME_SIZE * GAME_SIZE) / (GRID_CELL_SIZE * GRID_CELL_SIZE)
            ],
            food_grid: vec![
                FxHashSet::default();
                (GAME_SIZE * GAME_SIZE) / (GRID_CELL_SIZE * GRID_CELL_SIZE)
            ],
            cell_id_manager: IdManager::new(),
            food_id_manager: IdManager::new(),
            _num_cells: GAME_SIZE / GRID_CELL_SIZE,
        }
    }

    pub fn init(&mut self) {
        let mut rng = thread_rng();

        for _ in 0..STARTING_CELLS {
            let id = self.cell_id_manager.get_id();
            let cell = Cell::new(id, FxHashMap::default());
            self.add_cell(cell);
        }

        for _ in 0..STARTING_FOOD {
            let x = rng.gen_range(0.0..GAME_SIZE as f32);
            let y = rng.gen_range(0.0..GAME_SIZE as f32);
            let food = DEFAULT_FOOD_VALUE;
            self.add_food(x, y, food);
        }
    }

    fn get_cell_grid_index(&self, x: f32, y: f32) -> usize {
        let x = (x / GRID_CELL_SIZE as f32).floor() as usize;
        let y = (y / GRID_CELL_SIZE as f32).floor() as usize;
        y * self._num_cells + x
    }

    fn add_cell(&mut self, cell: Cell) {
        let (x, y, id) = (cell.x, cell.y, cell.id);
        self.cells.insert(id, cell);
        let index = self.get_cell_grid_index(x, y);
        self.cell_grid[index].insert(id);
    }

    fn add_food(&mut self, x: f32, y: f32, food: f32) {
        let id = self.food_id_manager.get_id();
        self.food.insert(id, (x, y, food));
        let index = self.get_cell_grid_index(x, y);
        self.food_grid[index].insert(id);
    }

    fn remove_cell(&mut self, id: u64) {
        if let Some(cell) = self.cells.remove(&id) {
            let index = self.get_cell_grid_index(cell.x, cell.y);
            self.cell_grid[index].remove(&id);
            self.cell_id_manager.restore_id(id);
        }
    }

    fn remove_food(&mut self, id: u64) {
        if let Some(food) = self.food.remove(&id) {
            let index = self.get_cell_grid_index(food.0, food.1);
            self.food_grid[index].remove(&id);
            self.food_id_manager.restore_id(id);
        }
    }

    fn move_cell(&mut self, id: u64, prev_x: f32, prev_y: f32, x: f32, y: f32) {
        let prev_index = self.get_cell_grid_index(prev_x, prev_y);
        let index = self.get_cell_grid_index(x, y);

        if prev_index != index {
            self.cell_grid[prev_index].remove(&id);
            self.cell_grid[index].insert(id);
        }
    }

    fn get_neighbor_cells(&self, id: u64, x: f32, y: f32) -> Vec<u64> {
        let mut neighbor_cells = Vec::new();
        let index = self.get_cell_grid_index(x, y);

        let x_factor = (index % GRID_CELL_SIZE) as i64;
        let y_factor = (index / GRID_CELL_SIZE) as i64;

        for i in 0..3 {
            for j in 0..3 {
                let x = x_factor + i - 1;
                let y = y_factor + j - 1;

                if x >= 0 && x < GRID_CELL_SIZE as i64 && y >= 0 && y < GRID_CELL_SIZE as i64 {
                    let neighbor_index = (y * GRID_CELL_SIZE as i64 + x) as usize;
                    neighbor_cells.extend(self.cell_grid[neighbor_index].iter().copied());
                }
            }
        }

        neighbor_cells.retain(|&cell_id| cell_id != id);
        neighbor_cells
    }

    fn get_neighbor_food(&self, x: f32, y: f32) -> Vec<u64> {
        let mut neighbor_food = Vec::new();
        let index = self.get_cell_grid_index(x, y);

        let x_factor = (index % GRID_CELL_SIZE) as i64;
        let y_factor = (index / GRID_CELL_SIZE) as i64;

        for i in 0..3 {
            for j in 0..3 {
                let x = x_factor + i - 1;
                let y = y_factor + j - 1;

                if x >= 0 && x < GRID_CELL_SIZE as i64 && y >= 0 && y < GRID_CELL_SIZE as i64 {
                    let neighbor_index = (y * GRID_CELL_SIZE as i64 + x) as usize;
                    neighbor_food.extend(self.food_grid[neighbor_index].iter().copied());
                }
            }
        }

        neighbor_food
    }

    fn emit_forces(&mut self) {
        for (_, food) in self.food.iter() {
            let (x, y, food) = *food;
            let neighbors = self.get_neighbor_cells(u64::MAX, x, y);
            for neighbor in neighbors {
                if let Some(cell) = self.cells.get_mut(&neighbor) {
                    if (cell.x - x).powi(2) + (cell.y - y).powi(2) < FORCE_MAX_RANGE_SQ {
                        cell.add_forces(&[(FOOD_FORCE, food)], x, y);
                    }
                }
            }
        }

        for index in 0..self.cells.len() {
            let id = self.cells.keys().nth(index).unwrap();
            let cell = self.cells.get(&id).unwrap();
            let (id, x, y, emissions) = (cell.id, cell.x, cell.y, cell.get_emissions());
            let neighbors = self.get_neighbor_cells(id, x, y);
            for neighbor in neighbors {
                if let Some(neighbor) = self.cells.get_mut(&neighbor) {
                    if (neighbor.x - x).powi(2) + (neighbor.y - y).powi(2) < FORCE_MAX_RANGE_SQ {
                        neighbor.add_forces(&emissions, x, y);
                    }
                }
            }
        }
    }

    fn attempt_to_eat(&mut self, cell_id: u64) {
        let cell = self.cells.get(&cell_id).unwrap();
        let (x, y, size) = (cell.x, cell.y, cell.size);
        let neighbors = self.get_neighbor_food(x, y);
        for neighbor in neighbors {
            if let Some(food) = self.food.get(&neighbor) {
                let (food_x, food_y, food) = *food;
                if (x - food_x).powi(2) + (y - food_y).powi(2) <= size {
                    let cell = self.cells.get_mut(&cell_id).unwrap();
                    cell.add_food(food);
                    self.remove_food(neighbor);
                }
            }
        }
    }

    pub fn update(&mut self) {
        let mut rng = thread_rng();

        self.emit_forces();

        let mut index = 0;
        while index < self.cells.len() {
            let id = *self.cells.keys().nth(index).unwrap();
            let cell = self.cells.get_mut(&id).unwrap();
            let (prev_x, prev_y) = cell.update();
            let (x, y) = (cell.x, cell.y);
            self.move_cell(id, prev_x, prev_y, x, y);

            let cell = self.cells.get(&id).unwrap();
            if cell.is_dead() {
                self.add_food(cell.x, cell.y, DEFAULT_CELL_FOOD_VALUE);
                self.remove_cell(id);
                continue;
            }

            self.attempt_to_eat(id);

            let cell = self.cells.get_mut(&id).unwrap();
            if cell.can_replicate() {
                let id = self.cell_id_manager.get_id();
                let new_cell = cell.replicate(id);
                cell.reset();
                self.add_cell(new_cell);

                index += 1;
                continue;
            }

            cell.reset();
            index += 1;
        }

        for _ in 0..FOOD_ADDED_PER_FRAME {
            let x = rng.gen_range(0.0..GAME_SIZE as f32);
            let y = rng.gen_range(0.0..GAME_SIZE as f32);
            let food = DEFAULT_FOOD_VALUE;
            self.add_food(x, y, food);
        }
    }

    pub fn get_cells(&self) -> &FxHashMap<u64, Cell> {
        &self.cells
    }
}
