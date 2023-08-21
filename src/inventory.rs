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

    pub(crate) fn get_inventory_to_size(&mut self, _size: usize, module_part: String) -> [String; 3] {
        let inventory_items = [format!("{}", module_part), format!("Keys: {}", self.keys), String::new()];

        inventory_items
    }
}