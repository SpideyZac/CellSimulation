use rand::{thread_rng, Rng};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::config::*;

#[derive(Debug, Clone)]
pub struct DNA(Vec<f32>);

impl DNA {
    fn new_random(max_force: u32) -> Self {
        let mut rng = thread_rng();
        let dna_length_codons = rng.gen_range(1..=10);
        let mut dna = Vec::with_capacity(dna_length_codons * 3);

        for _ in 0..dna_length_codons {
            let base1 = rng.gen_range(0..=1);
            let mut base2 = rng.gen_range(0..=max_force);

            while base1 == 1 && base2 == FOOD_FORCE as u32 {
                base2 = rng.gen_range(0..=max_force);
            }

            let base3 = if base1 == 0 {
                rng.gen_range(-1.0..=1.0)
            } else {
                rng.gen_range(0.1..=1.0)
            };

            dna.extend_from_slice(&[base1 as f32, base2 as f32, base3]);
        }

        DNA(dna)
    }

    fn mutate(&self, activated_genes: &[u32]) -> Self {
        let mut rng = thread_rng();
        let mut new_dna = self.0.clone();
        let mut gene_rates = FxHashMap::default();
        let mut gene_rate = DEFAULT_MUTATION_RATE;
        let mut primary_rate = PRIMARY_DEFAULT_MUTATION_RATE;
        let mut secondary_rate = SECONDARY_DEFAULT_MUTATION_RATE;
        let mut delete_rate = DELETE_CODON_DEFAULT_MUTATION_RATE;
        let mut add_rate = ADD_CODON_DEFAULT_MUTATION_RATE;

        for (chunk_index, chunk) in self.0.chunks(3).enumerate() {
            if !activated_genes.contains(&(chunk_index as u32)) {
                continue;
            }

            match chunk[0] as u16 {
                3 => {
                    gene_rates.insert(chunk[1] as u32, chunk[2]);
                }
                4 => gene_rate = chunk[2],
                5 => primary_rate = chunk[2],
                6 => secondary_rate = chunk[2],
                7 => delete_rate = chunk[2],
                8 => add_rate = chunk[2],
                _ => {}
            }
        }

        for (i, chunk) in new_dna.chunks_mut(3).enumerate() {
            let mut_rate = *gene_rates.get(&(i as u32)).unwrap_or(&gene_rate);

            if rng.gen_range(0.0..=1.0) < mut_rate {
                let r = rng.gen_range(0.0..=1.0);

                if r < primary_rate {
                    chunk[0] = rng.gen_range(0..=9) as f32;
                } else if r < secondary_rate {
                    chunk[1] += rng.gen_range(-1.0..=1.0);
                } else {
                    chunk[2] += rng.gen_range(-1.0..=1.0);
                }
            }
        }

        self.handle_frameshift(&mut new_dna, delete_rate, add_rate);
        self.fix_broken_codons(&mut new_dna);

        DNA(new_dna)
    }

    fn handle_frameshift(&self, dna: &mut Vec<f32>, delete_rate: f32, add_rate: f32) {
        let mut rng = thread_rng();
        let r = rng.gen_range(0.0..=1.0);

        if r < delete_rate {
            let index = rng.gen_range(0..=(dna.len() / 3 - 1)) * 3;

            dna.drain(index..index + 3);
        } else if r < add_rate {
            let index = rng.gen_range(0..=(dna.len() / 3)) * 3;
            let new_codon = [
                rng.gen_range(0..=9) as f32,
                rng.gen_range(-10.0..=10.0),
                rng.gen_range(-1.0..=1.0),
            ];

            dna.splice(index..index, new_codon.iter().copied());
        }
    }

