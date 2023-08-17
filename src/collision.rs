use crossterm::event::KeyCode;
use crossterm::event::KeyCode::F;
use crate::player::Player;
use crate::{player, tile_set};
use crate::chat::Chat;
use crate::map::MapData;
use crate::map_manager::MapManager;
use crate::tile_set::{DEFAULT_TILE_SET, LADDER_TILE_SET, TileSet};

pub struct CollisionEngine {

}

impl CollisionEngine {
    pub(crate) fn new() -> Self {
        CollisionEngine {}
    }

    pub(crate) fn process_input(&mut self, mut player: &mut Player, map_manager: &mut MapManager, chat: &mut Chat) {
        match player.key_event {
            KeyCode::Left => {
                self.move_player(player, map_manager, chat);
            }
            KeyCode::Right => {
                self.move_player(player, map_manager, chat);
            }
            KeyCode::Up => {
                self.move_player(player, map_manager, chat);
            }
            KeyCode::Down => {
                self.move_player(player, map_manager, chat);
            }
            KeyCode::Esc => {
                chat.process_chat_message("Pressed ESC & Exited the Game");
                player.previous_key_event = KeyCode::Esc;
                player.key_state = true;
            }
            _ => {}
        }
    }

    pub(crate) fn move_player(&mut self, player: &mut Player, map_manager: &mut MapManager, chat: &mut Chat) {
        let map = map_manager.get_map_mut(map_manager.current_map_index);

        if let Some(map_data) = map {
            let (new_row_idx, new_col_idx) = match player.key_event {
                KeyCode::Up => {
                    // Move up
                    chat.process_chat_message("You walk up.");
                    (map_data.player_position.x - 1, map_data.player_position.y)
                },
                KeyCode::Down => {
                    // Move down
                    chat.process_chat_message("You walk down.");
                    (map_data.player_position.x + 1, map_data.player_position.y)
                },
                KeyCode::Left => {
                    // Move left
                    chat.process_chat_message("You walk left.");
                    (map_data.player_position.x, map_data.player_position.y - 1)
                },
                KeyCode::Right => {
                    // Move right
                    chat.process_chat_message("You walk right.");
                    (map_data.player_position.x, map_data.player_position.y + 1)
                },
                _ => (map_data.player_position.x, map_data.player_position.y), // invalid direction, stay in place
            };

            if map_data.tile_set.name == DEFAULT_TILE_SET.name {
                self.process_move(player, map_data, chat,map_data.player_position.x, map_data.player_position.y, new_row_idx, new_col_idx);
            } else if map_data.tile_set.name == LADDER_TILE_SET.name {
                // we're in a ladder scene
                self.process_move_ladder(player, map_data, map_data.player_position.y, new_row_idx, new_row_idx,new_col_idx);
            }

            // update map
            let modules = [player.status.get_status(), player.inventory.get_inventory_to_size(2, format!("FLOOR: {}", map_data.current_floor))];
            map_data.update_str_map_with_modules(&modules);
            //println!("Moved player from row: {}, column: {} to row: {}, column: {}", row_idx, col_idx, new_row_idx, new_col_idx);
        }
    }

