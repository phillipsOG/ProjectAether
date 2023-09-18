use crate::map_data::MapData;

use crate::MovementType;
use crate::Vec2;

use crate::chat::Chat;
use crate::terrain_data::TerrainData;
use futures::lock::Mutex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct MapManager {
    maps: HashMap<usize, MapData>,
    pub current_map_index: usize,
    pub should_transition: bool,
    pub target_map: String,
    pub target_position: Vec2,
}

impl MapManager {
    pub(crate) fn new() -> Self {
        Self {
            maps: HashMap::new(),
            current_map_index: 0,
            should_transition: false,
            target_map: String::new(),
            target_position: Vec2::ZERO,
        }
    }

    pub(crate) fn add_map(&mut self, map_index: usize, map: MapData) {
        self.maps.insert(map_index, map);
    }

    /*pub(crate) fn get_map_mut(&mut self, map_index: usize) -> Option<&mut MapData> {
        self.maps.get_mut(&map_index)
    }*/

    pub(crate) fn get_map_mut(&mut self, map_index: usize) -> Option<&mut MapData> {
        self.maps.get_mut(&map_index)
    }

    /*pub(crate) fn get_map_mut<'a>(&'a mut self, map_index: usize) -> Option<&'a mut MapData> {
        self.maps.get_mut(&map_index)
    }*/

    pub(crate) fn get_map(&self, map_index: usize) -> Option<&MapData> {
        self.maps.get(&map_index)
    }

    pub(crate) fn update_current_map(
        &mut self,
        terrain_data: TerrainData,
        _chat_clone: &mut Arc<Mutex<Chat>>,
    ) {
        let map_index = self.current_map_index;
        let map = self.get_map_mut(map_index).expect("map data");
        map.map = terrain_data.map;
        map.height += terrain_data.height_increase;
        map.width += terrain_data.width_increase;
    }

    /*pub(crate) fn add_map_set_player_position(
        &mut self,
        player: &mut Player,
        map_name: &str,
        pos: Vec2,
    ) {
        let mut map = "".to_owned();
        let full_map_name = format!("src/maps/{}.txt", map_name);

        map += "\n";
        if let Ok(lines) = self.read_lines(&full_map_name) {
            for line in lines {
                if let Ok(tile) = line {
                    map += &tile;
                    map += "\n";
                }
            }
        }
        let map_lines: Vec<&str> = map.trim().lines().collect();
        let mut new_map = MapData::new();
        new_map.map = map_lines
            .iter()
            .map(|line| line.chars().map(Space::new).collect())
            .collect();

        player.position = pos;
        player.tile_below_player = DEFAULT_TILE_SET.floor;
        new_map.set_player_position(pos);

        new_map.height = new_map.map.len();
        new_map.width = if new_map.height > 0 {
            new_map.map[0].len()
        } else {
            0
        };

        if map_name == "scene_ladder" {
            new_map.tile_set = LADDER_TILE_SET;
        } else {
            new_map.tile_set = DEFAULT_TILE_SET;
        }

        self.add_map(self.current_map_index, new_map);
        self.current_map_index += 1;
    }*/

    pub(crate) fn add_generated_map(&mut self, generated_map: MapData) {
        self.add_map(self.current_map_index, generated_map);
        self.current_map_index += 1;
    }

    pub(crate) fn get_mut_current_map(&mut self) -> &mut MapData {
        self.get_map_mut(self.current_map_index).expect("map data")
    }

    pub(crate) fn get_mut_current_map_items(&mut self) -> &mut MapData {
        self.get_map_mut(self.current_map_index).expect("map data")
    }

    pub(crate) fn load_map(
        &mut self,
        map_name: &str,
        _player_move: MovementType,
    ) -> Option<&mut MapData> {
        if map_name == "gen_map" {
            self.current_map_index = 0;
        } else if map_name == "map1" {
            self.current_map_index = 1;
        } else if map_name == "test" {
            self.current_map_index = 2;
        } else if map_name == "test_map" {
            self.current_map_index = 3;
        } else if map_name == "scene_ladder" {
            self.current_map_index = 4;
        } else if map_name == "map3" {
            self.current_map_index = 5;
        }

        self.get_map_mut(self.current_map_index)
    }

    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
