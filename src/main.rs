mod cell;
mod cell_manager;
mod config;
mod dna;
mod id;

use std::fs::File;

fn main() {
    let guard = pprof::ProfilerGuard::new(1000).unwrap();

    let mut cell_manager = cell_manager::CellManager::new();
    cell_manager.init();

    for _ in 0..1000 {
        cell_manager.update();
    }

    if let Ok(report) = guard.report().build() {
        println!("report: {:?}", &report);
        let file = File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    };
}
