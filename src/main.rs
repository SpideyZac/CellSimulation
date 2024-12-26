mod cell;
mod config;

use cell::CellManager;

fn main() {
    let mut cell_manager = CellManager::new();
    let mut i = 0;
    loop {
        i += 1;
        cell_manager.update();
        println!("{}", i);
    }
}
