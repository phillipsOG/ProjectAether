use std::io::BufRead;
use crate::chat::Chat;
use crate::{Map, TerrainData};
use crate::tile_set::{DEFAULT_TILE_SET};
use crate::map_data::{MapData, Vec2};
use crate::map_manager::MapManager;
use crate::space::Space;

pub struct MapFactory {
}

impl MapFactory {
    pub(crate) fn new() -> Self {
        MapFactory {
        }
    }

    pub(crate) fn generate_map(&mut self, height: usize, width: usize, pos: Vec2) -> MapData {
        let mut map = "".to_owned();

        // small bug with the way
        for pos_y in 1..height+1 {
            for pos_x in 1..width+1 {
                if pos_y == 0 && pos_x > 0
                    || pos_y > 0 && pos_x == 0
                    /*|| pos_x == width*/
                    || pos_y == height  {

                    //map += &*format!("{}", DEFAULT_TILE_SET.wall);
                } else {

                }
                map += &*format!("{}", DEFAULT_TILE_SET.floor);
            }

            map += &*format!("\n");
        }

        let map_lines: Vec<&str> = map.trim().lines().collect();
        let mut new_map = MapData::new();
        new_map.map = map_lines
            .iter()
            .map(|line| line.chars().map(Space::from_char).collect())
            .collect();
        new_map.set_player_position(pos);
        new_map.tile_below_player = DEFAULT_TILE_SET.floor;
        new_map.map_height = new_map.map.len();
        new_map.map_width = if new_map.map_height > 0 { new_map.map[0].len() } else { 0 };

        new_map.set_player_vision(pos);

        return new_map;
    }

    pub(crate) fn generate_terrain(&mut self, map_manager: &mut MapManager, new_player_position: Vec2, chat: &mut Chat) -> TerrainData {
        let mut map = map_manager.get_map_mut(map_manager.current_map_index);
        let mut terrain_data = TerrainData::new();

        if let Some (map_data) = map {
            terrain_data.width_increase = 1;
            terrain_data.height_increase = 1;

            if new_player_position.x >= map_data.map_width - 1 {
                terrain_data.width_increase = 1;

            } else if new_player_position.y >= map_data.map_height - 1 {
                terrain_data.height_increase = 1;

            } else {
                return terrain_data;
            }

            let mut updated_map_data = self.get_new_map_size(new_player_position, map_data.map_height, map_data.map_width);

            for (pos_y, row) in map_data.map.iter().enumerate() {
                for (pos_x, _space) in row.iter().enumerate() {
                    if pos_y >= map_data.map_height -1 {
                        let mut new_tile = Space::new('.');
                        new_tile.is_visible = false;
                        updated_map_data[pos_y][pos_x] = new_tile;
                    } else {
                        updated_map_data[pos_y][pos_x] = _space.clone();
                    }
                }
            }

            chat.clear_chat();
            chat.process_chat_message(&format!("y: {}, x: {}", map_data.player_position.y, map_data.player_position.x));
            chat.process_chat_message(&format!("map_width: {}, map_height: {}", map_data.map_width, map_data.map_height));
            chat.process_chat_message("new land bound x");

            terrain_data.map = updated_map_data;

            return terrain_data;
        }
        return terrain_data;
    }

    fn get_new_map_size(&self, new_player_position: Vec2, map_height: usize, map_width: usize) -> Vec<Vec<Space>> {

        if new_player_position.y >= map_height - 1 {
            return vec![vec![Space::new('.'); map_width]; map_height+1];
        } else if new_player_position.x >= map_width - 1 {
            return vec![vec![Space::new('.'); map_width+1]; map_height];
        }

        let mut new_map_data = vec![vec![Space::new('.'); map_width]; map_height];
        new_map_data
    }
}