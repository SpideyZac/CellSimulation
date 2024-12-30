mod cell;
mod cell_manager;
mod config;
mod dna;
mod id;

use config::*;

fn main() {
    // let guard = pprof::ProfilerGuard::new(10000).unwrap();

    let mut cell_manager = cell_manager::CellManager::new();
    cell_manager.init();

    for i in 0..ITERATIONS {
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

    // if let Ok(report) = guard.report().build() {
    //     // println!("report: {:?}", &report);
    //     let file = std::fs::File::create("flamegraph.svg").unwrap();
    //     report.flamegraph(file).unwrap();
    // };
}
