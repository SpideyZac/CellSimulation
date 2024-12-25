use rand::{thread_rng, Rng};

use rustc_hash::FxHashMap;

// Game Constants
const MAX_CELLS: u16 = 1000;
const GAME_SIZE: u16 = 1000; // Total board is GAME_SIZE x GAME_SIZE
const GAME_SQUARE_SIZE: u16 = 10; // Each cell is GAME_SQUARE_SIZE x GAME_SQUARE_SIZE
const STARTING_FOOD_SPAWNED: u16 = 500; // Food spawned at start
const STARTING_FOOD_CELL: f32 = 100.0; // Food in each cell at start
const FOOD_PER_TURN: u16 = 1; // Food spawned per turn
const FOOD_FORCE: u8 = 0; // Force of food
const TOXIN_FORCE: u8 = 1; // Force of toxin

// Cell Constants
const CELL_SIZE_SQ: u16 = 4; // r = 2 - we try to avoid square roots
const DEFAULT_MUTATION_RATE: f32 = 0.01; // Default mutation rate for a codon
const PRIMARY_DEFAULT_MUTATION_RATE: f32 = 0.001; // Mutation rate for the primary base
const SECONDARY_DEFAULT_MUTATION_RATE: f32 = 0.10; // Mutation rate for the secondary base
const ADD_CODON_DEFAULT_MUTATION_RATE: f32 = 0.005; // Mutation rate for adding a codon
const DELETE_CODON_DEFAULT_MUTATION_RATE: f32 = 0.0005; // Mutation rate for deleting a codon

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
    activated_genes: Vec<u32>,
}

impl Cell {
    fn new(x: f32, y: f32, dna: Vec<f32>, next_forces: FxHashMap<u16, f32>) -> Cell {
        let (attractions, emissions, activated_genes) =
            Cell::process_initial_dna(&dna, &next_forces);

        Cell {
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
        }
    }

    fn new_random(max_force: u32) -> Cell {
        let mut rng = thread_rng();
        let mut dna = Vec::new();
        let dna_length_codons = rng.gen_range(1..=10);

        // this is the most basic type of DNA - where the Primary Base is 0 or 1 meaning it only has force attractions and emissions
        for _ in 0..dna_length_codons {
            let base1 = rng.gen_range(0..=1); // Primary Base
            let mut base2 = rng.gen_range(0..=max_force); // Secondary Base

            // Prevent emittion of food force
            while base1 == 1 && base2 == FOOD_FORCE as u32 {
                base2 = rng.gen_range(0..=max_force);
            }

            // Tertiary Base
            let base3 = if base1 == 0 {
                rng.gen_range(-1.0..=1.0)
            } else {
                rng.gen_range(0.1..=1.0)
            };

            dna.push(base1 as f32);
            dna.push(base2 as f32);
            dna.push(base3);
        }

        let (attractions, emissions, activated_genes) =
            Cell::process_initial_dna(&dna, &FxHashMap::default());

        let start_x = rng.gen_range(0.0..=GAME_SIZE as f32);
        let start_y = rng.gen_range(0.0..=GAME_SIZE as f32);

        Cell {
            x: start_x,
            y: start_y,
            prev_x: start_x,
            prev_y: start_y,
            next_forces: FxHashMap::default(),
            dna,
            attractions,
            emissions,
            food: STARTING_FOOD_CELL,
            activated_genes,
        }
    }

