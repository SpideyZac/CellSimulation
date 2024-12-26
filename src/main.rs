mod cell;
mod config;

use cell::CellManager;

fn main() {
    let mut cell_manager = CellManager::new();
    let mut i = 0;
    loop {
        i += 1;
        cell_manager.update();
        println!(
            "{} Cells: {} {} {}",
            i,
            cell_manager.get_cells().len(),
            cell_manager.next_cell_id,
            cell_manager.next_food_id
        );
        if cell_manager.get_cells().len() == 0 {
            cell_manager = CellManager::new();
            i = 0;
        }
    }
}
