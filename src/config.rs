pub const STATE_PATH: &str = "state";

#[allow(dead_code)]
pub const WINDOW_WIDTH: usize = 1000;
#[allow(dead_code)]
pub const WINDOW_HEIGHT: usize = 1000;
#[allow(dead_code)]
pub const SLEEP_TIME: f32 = 0.05;

pub const ITERATIONS: usize = 100_000_000_000_000;
pub const PRINT_DETAILS_AFTER_FRAMES: usize = 1000;

pub const GAME_SIZE: usize = 1000;
pub const GRID_CELL_SIZE: usize = 100;

pub const STARTING_CELLS: usize = 100;
pub const STARTING_FOOD: usize = 100;

pub const DEFAULT_FOOD_VALUE: f32 = 25.0;
pub const DEFAULT_CELL_FOOD_VALUE: f32 = 15.0;
pub const FOOD_ADDED_PER_FRAME: usize = 5;
pub const MAX_FOOD: usize = 1000;

pub const FORCE_MAX_RANGE_SQ: f32 = 10000.0;
pub const FOOD_FORCE: u16 = 0;
pub const TOXIN_FORCE: u16 = 1;

pub const CELL_STARTING_FOOD: f32 = 50.0;
pub const MAX_CELLS: usize = 1000;
pub const MIN_FOOD_TO_REPLICATE_RATIO: f32 = 1.1;
pub const MAX_TOXIN_FORCE: f32 = 2.0;

pub const DEFAULT_MUTATION_RATE: f32 = 0.01;
pub const DEFAULT_PRIMARY_MUTATION_RATE: f32 = 0.001;
pub const DEFAULT_SECONDARY_MUTATION_RATE: f32 = 0.01;
pub const DEFAULT_ADD_CODON_MUTATION_RATE: f32 = 0.001;
pub const DEFAULT_REMOVE_CODON_MUTATION_RATE: f32 = 0.0001;
pub const DEFAULT_FOOD_TO_REPLICATE: f32 = 60.0;
pub const DEFAULT_CELL_SIZE_SQ: f32 = 16.0;

pub const FOOD_USED_PER_FRAME: f32 = 0.2;
pub const FOOD_STOLEN_PER_TOXIN_UNIT: f32 = 5.0;
pub const FOOD_USED_PER_UNIT_MOVED: f32 = 0.02;
pub const FOOD_USED_PER_SIZE_UNIT: f32 = 0.05;
pub const FOOD_USED_PER_FORCE_EMITTED: f32 = 0.001;
pub const FOOD_USED_PER_TOXIN_UNIT_EMITTED: f32 = 0.002;
pub const FOOD_RETENTION_FROM_REPLICATION: f32 = 0.5;