    fn process_initial_dna(
        dna: &[f32],
        initial_forces: &FxHashMap<u16, f32>,
    ) -> (FxHashMap<u16, f32>, Vec<(u16, f32)>, Vec<u32>) {
        let mut attractions = FxHashMap::default();
        let mut emissions = Vec::new();
        let mut activated_genes = Vec::new();

        for i in (0..dna.len()).step_by(3) {
            activated_genes.push(i as u32 / 3);
        }

        for i in (0..dna.len()).step_by(3) {
            let base1 = dna[i] as u16;
            let base2 = dna[i + 1] as u16;
            let base3 = dna[i + 2];

            if base1 == 2 {
                if *initial_forces
                    .get(&base2)
                    .or(Some(&f32::NEG_INFINITY))
                    .unwrap()
                    < base3
                    && base2 < (dna.len() / 3) as u16
                {
                    activated_genes.remove(base2 as usize);
                }
            }
        }

        for i in (0..dna.len()).step_by(3) {
            if !activated_genes.contains(&(i as u32 / 3)) {
                continue;
            }

            let base1 = dna[i] as u16;
            let base2 = dna[i + 1] as u16;
            let base3 = dna[i + 2];

            if base1 == 0 {
                if let Some(attraction) = attractions.get_mut(&base2) {
                    *attraction += base3;
                } else {
                    attractions.insert(base2, base3);
                }
            } else if base1 == 1 {
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

        (attractions, emissions, activated_genes)
    }

    fn replicate(&self) -> Cell {
        /*
        Replication is decently complex in this model.
        1. Create a new cell with the same DNA, x, y, prev_x, prev_y (food starts at STARTING_FOOD_CELL)
        2. Based on the dna, we need to mutate the dna. Explanation:
            a. there are a few primary bases that determine how mutations occur (if they are activated)
                i. 3 - add a custom mutation rate for a specific gene (the secondary base) with a value 0-1 determined by the ternary base
                ii. 4 - add a custom mutation rate for all genes with a value 0-1 determined by the secondary base
                iii. 5 - add a custom mutation rate for all primary bases with a value 0-1 determined by the secondary base
                iv. 6 - add a custom mutation rate for all secondary bases with a value 0-1 determined by the secondary base
                v. 7 - add a custom mutation rate for deleting a codon with a value 0-1 determined by the secondary base
                vi. 8 - add a custom mutation rate for adding a codon with a value 0-1 determined by the secondary base
            b. otherwise, if there is no mutation rate for a gene, we use the defaults
        3. Fix any broken codons: rules:
            Primary bases:
                0:
                    none!
                1:
                    a. secondary base (as u16) cannot be the food force
                    b. if the secondary base is the toxin force, the teritary base must be between 0.1 and 2.0
                    c. the teritary base cannot be negative
                2:
                    a. teritary base cannot be negative
                3:
                    a. teritary base cannot be negative
                4:
                    a. teritary base cannot be negative
                5:
                    a. teritary base cannot be negative
                6:
                    a. teritary base cannot be negative
                7:
                    a. teritary base cannot be negative
                8:
                    a. teritary base cannot be negative
        4. Return the new cell
         */

        let mut rng = thread_rng();
        let mut new_dna = self.dna.clone();

        let mut genes_mutation_rates: FxHashMap<u32, f32> = FxHashMap::default();
        let mut gene_mutation_rate = DEFAULT_MUTATION_RATE;
        let mut gene_mutation_rate_primary = PRIMARY_DEFAULT_MUTATION_RATE;
        let mut gene_mutation_rate_secondary = SECONDARY_DEFAULT_MUTATION_RATE;
        let mut gene_mutation_rate_delete_codon = DELETE_CODON_DEFAULT_MUTATION_RATE;
        let mut gene_mutation_rate_add_codon = ADD_CODON_DEFAULT_MUTATION_RATE;

        // Calculate mutation rates
        for i in (0..new_dna.len()).step_by(3) {
            let base1 = new_dna[i] as u16;
            let base2 = new_dna[i + 1] as u16;
            let base3 = new_dna[i + 2];

            if base1 == 3 {
                genes_mutation_rates.insert(base2 as u32, base3);
            } else if base1 == 4 {
                gene_mutation_rate = base3;
            } else if base1 == 5 {
                gene_mutation_rate_primary = base3;
            } else if base1 == 6 {
                gene_mutation_rate_secondary = base3;
            } else if base1 == 7 {
                gene_mutation_rate_delete_codon = base3;
            } else if base1 == 8 {
                gene_mutation_rate_add_codon = base3;
            }
        }

        // Mutate DNA
        for i in (0..new_dna.len()).step_by(3) {
            let mut_rate = *genes_mutation_rates
                .get(&(i as u32 / 3))
                .or(Some(&gene_mutation_rate))
                .unwrap();

            if rng.gen_range(0.0..=1.0) >= mut_rate {
                continue;
            }
            let decider = rng.gen_range(0.0..=1.0);

            if decider < gene_mutation_rate_primary {
                new_dna[i] = rng.gen_range(0..=8) as f32;
            } else if decider < gene_mutation_rate_secondary {
                new_dna[i + 1] += rng.gen_range(-1.0..=1.0);
            } else {
                new_dna[i + 2] += rng.gen_range(-1.0..=1.0);
            }
        }

        let frameshift = rng.gen_range(0.0..=1.0);
        if frameshift < gene_mutation_rate_delete_codon {
            let index = rng.gen_range(0..=(new_dna.len() / 3) as usize);
            new_dna.remove(index * 3);
            new_dna.remove(index * 3);
            new_dna.remove(index * 3);
        } else if frameshift < gene_mutation_rate_add_codon {
            let index = rng.gen_range(0..=(new_dna.len() / 3) as usize);
            let base1 = rng.gen_range(0..=8) as f32;
            let base2 = rng.gen_range(-10.0..=10.0);
            let base3 = rng.gen_range(-1.0..=1.0);

            new_dna.insert(index * 3, base1);
            new_dna.insert(index * 3 + 1, base2);
            new_dna.insert(index * 3 + 2, base3);
        }

        // Fix broken codons
        for i in (0..new_dna.len()).step_by(3) {
            if new_dna[i] as u16 == 1 {
                while new_dna[i + 1] as u16 == FOOD_FORCE as u16 {
                    new_dna[i + 1] += rng.gen_range(-1.0..=1.0);
                }
                if new_dna[i + 1] as u16 == TOXIN_FORCE as u16 {
                    while new_dna[i + 2] < 0.1 || new_dna[i + 2] > 2.0 {
                        new_dna[i + 2] = rng.gen_range(0.1..=2.0);
                    }
                }
                if new_dna[i + 2] < 0.0 {
                    new_dna[i + 2] = rng.gen_range(0.0..=1.0);
                }
            } else if new_dna[i] as u16 == 2 {
                if new_dna[i + 2] < 0.0 {
                    new_dna[i + 2] = rng.gen_range(0.0..=1.0);
                }
            } else if new_dna[i] as u16 == 3 {
                if new_dna[i + 2] < 0.0 {
                    new_dna[i + 2] = rng.gen_range(0.0..=1.0);
                }
            } else if new_dna[i] as u16 == 4 {
                if new_dna[i + 2] < 0.0 {
                    new_dna[i + 2] = rng.gen_range(0.0..=1.0);
                }
            } else if new_dna[i] as u16 == 5 {
                if new_dna[i + 2] < 0.0 {
                    new_dna[i + 2] = rng.gen_range(0.0..=1.0);
                }
            } else if new_dna[i] as u16 == 6 {
                if new_dna[i + 2] < 0.0 {
                    new_dna[i + 2] = rng.gen_range(0.0..=1.0);
                }
            } else if new_dna[i] as u16 == 7 {
                if new_dna[i + 2] < 0.0 {
                    new_dna[i + 2] = rng.gen_range(0.0..=1.0);
                }
            } else if new_dna[i] as u16 == 8 {
                if new_dna[i + 2] < 0.0 {
                    new_dna[i + 2] = rng.gen_range(0.0..=1.0);
                }
            }
        }

        Cell::new(self.x, self.y, new_dna, self.next_forces.clone())
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

    for i in 0..1 {
        let cell = &cells[i as usize];
        let new_cell = cell.replicate();
        println!("Cell {}: {:?} {}", i, new_cell, cell.dna == new_cell.dna);
    }
}
