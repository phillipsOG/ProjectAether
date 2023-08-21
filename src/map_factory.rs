use std::io::BufRead;
use crate::chat::Chat;
use crate::Map;
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

    pub(crate) fn generate_map(&mut self, height: usize, width: usize, pos_y: usize, pos_x: usize) -> MapData {
        let mut map = "".to_owned();

        for pos_y in 1..height+1 {
            for pos_x in 1..width+1 {
                if pos_y == 1 && pos_x > 0
                    || pos_y > 0 && pos_x == 1
                    /*|| pos_x == width*/
                    || pos_y == height  {

                    //map += &*format!("{}", DEFAULT_TILE_SET.wall);
                } else {
                    map += &*format!("{}", DEFAULT_TILE_SET.floor);
                }
            }

            map += &*format!("\n");
        }

        let map_lines: Vec<&str> = map.trim().lines().collect();
        let mut new_map = MapData::new();
        new_map.map = map_lines
            .iter()
            .map(|line| line.chars().map(Space::from_char).collect())
            .collect();
        new_map.set_player_position(pos_y, pos_x);
        new_map.tile_below_player = DEFAULT_TILE_SET.floor;
        new_map.map_width = new_map.map.len();
        new_map.map_height = if new_map.map_width > 0 { new_map.map[0].len() } else { 0 };
        new_map.set_player_vision(Vec2::new(pos_y, pos_x));

        return new_map;
    }

    pub(crate) fn generate_terrain(&mut self, map_manager: &mut MapManager, new_player_position: Vec2, chat: &mut Chat) -> Map {
        let mut map = map_manager.get_map_mut(map_manager.current_map_index);
        let mut u_m_d = Map::new();

        if let Some (map_data) = map {

            let mut updated_map_data = vec![vec![Space::new('.'); map_data.map_width+30]; map_data.map_height+30];

            chat.clear_chat();
            chat.process_chat_message(&format!("y: {}, x: {}", map_data.player_position.y, map_data.player_position.x));

            for (pos_y, row) in map_data.map.iter().enumerate() {
                for (pos_x, _space) in row.iter().enumerate() {
                    updated_map_data[pos_y][pos_x].tile = _space.tile;
                    updated_map_data[pos_y][pos_x].is_solid = _space.is_solid;
                    updated_map_data[pos_y][pos_x].is_visible = _space.is_visible;
                }
            }
            let mut new_tile = Space::new('.');
            new_tile.is_solid = false;
            new_tile.is_visible = true;

            updated_map_data[new_player_position.y+1][new_player_position.x] = new_tile;
            updated_map_data[new_player_position.y][new_player_position.x+1] = new_tile;

            if new_player_position.x < map_data.map_width {
                //chat.clear_chat();
                chat.process_chat_message("new land bound");
            } /*else if new_player_position.y <= map_data.map_height {
                chat.process_chat_message("new land bound");
                updated_map_data[new_player_position.y][new_player_position.x+1] = new_tile;
                updated_map_data[new_player_position.y][new_player_position.x-2] = new_tile;
            }*/
            u_m_d = updated_map_data;
        }
        return u_m_d;
    }
}