mod cell;
mod cell_manager;
mod config;
mod dna;
mod id;

use config::*;

#[cfg(feature = "graphics")]
mod graphics {
    use minifb::{Key, Window, WindowOptions};

    use crate::config::*;

    pub struct Graphics {
        window: Window,
        scale_factor_x: f32,
        scale_factor_y: f32,
    }

    impl Graphics {
        pub fn new() -> Self {
            let window = Window::new(
                "Cellular Automata",
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                WindowOptions::default(),
            )
            .unwrap();
            let scale_factor_x = WINDOW_WIDTH as f32 / GAME_SIZE as f32;
            let scale_factor_y = WINDOW_HEIGHT as f32 / GAME_SIZE as f32;
            Self {
                window,
                scale_factor_x,
                scale_factor_y,
            }
        }

        pub fn update(&mut self, cells: &Vec<crate::cell::Cell>, food: &Vec<(f32, f32, f32)>) {
            let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

            for cell in cells {
                let x = cell.get_x();
                let y = cell.get_y();

                let x = (x * self.scale_factor_x) as usize;
                let y = (y * self.scale_factor_y) as usize;

                let color = 0xFFFFFF;
                buffer[y * WINDOW_WIDTH + x] = color;
            }

            for f in food {
                let x = f.0;
                let y = f.1;

                let x = (x * self.scale_factor_x) as usize;
                let y = (y * self.scale_factor_y) as usize;

                let color = 0xFF0000;
                buffer[y * WINDOW_WIDTH + x] = color;
            }

            self.window
                .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
                .unwrap();
        }

        pub fn is_open(&self) -> bool {
            self.window.is_open() && !self.window.is_key_down(Key::Escape)
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SimulationState {
    cells: Vec<cell::Cell>,
    food: Vec<(f32, f32, f32)>,
}

fn save_state(state: &SimulationState, path: &str) -> std::io::Result<()> {
    std::fs::File::create(path)?;
    let buffer = bincode::serialize(state).unwrap();
    std::fs::write(path, buffer)?;
    Ok(())
}

fn load_state(path: &str) -> std::io::Result<SimulationState> {
    let buffer = std::fs::read(path)?;
    let state = bincode::deserialize(&buffer).unwrap();
    Ok(state)
}

fn main() {
    #[cfg(feature = "profiling")]
    let guard = pprof::ProfilerGuard::new(10000).unwrap();

    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let mut cell_manager = cell_manager::CellManager::new();

    let running_clone = running.clone();
    let _ = ctrlc::set_handler(move || {
        let _ = running_clone.store(false, std::sync::atomic::Ordering::SeqCst);
    });

    if let Ok(state) = load_state(STATE_PATH) {
        println!("Loaded state from file");
        cell_manager.init_with_starting(state.cells, state.food);
    } else {
        println!("No state file found, starting fresh");
        cell_manager.init();
    }

    #[cfg(feature = "graphics")]
    println!("Initiating graphics");
    #[cfg(feature = "graphics")]
    let mut graphics_win = graphics::Graphics::new();

    for i in 0..ITERATIONS {
        if !running.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }
        cell_manager.update();
        let cells = cell_manager.get_cells();
        let len = cells.len();
        if i % PRINT_DETAILS_AFTER_FRAMES == 0 {
            println!("iteration: {} cells: {}", i, len);
        }

        #[cfg(feature = "graphics")]
        {
            let cells = cell_manager.get_cells_cloned().into_values().collect();
            let food = cell_manager.get_food_cloned().into_values().collect();

            graphics_win.update(&cells, &food);
            if !graphics_win.is_open() {
                break;
            }

            std::thread::sleep(std::time::Duration::from_secs_f32(SLEEP_TIME));
        }

        if len == 0 {
            println!("All cells are dead, restarting simulation");
            cell_manager = cell_manager::CellManager::new();
            cell_manager.init();
        }
    }

    let state = SimulationState {
        cells: cell_manager.get_cells_cloned().into_values().collect(),
        food: cell_manager.get_food_cloned().into_values().collect(),
    };

    println!("Saving state to file");
    save_state(&state, STATE_PATH).unwrap();

    #[cfg(feature = "profiling")]
    if let Ok(report) = guard.report().build() {
        println!("Writing flamegraph to flamegraph.svg");
        let file = std::fs::File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    }
}
