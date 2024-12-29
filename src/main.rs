mod cell;
mod cell_manager;
mod config;
mod dna;
mod id;

fn main() {
    let guard = pprof::ProfilerGuard::new(10000).unwrap();

    let mut cell_manager = cell_manager::CellManager::new();
    cell_manager.init();

    for i in 0..10000 {
        cell_manager.update();
        let len = cell_manager.get_cells().len();
        println!("iteration: {} cells: {}", i, len);
        if len == 0 {
            cell_manager = cell_manager::CellManager::new();
            cell_manager.init();
        }
    }

    if let Ok(report) = guard.report().build() {
        // println!("report: {:?}", &report);
        let file = std::fs::File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    };
}
