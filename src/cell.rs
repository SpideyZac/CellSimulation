use rand::{thread_rng, Rng};

use rustc_hash::FxHashMap;

use crate::config::*;
use crate::dna::DNA;

pub struct Cell {
    pub id: u64,
    dna: DNA,
    attractions: FxHashMap<u16, f32>,
    emissions: Vec<(u16, f32)>,
    food_to_replicate: f32,
    pub size: f32,
    pub x: f32,
    pub y: f32,
    next_x: f32,
    next_y: f32,
    pub food: f32,
    last_forces: FxHashMap<u16, f32>,
}

impl Cell {
    fn _new(
        id: u64,
        initial_forces: FxHashMap<u16, f32>,
        dna: DNA,
        x: f32,
        y: f32,
    ) -> (Self, Vec<usize>) {
        let activated_codons = dna.get_activated_codons(initial_forces);
        let (attractions, emissions, food_to_replicate, size) = dna.process_dna(&activated_codons);

        (
            Cell {
                id,
                dna,
                attractions,
                emissions,
                food_to_replicate,
                size,
                x,
                y,
                next_x: x,
                next_y: y,
                food: CELL_STARTING_FOOD,
                last_forces: FxHashMap::default(),
            },
            activated_codons,
        )
    }

    pub fn new(id: u64, initial_forces: FxHashMap<u16, f32>) -> Self {
        let dna = DNA::new();

        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..GAME_SIZE as f32);
        let y = rng.gen_range(0.0..GAME_SIZE as f32);

        let (cell, _) = Self::_new(id, initial_forces, dna, x, y);
        cell
    }

    pub fn add_food(&mut self, food: f32) {
        self.food += food;
    }

    pub fn remove_food(&mut self, food: f32) {
        self.food -= food;
    }

    pub fn add_forces(&mut self, forces: &[(u16, f32)], force_x: f32, force_y: f32) {
        for (force, magnitude) in forces {
            *self.last_forces.entry(*force).or_insert(0.0) += *magnitude;

            let x = self.x - force_x;
            let y = self.y - force_y;

            let distance_sq = x * x + y * y;
            let scaled_force =
                *magnitude * *self.attractions.get(&force).unwrap_or(&0.0) / distance_sq;

            self.next_x += -x * scaled_force;
            self.next_y += -y * scaled_force;

            if *force == TOXIN_FORCE {
                self.remove_food(*magnitude * FOOD_STOLEN_PER_TOXIN_UNIT);
            }
        }
    }

    fn update_pos(&mut self) {
        self.x = self.next_x;
        self.y = self.next_y;
    }

    fn calculate_general_food_usage(&self, prev_x: f32, prev_y: f32) -> f32 {
        let mut food_usage = 0.0;

        food_usage += (self.x - prev_x).abs() * FOOD_USED_PER_UNIT_MOVED;
        food_usage += (self.y - prev_y).abs() * FOOD_USED_PER_UNIT_MOVED;

        food_usage += self.size * FOOD_USED_PER_SIZE_UNIT;

        for (id, magnitude) in self.emissions.iter() {
            if *id == TOXIN_FORCE {
                food_usage += *magnitude * FOOD_USED_PER_TOXIN_UNIT_EMITTED;
            } else {
                food_usage += *magnitude * FOOD_USED_PER_FORCE_EMITTED;
            }
        }

        food_usage
    }

    fn update_food(&mut self, prev_x: f32, prev_y: f32) {
        let food_usage = self.calculate_general_food_usage(prev_x, prev_y);
        self.remove_food(food_usage);
    }

    pub fn update(&mut self) {
        let prev_x = self.x;
        let prev_y = self.y;

        self.update_pos();
        self.update_food(prev_x, prev_y);
    }

    pub fn replicate(&mut self, id: u64) -> Cell {
        let (mut new_cell, activated_codons) = Self::_new(
            id,
            self.last_forces.clone(),
            self.dna.clone(),
            self.x,
            self.y,
        );
        new_cell.dna.mutate(&activated_codons);

        self.food -= self.food_to_replicate * FOOD_RETENTION_FROM_REPLICATION;

        new_cell
    }

    pub fn can_replicate(&self) -> bool {
        self.food >= self.food_to_replicate
    }

    pub fn reset(&mut self) {
        self.last_forces.clear();
    }

    pub fn is_dead(&self) -> bool {
        self.food <= 0.0
    }
}