    fn fix_broken_codons(&self, dna: &mut Vec<f32>) {
        let mut rng = thread_rng();

        for chunk in dna.chunks_mut(3) {
            match chunk[0] as u16 {
                1 => {
                    while chunk[1] as u16 == FOOD_FORCE as u16 {
                        chunk[1] += rng.gen_range(-1.0..=1.0);
                    }

                    if chunk[1] as u16 == TOXIN_FORCE as u16 {
                        chunk[2] = chunk[2].clamp(0.1, 2.0);
                    }

                    chunk[2] = chunk[2].max(0.0);
                }
                2..=8 => chunk[2] = chunk[2].max(0.0),
                9 => chunk[2] = chunk[2].max(DEFAULT_FOOD_REQUIRED_TO_REPLICATE * 0.5),
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    x: f32,
    y: f32,
    prev_x: f32,
    prev_y: f32,
    next_forces: FxHashMap<u16, f32>,
    dna: DNA,
    attractions: FxHashMap<u16, f32>,
    emissions: Vec<(u16, f32)>,
    food: f32,
    activated_genes: Vec<u32>,
    food_to_replicate: f32,
}

impl Cell {
    pub fn new(x: f32, y: f32, dna: DNA, next_forces: FxHashMap<u16, f32>) -> Self {
        let (attractions, emissions, activated_genes, food_to_replicate) =
            Self::process_dna(&dna, &next_forces);

        Self {
            x,
            y,
            prev_x: x,
            prev_y: y,
            next_forces: FxHashMap::default(),
            dna,
            attractions,
            emissions,
            food: STARTING_FOOD_CELL,
            activated_genes,
            food_to_replicate,
        }
    }

    pub fn new_random(max_force: u32) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..GAME_SIZE as f32);
        let y = rng.gen_range(0.0..GAME_SIZE as f32);
        let dna = DNA::new_random(max_force);

        Self::new(x, y, dna, FxHashMap::default())
    }

    fn process_dna(
        dna: &DNA,
        initial_forces: &FxHashMap<u16, f32>,
    ) -> (FxHashMap<u16, f32>, Vec<(u16, f32)>, Vec<u32>, f32) {
        let mut attractions = FxHashMap::default();
        let mut emissions = Vec::new();
        let mut activated_genes = (0..dna.0.len() / 3).map(|i| i as u32).collect::<Vec<_>>();
        let mut food_to_replicate = DEFAULT_FOOD_REQUIRED_TO_REPLICATE;

        for chunk in dna.0.chunks(3) {
            if chunk[0] as u16 == 2 {
                let base2 = chunk[1] as u16;

                if base2 < (dna.0.len() / 3) as u16
                    && *initial_forces.get(&base2).unwrap_or(&f32::NEG_INFINITY) < chunk[2]
                {
                    activated_genes.retain(|&x| x != base2 as u32);
                }
            } else if chunk[0] as u16 == 9 {
                food_to_replicate = chunk[2];
            }
        }

        for (i, chunk) in dna.0.chunks(3).enumerate() {
            if !activated_genes.contains(&(i as u32)) {
                continue;
            }

            match chunk[0] as u16 {
                0 => {
                    attractions
                        .entry(chunk[1] as u16)
                        .and_modify(|e| *e += chunk[2])
                        .or_insert(chunk[2]);
                }
                1 => {
                    if let Some(pos) = emissions.iter().position(|&(id, _)| id == chunk[1] as u16) {
                        emissions[pos].1 += chunk[2];
                    } else {
                        emissions.push((chunk[1] as u16, chunk[2]));
                    }
                }
                _ => {}
            }
        }

        (attractions, emissions, activated_genes, food_to_replicate)
    }

    pub fn replicate(&self) -> Self {
        let new_dna = self.dna.mutate(&self.activated_genes);

        Self::new(self.x, self.y, new_dna, self.next_forces.clone())
    }

    pub fn add_force(&mut self, force_id: u16, force: f32, force_x: f32, force_y: f32) {
        self.next_forces
            .entry(force_id)
            .and_modify(|e| *e += force)
            .or_insert(force);

        let x = self.x - force_x;
        let y = self.y - force_y;

        let dx = -x;
        let dy = -y;

        let distance_sq = dx * dx + dy * dy;
        if distance_sq < 0.1 {
            return;
        }

        let attraction = self.attractions.get(&force_id).unwrap_or(&0.0);
        let scaled_force = force * attraction / distance_sq;

        self.prev_x += scaled_force * dx;
        self.prev_y += scaled_force * dy;
    }

    pub fn update_pos(&mut self) {
        self.x = self.prev_x;
        self.y = self.prev_y;
        self.next_forces.clear();
    }

    fn calculate_food_used(&self) -> f32 {
        let mut food_used = 0.0;

        let total_changed = (self.x - self.prev_x).abs() + (self.y - self.prev_y).abs();
        food_used += total_changed * FOOD_USED_PER_UNIT_MOVED;

        for (type_, emission) in &self.emissions {
            if *type_ == TOXIN_FORCE as u16 {
                food_used += emission * FOOD_USED_PER_UNIT_TOXIN_EMITTED;
            } else {
                food_used += emission * FOOD_USED_PER_UNIT_EMITTED;
            }
        }

        food_used
    }

    fn update_food(&mut self) {
        let food_used = self.calculate_food_used();

        self.food -= food_used;
    }

    pub fn update(&mut self) {
        self.update_pos();
        self.update_food();
    }

    pub fn is_dead(&self) -> bool {
        self.food <= 0.0
    }

    pub fn can_replicate(&self) -> bool {
        self.food >= self.food_to_replicate
    }

    pub fn consume_replication_food(&mut self) {
        self.food -= self.food_to_replicate;
    }

    pub fn eat_food(&mut self, food: f32) {
        self.food += food;
    }

    pub fn remove_food(&mut self, food: f32) {
        self.food -= food;
    }

    pub fn get_emissions(&self) -> &Vec<(u16, f32)> {
        &self.emissions
    }
}

pub struct CellManager {
    cells: FxHashMap<usize, Cell>,
    food: FxHashMap<usize, (f32, f32, f32)>,
    cells_grid: Vec<FxHashSet<usize>>,
    food_grid: Vec<FxHashSet<usize>>,
    next_cell_id: usize,
    next_food_id: usize,
    _num_cells: usize,
}

impl CellManager {
    pub fn new() -> Self {
        let mut manager = Self {
            cells: FxHashMap::default(),
            food: FxHashMap::default(),
            cells_grid: vec![
                FxHashSet::default();
                (GAME_SIZE * GAME_SIZE / (GAME_SQUARE_SIZE * GAME_SQUARE_SIZE))
                    as usize
            ],
            food_grid: vec![
                FxHashSet::default();
                (GAME_SIZE * GAME_SIZE / (GAME_SQUARE_SIZE * GAME_SQUARE_SIZE)) as usize
            ],
            next_cell_id: 0,
            next_food_id: 0,
            _num_cells: (GAME_SIZE / GAME_SQUARE_SIZE) as usize,
        };

        for _ in 0..STARTING_CELLS {
            manager.create_random_cell();
        }

        for _ in 0..STARTING_FOOD_SPAWNED {
            manager.create_random_food();
        }

        manager
    }