    pub(crate) fn process_move(&mut self, player: &mut Player, map_data: &mut MapData, chat: &mut Chat, previous_row_coord: usize, previous_col_coord: usize, new_row_coord: usize, new_col_coord: usize) {
        let mut process_move = false;
        let mut tmp_tile = map_data.map[new_row_coord][new_col_coord];

        if tmp_tile == map_data.tile_set.floor
        {
            process_move = true;
        }
        else if tmp_tile == map_data.tile_set.wall
        {
            process_move = false;
        }
        else if tmp_tile == map_data.tile_set.key
        {
            chat.process_chat_message("You pick up a rusty key.");
            player.inventory.add_key(1);
            map_data.map[new_row_coord][new_col_coord] = map_data.tile_set.floor;
            process_move = false;
        } else if tmp_tile == map_data.tile_set.closed_door_side || tmp_tile == map_data.tile_set.closed_door_top
        {
            if player.inventory.keys >= 1
            {
                player.inventory.remove_key(1);
                chat.process_chat_message("You unlock the door using a rusty key.");
                map_data.map[new_row_coord][new_col_coord] = map_data.tile_set.open_door;
                process_move = false;
            } else {
                chat.process_chat_message("You need a rusty key to open this door.");
                process_move = false;
            }
        } else if tmp_tile == map_data.tile_set.open_door
        {
            process_move = true;
        }

        // @TODO logic for changing scene
        let res = self.check_for_multi_tile(map_data, tmp_tile, new_row_coord, new_col_coord);
        if res == map_data.tile_set.ladder && map_data.tile_set.name == DEFAULT_TILE_SET.name {
            let mut enter_scene_direction = 0;

            if player.key_event == KeyCode::Up {
                enter_scene_direction = 2;
                // @TODO change to use map manager
                //player.map.set_previous_map_data("map2");
                map_data.current_floor += 1;

                // @TODO map_manager handles this now
                //map_data.load_map_set_player_position("scene_ladder", enter_scene_direction, 3);
                map_data.set_map_tile_set(LADDER_TILE_SET);
                process_move = false;
            }
        }

        // set the new player position
        if process_move {
            map_data.map[previous_row_coord][previous_col_coord] = self.update_tile(map_data, tmp_tile, new_row_coord, new_col_coord);
            map_data.map[new_row_coord][new_col_coord] = map_data.tile_set.player;

            map_data.update_player_position();
            map_data.update_tile_below_player(tmp_tile, new_row_coord, new_col_coord);
        }
    }

    fn process_move_ladder(&mut self, mut player: &mut Player, map_data: &mut MapData, previous_row_coord: usize, previous_col_coord: usize, new_row_coord: usize, new_col_coord: usize) {
        let mut process_move = true;
        let mut tmp_tile = map_data.map[new_row_coord][new_col_coord];
        let res = self.check_for_multi_tile(map_data, tmp_tile, new_row_coord, new_col_coord);
        if res == map_data.tile_set.ladder && map_data.tile_set.name == LADDER_TILE_SET.name {
            if player.key_event == KeyCode::Down && new_row_coord == 3 {
                map_data.current_floor -= 1;

                //map_data.load_previous_map();
                map_data.update_player_position();
                map_data.set_map_tile_set(DEFAULT_TILE_SET);
                process_move = false;
            } else if player.key_event == KeyCode::Up && new_row_coord == 0 {

                //map_data.load_map_set_player_position("map1", 3, 3);
                map_data.set_map_tile_set(DEFAULT_TILE_SET);
                process_move = false;
            } else {
                process_move = true;
            }
        }
        
        if process_move {
            map_data.map[previous_row_coord][previous_col_coord] = self.update_tile(map_data, tmp_tile, new_row_coord, new_col_coord);
            map_data.map[new_row_coord][new_col_coord] = map_data.tile_set.player;
            map_data.update_player_position();
        }
    }

    fn check_for_multi_tile(&mut self, map_data: &MapData, tmp_tile: char, current_x: usize, current_y: usize) -> String {

        for (row_idx, row) in map_data.map.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if c == '@' {
                    let mut tile_left = map_data.map[current_x][current_y - 1];
                    let mut tile_right = map_data.map[current_x][current_y + 1];
                    let mut next_tile = format!("{}{}{}", tile_left, tmp_tile, tile_right );

                    //player.chat.process_chat_message(&next_tile);
                    if next_tile == DEFAULT_TILE_SET.ladder {
                        return format!("{}", DEFAULT_TILE_SET.ladder);
                    }
                }
            }
        }

        return "".to_string() // Return value as needed
    }

    fn update_tile(&mut self, map_data: &MapData, mut tmp_tile: char, new_row_coord: usize, new_col_coord: usize) -> char {
        let tile_set = &map_data.tile_set;

        if tmp_tile == tile_set.open_door {
            tmp_tile = tile_set.floor;
        }

        if map_data.tile_below_player == tile_set.open_door {
            tmp_tile = tile_set.open_door;
        }

        if map_data.tile_below_player == tile_set.closed_door_top {
            tmp_tile = tile_set.closed_door_top;
        }
        tmp_tile
    }
}