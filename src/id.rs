pub struct IdManager {
    next_id: u64,
    recycled_ids: Vec<u64>,
}

impl IdManager {
    pub fn new() -> Self {
        IdManager {
            next_id: 0,
            recycled_ids: Vec::new(),
        }
    }

    pub fn get_id(&mut self) -> u64 {
        if let Some(id) = self.recycled_ids.pop() {
            id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            id
        }
    }

    pub fn restore_id(&mut self, id: u64) {
        self.recycled_ids.push(id);
    }
}
