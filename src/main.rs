use std::collections::{HashMap, HashSet};

const GRID_CELL_SIZE: f32 = 10.0; // This is also the max radius of the forces that a cell can emit
const GRID_SIZE: usize = 100;
const CELL_RADIUS: f32 = 1.0;
const CELL_START_FOOD: u16 = 100;

// move this later
fn calculate_dx_dy(
    cell_x: f32,
    cell_y: f32,
    force_x: f32,
    force_y: f32,
    force_magnitude: f32,
    cell_attraction: f32,
) -> (f32, f32) {
    // first we want to scale the force down to 0, 0 and scale the cell relative to that
    let cell_x = cell_x - force_x;
    let cell_y = cell_y - force_y;

    let dx = -cell_x;
    let dy = -cell_y;

    let distance_sq = dx * dx + dy * dy;

    // if the distance is too small, we don't want to divide by it
    if distance_sq < 1e-6 {
        return (0.0, 0.0);
    }

    // scale the force by its magnitude and the cell's attraction and then scale by the inverse square of the distance as that makes the force weaker the further away the cell is
    let scaled_force = force_magnitude * cell_attraction / distance_sq;

    let dx = dx * scaled_force;
    let dy = dy * scaled_force;

    (dx, dy)
}

struct Cell {
    id: usize,
    x: f32,
    y: f32,
    genes: Vec<isize>,
    attractions: HashMap<usize, f32>,
    emissions: HashMap<usize, f32>,
    food: u16,
    next_x: f32,
    next_y: f32,
    next_forces: Vec<(usize, f32)>,
}

impl Cell {
    fn new(id: usize, x: f32, y: f32, genes: Vec<isize>) -> Self {
        // todo auto creation of attractions and emissions based on genes
        // also when we add reproduction we need to handle the other DNA thingies
        Cell {
            id,
            x,
            y,
            genes,
            attractions: HashMap::new(),
            emissions: HashMap::new(),
            food: CELL_START_FOOD,
            next_x: x,
            next_y: y,
            next_forces: Vec::new(),
        }
    }
}

struct SpatialGrid {
    cells: Vec<HashSet<usize>>,
}

impl SpatialGrid {
    fn new() -> Self {
        SpatialGrid {
            cells: vec![HashSet::new(); GRID_SIZE * GRID_SIZE],
        }
    }

    fn add_cell(&mut self, cell: &Cell) {
        let x = (cell.x / GRID_CELL_SIZE) as usize;
        let y = (cell.y / GRID_CELL_SIZE) as usize;
        let index = x + y * GRID_SIZE;
        self.cells[index].insert(cell.id);
    }

    fn remove_cell(&mut self, cell: &Cell) {
        let x = (cell.x / GRID_CELL_SIZE) as usize;
        let y = (cell.y / GRID_CELL_SIZE) as usize;
        let index = x + y * GRID_SIZE;
        self.cells[index].remove(&cell.id);
    }

    fn move_cell(&mut self, cell: &mut Cell, new_x: f32, new_y: f32) {
        self.remove_cell(cell);
        cell.x = new_x;
        cell.y = new_y;
        let new_x = (new_x / GRID_CELL_SIZE) as usize;
        let new_y = (new_y / GRID_CELL_SIZE) as usize;
        let index = new_x + new_y * GRID_SIZE;
        self.cells[index].insert(cell.id);
    }

    fn get_neighbors(&self, cell: &Cell) -> Vec<usize> {
        let x = (cell.x / GRID_CELL_SIZE) as usize;
        let y = (cell.y / GRID_CELL_SIZE) as usize;
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < GRID_SIZE as isize && ny >= 0 && ny < GRID_SIZE as isize {
                    let index = nx as usize + ny as usize * GRID_SIZE;
                    for id in &self.cells[index] {
                        neighbors.push(*id);
                    }
                }
            }
        }
        neighbors
    }
}

struct CellManager {
    cells: Vec<Cell>,
    grid: SpatialGrid,
}

impl CellManager {
    fn new() -> Self {
        CellManager {
            cells: Vec::new(),
            grid: SpatialGrid::new(),
        }
    }

    fn add_cell(&mut self, x: f32, y: f32, genes: Vec<isize>) {
        let id = self.cells.len();
        let cell = Cell::new(id, x, y, genes);
        self.grid.add_cell(&cell);
        self.cells.push(cell);
    }

    fn remove_cell(&mut self, id: usize) {
        let cell = &self.cells[id];
        self.grid.remove_cell(cell);
        self.cells.remove(id);
    }

    fn move_cell(&mut self, id: usize, new_x: f32, new_y: f32) {
        let cell = &mut self.cells[id];
        self.grid.move_cell(cell, new_x, new_y);
    }

    fn get_neighbors(&self, id: usize) -> Vec<usize> {
        let cell = &self.cells[id];
        self.grid.get_neighbors(cell)
    }
}

fn main() {
    let mut cell_manager = CellManager::new();
    cell_manager.add_cell(0.0, 0.0, vec![1, 2, 3]);
    cell_manager.add_cell(10.0, 10.0, vec![4, 5, 6]);
    cell_manager.add_cell(20.0, 20.0, vec![7, 8, 9]);
    let neighbors = cell_manager.get_neighbors(0);
    println!("{:?}", neighbors);
    cell_manager.move_cell(0, 10.0, 10.0);
    let neighbors = cell_manager.get_neighbors(0);
    println!("{:?}", neighbors);
}
