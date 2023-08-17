use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::map::{MapData, Vec2};

pub struct MapManager {
    maps: HashMap<usize, MapData>,
    pub current_map_index: usize
}

impl MapManager {
    pub(crate) fn new() -> Self {
        Self {
            maps: HashMap::new(),
            current_map_index: 0,
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

        // 2D rep of our ascii map
        let map_2d = self.get_map_mut(self.current_map_index);
        if let Some(map_data) = map_2d {
            map_data.map = map_lines.iter().map(|line| line.chars().collect()).collect();
            self.set_player_position(pos_x, pos_y);
        }
    }

    pub(crate) fn set_player_position(&mut self, pos_x: usize, pos_y: usize) {
        let mut map = self.get_map_mut(self.current_map_index);

        let mut positions_to_modify = Vec::new();
        if let Some(map_data) = map {
            let tile_set = &map_data.tile_set;

            for (row_idx, row) in map_data.map.iter().enumerate() {
                for (col_idx, &c) in row.iter().enumerate() {
                    if row_idx == pos_x && col_idx == pos_y {
                        if map_data.map[row_idx][col_idx] != tile_set.wall && (row_idx == pos_x && col_idx == pos_y) {
                            positions_to_modify.push((pos_x, pos_y));
                        } else if map_data.map[row_idx+1][col_idx+1] != tile_set.wall && map_data.map[row_idx+1][col_idx+1] != tile_set.closed_door_side {
                            positions_to_modify.push((pos_x+1, pos_y+1));
                        }
                    }
                }
            }

            map_data.player_position = Vec2::new(pos_x, pos_y);
            map_data.map[pos_x][pos_y] = tile_set.player;
        }
    }

    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}