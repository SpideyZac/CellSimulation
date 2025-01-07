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
    _cells_per_axis: usize,
    _relation_matrix: Vec<Vec<usize>>,
}

impl CellManager {
    pub fn new() -> Self {
        let _cells_per_axis = GAME_SIZE / GRID_CELL_SIZE;
        let mut _relation_matrix = vec![vec![]; _cells_per_axis * _cells_per_axis];

        for y in 0.._cells_per_axis {
            for x in 0.._cells_per_axis {
                let index = y * _cells_per_axis + x;
                let mut neighbors = vec![];
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0
                            && nx < _cells_per_axis as i32
                            && ny >= 0
                            && ny < _cells_per_axis as i32
                        {
                            neighbors.push((ny as usize) * _cells_per_axis + nx as usize);
                        }
                    }
                }

                _relation_matrix[index] = neighbors;
            }
        }

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
            _cells_per_axis,
            _relation_matrix,
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

    pub fn init_with_starting(&mut self, cells: Vec<Cell>, food: Vec<(f32, f32, f32)>) {
        for mut cell in cells {
            let id = self.cell_id_manager.get_id();
            cell.id = id;
            self.add_cell(cell);
        }

        for (x, y, food) in food {
            self.add_food(x, y, food);
        }
    }

    fn get_cell_grid_index(&self, x: f32, y: f32) -> usize {
        let x = (x / GRID_CELL_SIZE as f32).floor() as usize;
        let y = (y / GRID_CELL_SIZE as f32).floor() as usize;
        y * self._cells_per_axis + x
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

    fn emit_forces(&mut self, cell_keys: &Vec<u64>) {
        for (_, food) in self.food.iter() {
            let (x, y, food) = *food;
            let index = self.get_cell_grid_index(x, y);
            let neighbors = &self._relation_matrix[index];
            for neighbor in neighbors {
                let cells = &self.cell_grid[*neighbor];
                for cell_id in cells.iter() {
                    if let Some(cell) = self.cells.get_mut(cell_id) {
                        if (cell.x - x).powi(2) + (cell.y - y).powi(2) < FORCE_MAX_RANGE_SQ {
                            cell.add_forces(&[(FOOD_FORCE, food)], x, y);
                        }
                    }
                }
            }
        }

        for index in 0..self.cells.len() {
            let id = cell_keys[index];
            let cell = self.cells.get(&id).unwrap();
            let (id, x, y, emissions) = (cell.id, cell.x, cell.y, cell.get_emissions());
            let index = self.get_cell_grid_index(x, y);
            let neighbors = &self._relation_matrix[index];
            for neighbor in neighbors {
                let cells = &self.cell_grid[*neighbor];
                for cell_id in cells.iter() {
                    if *cell_id == id {
                        continue;
                    }
                    if let Some(cell) = self.cells.get_mut(cell_id) {
                        if (cell.x - x).powi(2) + (cell.y - y).powi(2) < FORCE_MAX_RANGE_SQ {
                            cell.add_forces(&emissions, x, y);
                        }
                    }
                }
            }
        }
    }

    fn attempt_to_eat(&mut self, cell_id: u64) {
        let cell = self.cells.get(&cell_id).unwrap();
        let (x, y, size) = (cell.x, cell.y, cell.size);
        let index = self.get_cell_grid_index(x, y);
        let neighbors = self._relation_matrix[index].clone();
        for neighbor in neighbors {
            let foods = self.food_grid[neighbor].clone();
            for food_id in foods.iter() {
                if let Some(food) = self.food.get(food_id) {
                    let (food_x, food_y, food) = *food;
                    if (x - food_x).powi(2) + (y - food_y).powi(2) <= size {
                        let cell = self.cells.get_mut(&cell_id).unwrap();
                        cell.add_food(food);
                        let food_id = *food_id;
                        self.remove_food(food_id);
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        let mut rng = thread_rng();
        let cell_keys: Vec<u64> = self.cells.keys().copied().collect();
        let mut cells_length = cell_keys.len();

        self.emit_forces(&cell_keys);

        for id in cell_keys.iter() {
            let cell = self.cells.get_mut(&id).unwrap();
            let (prev_x, prev_y) = cell.update();
            let (x, y) = (cell.x, cell.y);
            self.move_cell(*id, prev_x, prev_y, x, y);

            let cell = self.cells.get(&id).unwrap();
            if cell.is_dead() {
                if self.food.len() < MAX_FOOD {
                    self.add_food(cell.x, cell.y, DEFAULT_CELL_FOOD_VALUE);
                }
                self.remove_cell(*id);
                cells_length -= 1;
                continue;
            }

            self.attempt_to_eat(*id);

            let cell = self.cells.get_mut(&id).unwrap();
            if cell.can_replicate() && cells_length < MAX_CELLS {
                let id = self.cell_id_manager.get_id();
                let new_cell = cell.replicate(id);
                cell.reset();
                self.add_cell(new_cell);
                cells_length += 1;
                continue;
            }

            cell.reset();
        }

        if self.food.len() < MAX_FOOD {
            for _ in 0..FOOD_ADDED_PER_FRAME {
                let x = rng.gen_range(0.0..GAME_SIZE as f32);
                let y = rng.gen_range(0.0..GAME_SIZE as f32);
                let food = DEFAULT_FOOD_VALUE;
                self.add_food(x, y, food);
            }
        }
    }

    pub fn get_cells(&self) -> &FxHashMap<u64, Cell> {
        &self.cells
    }

    pub fn get_cells_cloned(&self) -> FxHashMap<u64, Cell> {
        self.cells.clone()
    }

    pub fn get_food_cloned(&self) -> FxHashMap<u64, (f32, f32, f32)> {
        self.food.clone()
    }
}
