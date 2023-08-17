use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::map::{MapData, Vec2};

pub struct MapManager {
    maps: HashMap<usize, MapData>,
    pub current_map_index: usize,
    pub should_transition: bool,
    pub target_map: String,
    pub target_position: Vec2
}

impl MapManager {
    pub(crate) fn new() -> Self {
        Self {
            maps: HashMap::new(),
            current_map_index: 0,
            should_transition: false,
            target_map: String::new(),
            target_position: Vec2::ZERO
        }
    }

    pub(crate) fn add_map(&mut self, map_index: usize, map: MapData) {
        self.maps.insert(map_index, map);
    }

    pub(crate) fn get_map_mut(&mut self, map_index: usize) -> Option<&mut MapData> {
        self.maps.get_mut(&map_index)
    }

    pub(crate) fn get_map(&self, map_index: usize) -> Option<&MapData> {
        self.maps.get(&map_index)
    }

    pub(crate) fn load_map_set_player_position(&mut self, map_name: &str, pos_x: usize, pos_y: usize) {
        let mut map = "".to_owned();
        let map_name = format!("src/maps/{}.txt", map_name);

        map += "\n";
        if let Ok(lines) = self.read_lines(&map_name) {
            for line in lines {
                if let Ok(tile) = line {
                    map += &tile;
                    map += "\n";
                }
            }
        }
        let map_lines: Vec<&str> = map.trim().lines().collect();
        let mut new_map = MapData::new();
        new_map.map = map_lines.iter().map(|line| line.chars().collect()).collect();
        new_map.set_player_position(pos_x, pos_y);
        self.add_map(self.current_map_index, new_map);
    }

    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}