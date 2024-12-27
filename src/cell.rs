use rand::{thread_rng, Rng};

use rustc_hash::FxHashMap;

use crate::config::*;
use crate::dna::DNA;

pub struct Cell {
    pub id: u64,
    pub dna: DNA,
    pub attractions: FxHashMap<u16, f32>,
    pub emissions: Vec<(u16, f32)>,
    pub food_to_replicate: f32,
    pub size: f32,
    pub x: f32,
    pub y: f32,
    pub next_x: f32,
    pub next_y: f32,
    pub food: f32,
    pub last_forces: FxHashMap<u16, f32>,
}

impl Cell {
    pub fn new(id: u64, initial_forces: FxHashMap<u16, f32>) -> Self {
        let dna = DNA::new();
        let activated_codons = dna.get_activated_codons(initial_forces);
        let (attractions, emissions, food_to_replicate, size) = dna.process_dna(activated_codons);

        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..GAME_SIZE as f32);
        let y = rng.gen_range(0.0..GAME_SIZE as f32);

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
        }
    }
}
