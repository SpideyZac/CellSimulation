mod cell;
mod cell_manager;
mod config;
mod dna;
mod id;

use config::*;

#[derive(serde::Serialize, serde::Deserialize)]
struct SimulationState {
    cells: Vec<cell::Cell>,
    food: Vec<(f32, f32, f32)>,
}

fn save_state(state: &SimulationState, path: &str) -> std::io::Result<()> {
    let file = std::fs::File::create(path)?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer(writer, state)?;
    Ok(())
}

fn load_state(path: &str) -> std::io::Result<SimulationState> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let state = serde_json::from_reader(reader)?;
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

    if let Ok(state) = load_state("state.json") {
        println!("Loaded state from file");
        cell_manager.init_with_starting(state.cells, state.food);
    } else {
        println!("No state file found, starting fresh");
        cell_manager.init();
    }

    for i in 0..ITERATIONS {
        if !running.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }
        cell_manager.update();
        let len = cell_manager.get_cells().len();
        if i % PRINT_DETAILS_AFTER_FRAMES == 0 {
            println!("iteration: {} cells: {}", i, len);
        }
        if len == 0 {
            cell_manager = cell_manager::CellManager::new();
            cell_manager.init();
        }
    }

    let state = SimulationState {
        cells: cell_manager.get_cells_cloned().into_values().collect(),
        food: cell_manager.get_food_cloned().into_values().collect(),
    };

    println!("Saving state to file");
    save_state(&state, "state.json").unwrap();

    #[cfg(feature = "profiling")]
    if let Ok(report) = guard.report().build() {
        println!("Writing flamegraph to flamegraph.svg");
        let file = std::fs::File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    }
}