    fn create_random_cell(&mut self) {
        let cell = Cell::new_random(10);
        self.add_cell(cell);
    }

    fn create_random_food(&mut self) {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..GAME_SIZE as f32);
        let y = rng.gen_range(0.0..GAME_SIZE as f32);
        let food = DEFAULT_FOOD_VALUE_PER_FOOD;
        self.add_food(x, y, food);
    }

    fn get_grid_index(&self, x: f32, y: f32) -> usize {
        (y / GAME_SQUARE_SIZE as f32).floor() as usize * self._num_cells as usize
            + (x / GAME_SQUARE_SIZE as f32).floor() as usize
    }

    fn add_cell(&mut self, cell: Cell) {
        let index = self.get_grid_index(cell.x, cell.y);
        self.cells_grid[index].insert(self.next_cell_id);
        self.cells.insert(self.next_cell_id, cell);
        self.next_cell_id += 1;
    }

    fn add_food(&mut self, x: f32, y: f32, food: f32) {
        let index = self.get_grid_index(x, y);
        self.food_grid[index].insert(self.next_food_id);
        self.food.insert(self.next_food_id, (x, y, food));
        self.next_food_id += 1;
    }

    fn remove_cell(&mut self, id: usize) {
        let cell = self.cells.remove(&id).unwrap();
        let index = self.get_grid_index(cell.x, cell.y);
        self.cells_grid[index].remove(&id);
    }

    fn remove_food(&mut self, id: usize) {
        let (x, y, _) = self.food.remove(&id).unwrap();
        let index = self.get_grid_index(x, y);

        self.food_grid[index].remove(&id);
    }

