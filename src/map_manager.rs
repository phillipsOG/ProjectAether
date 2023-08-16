use std::collections::HashMap;

type Map = Vec<Vec<char>>;

struct MapManager {
    maps: HashMap<usize, Map>,
}

impl MapManager {
    fn new() -> Self {
        Self {
            maps: HashMap::new(),
        }
    }

    fn add_map(&mut self, floor: usize, map: Map) {
        self.maps.insert(floor, map);
    }

    fn get_map(&self, floor: usize) -> Option<&Map> {
        self.maps.get(&floor)
    }
}
/*
fn main() {
    let mut map_manager = MapManager::new();

    // Access and print a specific map
    let selected_floor = 1; // Replace with the desired floor number
    if let Some(map) = map_manager.get_map(selected_floor) {
        for row in map {
            println!("{}", row.iter().collect::<String>());
        }
    } else {
        println!("Map not found for floor {}", selected_floor);
    }
}*/
