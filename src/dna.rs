use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

use rustc_hash::FxHashMap;

use crate::config::*;

enum PrimaryBases {
    Attraction = 0,
    Emission = 1,
    DisableCodon = 2,
    GlobalMutationRate = 3,
    IndividualMutationRate = 4,
    PrimaryMutationRate = 5,
    SecondaryMutationRate = 6,
    AddCodonMutationRate = 7,
    RemoveCodonMutationRate = 8,
    ReplicationFood = 9,
    CellSize = 10,
}

impl From<u8> for PrimaryBases {
    fn from(value: u8) -> Self {
        match value {
            0 => PrimaryBases::Attraction,
            1 => PrimaryBases::Emission,
            2 => PrimaryBases::DisableCodon,
            3 => PrimaryBases::GlobalMutationRate,
            4 => PrimaryBases::IndividualMutationRate,
            5 => PrimaryBases::PrimaryMutationRate,
            6 => PrimaryBases::SecondaryMutationRate,
            7 => PrimaryBases::AddCodonMutationRate,
            8 => PrimaryBases::RemoveCodonMutationRate,
            9 => PrimaryBases::ReplicationFood,
            10 => PrimaryBases::CellSize,
            _ => panic!("Invalid value for PrimaryBases"),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DNA(Vec<(u8, u16, f32)>);

impl DNA {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut dna = Self(Vec::new());
        dna.0.push(dna.random_codon(&mut rng));
        dna
    }

    pub fn get_codon_count(&self) -> usize {
        self.0.len()
    }

    pub fn get_activated_codons(&self, initial_forces: &FxHashMap<u16, f32>) -> Vec<usize> {
        let mut activated_codons = Vec::with_capacity(self.0.len());
        for codon_index in 0..self.0.len() {
            activated_codons.push(codon_index);
        }

        for codon_index in 0..self.0.len() {
            if activated_codons.contains(&codon_index) {
                if self.0[codon_index].0 == PrimaryBases::DisableCodon as u8
                    && *initial_forces.get(&self.0[codon_index].1).unwrap_or(&0.0)
                        >= self.0[codon_index].2
                {
                    activated_codons.retain(|&x| x != codon_index + 1);
                }
            }
        }

        activated_codons
    }

    pub fn get_disabled_codons(&self, initial_forces: &FxHashMap<u16, f32>) -> Vec<(usize, bool)> {
        let mut disabled_codons = Vec::new();
        for codon_index in 0..self.0.len() {
            if self.0[codon_index].0 == PrimaryBases::DisableCodon as u8 {
                if *initial_forces.get(&self.0[codon_index].1).unwrap_or(&0.0)
                    >= self.0[codon_index].2
                {
                    disabled_codons.push((codon_index, true));
                } else {
                    disabled_codons.push((codon_index, false));
                }
            }
        }

        disabled_codons
    }

    pub fn process_dna(
        &self,
        activated_codons: &[usize],
    ) -> (FxHashMap<u16, f32>, Vec<(u16, f32)>, f32, f32) {
        let mut attractions = FxHashMap::default();
        let mut emissions: Vec<(u16, f32)> = Vec::with_capacity(self.0.len());
        let mut food_to_replicate = DEFAULT_FOOD_TO_REPLICATE;
        let mut size = DEFAULT_CELL_SIZE_SQ;

        for codon_index in activated_codons {
            let codon_index = *codon_index;
            match PrimaryBases::from(self.0[codon_index].0) {
                PrimaryBases::Attraction => {
                    let entry = attractions.entry(self.0[codon_index].1).or_insert(0.0);
                    *entry += self.0[codon_index].2;
                }
                PrimaryBases::Emission => {
                    if let Some(emission) = emissions
                        .iter_mut()
                        .find(|x| (**x).0 == self.0[codon_index].1)
                    {
                        emission.1 += self.0[codon_index].2;
                    } else {
                        emissions.push((self.0[codon_index].1, self.0[codon_index].2));
                    }
                }
                PrimaryBases::ReplicationFood => food_to_replicate = self.0[codon_index].2,
                PrimaryBases::CellSize => size = self.0[codon_index].2,
                _ => (),
            }
        }

        (attractions, emissions, food_to_replicate, size)
    }

    fn get_mutation_rates(
        &self,
        rng: &mut ThreadRng,
        activated_codons: &[usize],
    ) -> (f32, FxHashMap<usize, f32>, f32, f32, f32, f32) {
        let mut global_mutation_rate = DEFAULT_MUTATION_RATE;
        let mut individual_mutation_rates = FxHashMap::default();
        let mut primary_mutation_rate = DEFAULT_PRIMARY_MUTATION_RATE;
        let mut secondary_mutation_rate = DEFAULT_SECONDARY_MUTATION_RATE;
        let mut add_codon_mutation_rate = DEFAULT_ADD_CODON_MUTATION_RATE;
        let mut remove_codon_mutation_rate = DEFAULT_REMOVE_CODON_MUTATION_RATE;

        if rng.gen_range(0.0..=1.0) > FUDGED_MUTATION_RATE_CHANCE {
            for codon_index in activated_codons {
                let codon_index = *codon_index;
                match PrimaryBases::from(self.0[codon_index].0) {
                    PrimaryBases::GlobalMutationRate => {
                        global_mutation_rate = self.0[codon_index].2
                    }
                    PrimaryBases::IndividualMutationRate => {
                        individual_mutation_rates
                            .insert(self.0[codon_index].1 as usize, self.0[codon_index].2);
                    }
                    PrimaryBases::PrimaryMutationRate => {
                        primary_mutation_rate = self.0[codon_index].2
                    }
                    PrimaryBases::SecondaryMutationRate => {
                        secondary_mutation_rate = self.0[codon_index].2
                    }
                    PrimaryBases::AddCodonMutationRate => {
                        add_codon_mutation_rate = self.0[codon_index].2
                    }
                    PrimaryBases::RemoveCodonMutationRate => {
                        remove_codon_mutation_rate = self.0[codon_index].2
                    }
                    _ => (),
                }
            }
        }

        (
            global_mutation_rate,
            individual_mutation_rates,
            primary_mutation_rate,
            secondary_mutation_rate,
            add_codon_mutation_rate,
            remove_codon_mutation_rate,
        )
    }

    pub fn get_mutation_rates_no_rng(&self) -> (f32, FxHashMap<usize, f32>, f32, f32, f32, f32) {
        let mut global_mutation_rate = DEFAULT_MUTATION_RATE;
        let mut individual_mutation_rates = FxHashMap::default();
        let mut primary_mutation_rate = DEFAULT_PRIMARY_MUTATION_RATE;
        let mut secondary_mutation_rate = DEFAULT_SECONDARY_MUTATION_RATE;
        let mut add_codon_mutation_rate = DEFAULT_ADD_CODON_MUTATION_RATE;
        let mut remove_codon_mutation_rate = DEFAULT_REMOVE_CODON_MUTATION_RATE;

        for codon_index in 0..self.0.len() {
            match PrimaryBases::from(self.0[codon_index].0) {
                PrimaryBases::GlobalMutationRate => global_mutation_rate = self.0[codon_index].2,
                PrimaryBases::IndividualMutationRate => {
                    individual_mutation_rates
                        .insert(self.0[codon_index].1 as usize, self.0[codon_index].2);
                }
                PrimaryBases::PrimaryMutationRate => primary_mutation_rate = self.0[codon_index].2,
                PrimaryBases::SecondaryMutationRate => {
                    secondary_mutation_rate = self.0[codon_index].2
                }
                PrimaryBases::AddCodonMutationRate => {
                    add_codon_mutation_rate = self.0[codon_index].2
                }
                PrimaryBases::RemoveCodonMutationRate => {
                    remove_codon_mutation_rate = self.0[codon_index].2
                }
                _ => (),
            }
        }

        (
            global_mutation_rate,
            individual_mutation_rates,
            primary_mutation_rate,
            secondary_mutation_rate,
            add_codon_mutation_rate,
            remove_codon_mutation_rate,
        )
    }

    fn fix_broken_codon(&mut self, codon_index: usize) {
        match PrimaryBases::from(self.0[codon_index].0) {
            PrimaryBases::Emission => {
                if self.0[codon_index].1 == FOOD_FORCE {
                    self.0[codon_index].1 = TOXIN_FORCE;
                }

                if self.0[codon_index].1 == TOXIN_FORCE && self.0[codon_index].2 > MAX_TOXIN_FORCE {
                    self.0[codon_index].2 = MAX_TOXIN_FORCE;
                }

                if self.0[codon_index].2 < 0.0 {
                    self.0[codon_index].2 = 0.0;
                }
            }
            PrimaryBases::ReplicationFood => {
                self.0[codon_index].2 = self.0[codon_index]
                    .2
                    .max(CELL_STARTING_FOOD * MIN_FOOD_TO_REPLICATE_RATIO)
            }
            PrimaryBases::DisableCodon | PrimaryBases::CellSize => {
                self.0[codon_index].2 = self.0[codon_index].2.max(0.0)
            }
            PrimaryBases::GlobalMutationRate
            | PrimaryBases::IndividualMutationRate
            | PrimaryBases::PrimaryMutationRate
            | PrimaryBases::SecondaryMutationRate
            | PrimaryBases::AddCodonMutationRate
            | PrimaryBases::RemoveCodonMutationRate => {
                self.0[codon_index].2 = self.0[codon_index].2.max(0.0).min(1.0)
            }
            _ => (),
        };
    }

    fn random_codon(&self, rng: &mut ThreadRng) -> (u8, u16, f32) {
        let primary_base = rng.gen_range(0..=10);
        if primary_base == PrimaryBases::Attraction as u8 {
            (
                primary_base,
                rng.gen_range(0..=10),
                rng.gen_range(-10.0..=10.0),
            )
        } else if primary_base == PrimaryBases::Emission as u8 {
            let secondary_force = rng.gen_range(0..=10);
            if secondary_force == FOOD_FORCE {
                (
                    primary_base,
                    TOXIN_FORCE,
                    rng.gen_range(0.0..=MAX_TOXIN_FORCE),
                )
            } else if secondary_force == TOXIN_FORCE {
                (
                    primary_base,
                    secondary_force,
                    rng.gen_range(0.0..=MAX_TOXIN_FORCE),
                )
            } else {
                (primary_base, secondary_force, rng.gen_range(-10.0..=10.0))
            }
        } else if primary_base == PrimaryBases::ReplicationFood as u8 {
            (
                primary_base,
                0,
                rng.gen_range(
                    CELL_STARTING_FOOD * MIN_FOOD_TO_REPLICATE_RATIO
                        ..=CELL_STARTING_FOOD * MIN_FOOD_TO_REPLICATE_RATIO * 2.0,
                ),
            )
        } else if primary_base == PrimaryBases::DisableCodon as u8 {
            (
                primary_base,
                rng.gen_range(0..=10) as u16,
                rng.gen_range(0.0..=100.0),
            )
        } else if primary_base == PrimaryBases::CellSize as u8 {
            (
                primary_base,
                0,
                rng.gen_range(0.1..=DEFAULT_CELL_SIZE_SQ * 2.0),
            )
        } else {
            (primary_base, 0, rng.gen_range(0.0..=1.0))
        }
    }

    fn frameshift_mutation(
        &mut self,
        rng: &mut ThreadRng,
        add_codon_mutation_rate: f32,
        remove_codon_mutation_rate: f32,
    ) {
        let r = rng.gen_range(0.0..=1.0);
        if r <= add_codon_mutation_rate {
            let codon_index = rng.gen_range(0..self.0.len());
            self.0.insert(codon_index, self.random_codon(rng));
        } else if r <= remove_codon_mutation_rate && self.0.len() > 0 {
            let codon_index = rng.gen_range(0..self.0.len());
            self.0.remove(codon_index);
        }
    }

    pub fn mutate(&mut self, activated_codons: &[usize]) {
        let mut rng = thread_rng();
        let (
            global_mutation_rate,
            individual_mutation_rates,
            primary_mutation_rate,
            secondary_mutation_rate,
            add_codon_mutation_rate,
            remove_codon_mutation_rate,
        ) = self.get_mutation_rates(&mut rng, activated_codons);

        for codon_index in 0..self.0.len() {
            let mutation_rate = individual_mutation_rates
                .get(&codon_index)
                .unwrap_or(&global_mutation_rate);

            if rng.gen_range(0.0..=1.0) > *mutation_rate {
                continue;
            }

            let mutation_type = rng.gen_range(0.0..=1.0);
            if mutation_type <= primary_mutation_rate {
                self.0[codon_index].0 = rng.gen_range(0..=10);
            } else if mutation_type <= secondary_mutation_rate {
                self.0[codon_index].1 =
                    (self.0[codon_index].1 as i16 + rng.gen_range(-1..=1)).max(0) as u16;
            } else {
                self.0[codon_index].2 += rng.gen_range(-1.0..=1.0);
            }
            self.fix_broken_codon(codon_index);
        }

        self.frameshift_mutation(
            &mut rng,
            add_codon_mutation_rate,
            remove_codon_mutation_rate,
        );
    }
}