    fn move_cell(&mut self, id: usize, x: f32, y: f32, prev_x: f32, prev_y: f32) {
        let prev_index = self.get_grid_index(prev_x, prev_y);
        let new_index = self.get_grid_index(x, y);

        if prev_index != new_index {
            self.cells_grid[prev_index].remove(&id);
            self.cells_grid[new_index].insert(id);
        }
    }

    fn get_cell_neighbors(&self, x: f32, y: f32, id: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let index = self.get_grid_index(x, y);

        for i in 0..3 {
            for j in 0..3 {
                let new_index = index + i * self._num_cells + j;
                if new_index >= self.cells_grid.len() {
                    continue;
                }
                neighbors.extend(self.cells_grid[new_index].iter());
            }
        }

        neighbors.retain(|&x| x != id);
        neighbors
    }

    fn get_food_neighbors(&self, x: f32, y: f32) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let index = self.get_grid_index(x, y);

        for i in 0..3 {
            for j in 0..3 {
                let new_index = index + i * self._num_cells + j;
                if new_index >= self.food_grid.len() {
                    continue;
                }
                neighbors.extend(self.food_grid[new_index].iter());
            }
        }

        neighbors
    }

    fn emit_forces(&mut self) {
        for (_, (x, y, food)) in self.food.iter() {
            let neighbors = self.get_cell_neighbors(*x, *y, usize::MAX);

            for neighbor_id in neighbors {
                let neighbor = self.cells.get_mut(&neighbor_id).unwrap();
                neighbor.add_force(FOOD_FORCE as u16, *food, *x, *y);
            }
        }

        let indexes = self.cells.keys().cloned().collect::<Vec<_>>();
        for index in indexes {
            let cell = self.cells.get(&index).unwrap();
            let emissions = cell.get_emissions().clone();
            let (x, y) = (cell.x, cell.y);

            for (type_, emission) in emissions {
                for neighbor_id in self.get_cell_neighbors(x, y, index) {
                    let neighbor = self.cells.get_mut(&neighbor_id).unwrap();
                    neighbor.add_force(type_, emission, x, y);

                    if type_ == TOXIN_FORCE as u16 {
                        neighbor.remove_food(emission * FOOD_USED_PER_UNIT_TOXIN_EMITTED);
                    }
                }
            }
        }
    }

    fn update_cell(&mut self, index: &usize) {
        let cell = self.cells.get(index).unwrap();
        let (prev_x, prev_y) = (cell.x, cell.y);
        let cell = self.cells.get_mut(index).unwrap();
        cell.update();
        let (x, y) = (cell.x, cell.y);
        self.move_cell(*index, x, y, prev_x, prev_y);
    }

    fn consume_food(&mut self, index: &usize) {
        let cell = self.cells.get(index).unwrap();
        let (x, y) = (cell.x, cell.y);
        let neighbors = self.get_food_neighbors(x, y);

        for neighbor_id in neighbors {
            let (nx, ny, food) = self.food.get(&neighbor_id).unwrap();
            let dx = nx - x;
            let dy = ny - y;
            let distance_sq = dx * dx + dy * dy;

            if distance_sq < CELL_SIZE_SQ {
                let cell = self.cells.get_mut(index).unwrap();
                cell.eat_food(*food);
                self.remove_food(neighbor_id);
            }
        }
    }

    fn kill_dead(&mut self, index: &usize) -> bool {
        let cell = self.cells.get(index).unwrap();
        if cell.is_dead() {
            self.add_food(cell.x, cell.y, STARTING_FOOD_CELL);
            self.remove_cell(*index);
            return true;
        }
        false
    }

    fn attempt_replication(&mut self, index: &usize) {
        let cell = self.cells.get(index).unwrap();

        if cell.can_replicate() {
            let new_cell = cell.replicate();
            self.add_cell(new_cell);
            self.cells
                .get_mut(index)
                .unwrap()
                .consume_replication_food();
        }
    }

    fn create_new_food(&mut self) {
        for _ in 0..FOOD_PER_TURN {
            self.create_random_food();
        }
    }

    pub fn update(&mut self) {
        self.emit_forces();

        let cell_ids = self.cells.keys().cloned().collect::<Vec<_>>();
        for id in cell_ids {
            self.update_cell(&id);
            self.consume_food(&id);
            if self.kill_dead(&id) {
                continue;
            }
            self.attempt_replication(&id);
        }

        self.create_new_food();
    }
}
