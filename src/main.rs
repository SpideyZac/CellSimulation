use rand::{thread_rng, Rng};

use rustc_hash::FxHashMap;

// Game Constants
const MAX_CELLS: u16 = 1000;
const GAME_SIZE: u16 = 1000; // Total board is GAME_SIZE x GAME_SIZE
const GAME_SQUARE_SIZE: u16 = 10; // Each cell is GAME_SQUARE_SIZE x GAME_SQUARE_SIZE
const STARTING_FOOD_SPAWNED: u16 = 500; // Food spawned at start
const STARTING_FOOD_CELL: f32 = 100.0; // Food in each cell at start
const FOOD_PER_TURN: u16 = 1; // Food spawned per turn

// Cell Constants
const CELL_SIZE_SQ: u16 = 4; // r = 2 - we try to avoid square roots

#[derive(Debug)]
struct Cell {
    x: f32,
    y: f32,
    prev_x: f32, // used for calculating food usage
    prev_y: f32,
    next_forces: FxHashMap<u16, f32>, // force_id, magnitude - used for reproduction - todo: figure out how to remove this
    dna: Vec<f32>,
    attractions: FxHashMap<u16, f32>, // force_id, magnitude - used for calculating movements
    emissions: Vec<(u16, f32)>,       // force_id, magnitude - used for creating forces
    food: f32,
}

impl Cell {
    fn new_random(max_force: u32) -> Cell {
        let mut rng = thread_rng();
        let mut dna = Vec::new();
        let dna_length_codons = rng.gen_range(1..=10);

        // this is the most basic type of DNA - where the Primary Base is 0 or 1 meaning it only has force attractions and emissions
        for _ in 0..dna_length_codons {
            let base1 = rng.gen_range(0..=1); // Primary Base
            let base2 = rng.gen_range(0..=max_force); // Secondary Base
            let base3 = if base1 == 0 {
                // Tertiary Base
                rng.gen_range(-1.0..=1.0)
            } else {
                rng.gen_range(0.1..=1.0)
            };

            dna.push(base1 as f32);
            dna.push(base2 as f32);
            dna.push(base3);
        }

        let mut attractions = FxHashMap::default();
        let mut emissions = Vec::new();

        for i in (0..dna.len()).step_by(3) {
            let base1 = dna[i] as u16;
            let base2 = dna[i + 1] as u16;
            let base3 = dna[i + 2];

            if base1 == 0 {
                if let Some(attraction) = attractions.get_mut(&base2) {
                    *attraction += base3;
                } else {
                    attractions.insert(base2, base3);
                }
            } else {
                let mut found = false;
                for (index, (force_id, magnitude)) in emissions.iter().enumerate() {
                    if *force_id == base2 {
                        emissions[index] = (*force_id, *magnitude + base3);
                        found = true;
                        break;
                    }
                }

                if !found {
                    emissions.push((base2, base3));
                }
            }
        }

        Cell {
            x: rng.gen_range(0.0..=GAME_SIZE as f32),
            y: rng.gen_range(0.0..=GAME_SIZE as f32),
            prev_x: 0.0,
            prev_y: 0.0,
            next_forces: FxHashMap::default(),
            dna,
            attractions,
            emissions,
            food: STARTING_FOOD_CELL,
        }
    }
}

fn main() {
    let mut cells = Vec::new();

    for _ in 0..1 {
        cells.push(Cell::new_random(10));
    }

    for i in 0..1 {
        let cell = &cells[i as usize];
        println!("Cell {}: {:?}", i, cell);
    }
}
