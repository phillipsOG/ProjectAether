pub struct Inventory
{
    pub(crate) keys: i32
}

impl Inventory {
    pub(crate) fn new() -> Self {
        Inventory {
            keys: 0,
        }
    }

    pub(crate) fn add_key(&mut self, amount: i32) {
        self.keys += amount;
    }

    pub(crate) fn remove_key(&mut self, amount: i32) {
        self.keys -= amount;
    }

    fn get_inventory(&mut self) -> [String; 1] {
        [
            format!("Keys: {}", self.keys),
        ]
    }

    pub(crate) fn get_inventory_to_size(&mut self, size: usize) -> [String; 3] {
        let mut inventory_items = [format!("Keys: {}", self.keys), String::new(), String::new()];

        for i in 1..size {
            inventory_items[i] = String::new();
        }

        inventory_items
    }
}