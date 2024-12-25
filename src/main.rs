use rand::{thread_rng, Rng};
use rustc_hash::FxHashMap;

// Game Constants
const MAX_CELLS: u16 = 1000;
const GAME_SIZE: u16 = 1000;
const GAME_SQUARE_SIZE: u16 = 10;
const STARTING_FOOD_SPAWNED: u16 = 500;
const STARTING_FOOD_CELL: f32 = 100.0;
const FOOD_PER_TURN: u16 = 1;
const FOOD_FORCE: u8 = 0;
const TOXIN_FORCE: u8 = 1;

// Cell Constants
const CELL_SIZE_SQ: u16 = 4;
const DEFAULT_MUTATION_RATE: f32 = 0.01;
const PRIMARY_DEFAULT_MUTATION_RATE: f32 = 0.001;
const SECONDARY_DEFAULT_MUTATION_RATE: f32 = 0.10;
const ADD_CODON_DEFAULT_MUTATION_RATE: f32 = 0.005;
const DELETE_CODON_DEFAULT_MUTATION_RATE: f32 = 0.0005;

#[derive(Debug, Clone)]
struct DNA(Vec<f32>);

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
                    chunk[0] = rng.gen_range(0..=8) as f32;
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
            let index = rng.gen_range(0..=(dna.len() / 3)) * 3;
            dna.drain(index..index + 3);
        } else if r < add_rate {
            let index = rng.gen_range(0..=(dna.len() / 3)) * 3;
            let new_codon = [
                rng.gen_range(0..=8) as f32,
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
                _ => {}
            }
        }
    }
}

#[derive(Debug)]
struct Cell {
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
}

impl Cell {
    fn new(x: f32, y: f32, dna: DNA, next_forces: FxHashMap<u16, f32>) -> Self {
        let (attractions, emissions, activated_genes) = Self::process_dna(&dna, &next_forces);

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
        }
    }

    fn new_random(max_force: u32) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=GAME_SIZE as f32);
        let y = rng.gen_range(0.0..=GAME_SIZE as f32);
        let dna = DNA::new_random(max_force);

        Self::new(x, y, dna, FxHashMap::default())
    }

    fn process_dna(
        dna: &DNA,
        initial_forces: &FxHashMap<u16, f32>,
    ) -> (FxHashMap<u16, f32>, Vec<(u16, f32)>, Vec<u32>) {
        let mut attractions = FxHashMap::default();
        let mut emissions = Vec::new();
        let mut activated_genes = (0..dna.0.len() / 3).map(|i| i as u32).collect::<Vec<_>>();

        for chunk in dna.0.chunks(3) {
            if chunk[0] as u16 == 2 {
                let base2 = chunk[1] as u16;
                if base2 < (dna.0.len() / 3) as u16
                    && *initial_forces.get(&base2).unwrap_or(&f32::NEG_INFINITY) < chunk[2]
                {
                    activated_genes.retain(|&x| x != base2 as u32);
                }
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

        (attractions, emissions, activated_genes)
    }

    fn replicate(&self) -> Self {
        let new_dna = self.dna.mutate(&self.activated_genes);
        Self::new(self.x, self.y, new_dna, self.next_forces.clone())
    }
}

fn main() {
    let mut cells = Vec::new();

    for _ in 0..1 {
        cells.push(Cell::new_random(10));
    }

    for (i, cell) in cells.iter().enumerate() {
        println!("Cell {}: {:?}", i, cell);
    }

    for (i, cell) in cells.iter().enumerate() {
        let new_cell = cell.replicate();
        println!(
            "Cell {}: {:?} {}",
            i,
            new_cell,
            cell.dna.0 == new_cell.dna.0
        );
    }
}
