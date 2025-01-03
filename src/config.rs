pub const WINDOW_WIDTH: usize = 500;
pub const WINDOW_HEIGHT: usize = 500;
pub const SLEEP_TIME: f32 = 0.1;

pub const ITERATIONS: usize = 100000;
pub const PRINT_DETAILS_AFTER_FRAMES: usize = 100;

pub const GAME_SIZE: usize = 100;
pub const GRID_CELL_SIZE: usize = 10;

pub const STARTING_CELLS: usize = 100;
pub const STARTING_FOOD: usize = 100;

pub const DEFAULT_FOOD_VALUE: f32 = 10.0;
pub const DEFAULT_CELL_FOOD_VALUE: f32 = 5.0;
pub const FOOD_ADDED_PER_FRAME: usize = 10;

pub const FORCE_MAX_RANGE_SQ: f32 = 100.0;
pub const FOOD_FORCE: u16 = 0;
pub const TOXIN_FORCE: u16 = 1;

pub const CELL_STARTING_FOOD: f32 = 50.0;

pub const DEFAULT_MUTATION_RATE: f32 = 0.015;
pub const DEFAULT_PRIMARY_MUTATION_RATE: f32 = 0.001;
pub const DEFAULT_SECONDARY_MUTATION_RATE: f32 = 0.01;
pub const DEFAULT_ADD_CODON_MUTATION_RATE: f32 = 0.001;
pub const DEFAULT_REMOVE_CODON_MUTATION_RATE: f32 = 0.0001;
pub const DEFAULT_FOOD_TO_REPLICATE: f32 = 65.0;
pub const DEFAULT_CELL_SIZE_SQ: f32 = 4.0;

pub const FOOD_USED_PER_FRAME: f32 = 0.1;
pub const FOOD_STOLEN_PER_TOXIN_UNIT: f32 = 0.5;
pub const FOOD_USED_PER_UNIT_MOVED: f32 = 0.1;
pub const FOOD_USED_PER_SIZE_UNIT: f32 = 0.1;
pub const FOOD_USED_PER_FORCE_EMITTED: f32 = 0.1;
pub const FOOD_USED_PER_TOXIN_UNIT_EMITTED: f32 = 0.1;
pub const FOOD_RETENTION_FROM_REPLICATION: f32 = 0.5;
