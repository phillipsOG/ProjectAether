use crossterm::event::KeyCode;
use crate::player::Player;
use crate::{player, tile_set};
use crate::tile_set::{LADDER_TILE_SET, TileSet};
//use crate::tile_set::TILE_SET;

pub struct CollisionEngine {

}

impl CollisionEngine {
    pub(crate) fn new() -> Self {
        CollisionEngine {

        }
    }

    pub(crate) fn process_input(&mut self, mut player: &mut Player) {
        match player.key_event {
            KeyCode::Left => {
                self.move_player(player);
            }
            KeyCode::Right => {
                self.move_player(player);
            }
            KeyCode::Up => {
                self.move_player(player);
            }
            KeyCode::Down => {
                self.move_player(player);
            }
            KeyCode::Esc => {
                player.chat.process_chat_message("Pressed ESC & Exited the Game");
                player.previous_key_event  = KeyCode::Esc;
                player.key_state = true;
            }
            _ => {}
        }
    }

    pub(crate) fn move_player(&mut self, mut player: &mut Player) {
        if let Some((row_idx, col_idx)) = player.map.player_position {
            let (new_row_idx, new_col_idx) = match player.key_event {
                KeyCode::Up => {
                    // Move up
                    player.chat.process_chat_message("You walk up.");
                    (row_idx - 1, col_idx)
                },
                KeyCode::Down => {
                    // Move down
                    player.chat.process_chat_message("You walk down.");
                    (row_idx + 1, col_idx)
                },
                KeyCode::Left => {
                    // Move left
                    player.chat.process_chat_message("You walk left.");
                    (row_idx, col_idx - 1)
                },
                KeyCode::Right => {
                    // Move right
                    player.chat.process_chat_message("You walk right.");
                    (row_idx, col_idx + 1)
                },
                _ => (row_idx, col_idx), // invalid direction, stay in place
            };

            // basic collision
            self.process_move(player, row_idx, col_idx, new_row_idx, new_col_idx);

            // update map
            let modules = [player.status.get_status(), player.inventory.get_inventory_to_size(2)];
            player.map.update_str_map_with_modules(&modules);
            //println!("Moved player from row: {}, column: {} to row: {}, column: {}", row_idx, col_idx, new_row_idx, new_col_idx);
        } else {
            println!("No '@' symbol found in the map.");
        }
    }

    pub(crate) fn process_move(&mut self, mut player: &mut Player, previous_row_coord: usize, previous_col_coord: usize, new_row_coord: usize, new_col_coord: usize) {
        //let tile_set = &;
        let mut tmp_tile = player.map.map[new_row_coord][new_col_coord];

        let mut process_move = false;

        if tmp_tile ==  player.map.tile_set.floor
        {
            process_move = true;
        }
        else if tmp_tile == player.map.tile_set.wall
        {
            process_move = false;
        }
        else if tmp_tile == player.map.tile_set.key
        {
            player.chat.process_chat_message("You pick up a rusty key.");
            player.inventory.add_key(1);
            player.map.map[new_row_coord][new_col_coord] = player.map.tile_set.floor;
            process_move = false;
        }
        else if tmp_tile == player.map.tile_set.closed_door_side || tmp_tile == player.map.tile_set.closed_door_top
        {
            if player.inventory.keys >= 1
            {
                player.inventory.remove_key(1);
                player.chat.process_chat_message("You unlock the door using a rusty key.");
                player.map.map[new_row_coord][new_col_coord] = player.map.tile_set.open_door;
                process_move = false;
            }
            else
            {
                player.chat.process_chat_message("You need a rusty key to open this door.");
                process_move = false;
            }
        }
        else if tmp_tile == player.map.tile_set.open_door
        {
            process_move = true;
        }

        let mut tile_left = player.map.map[new_row_coord][new_col_coord+1];
        let mut tile_right = player.map.map[new_row_coord][new_col_coord-1];
        let mut ladder = format!("{}{}{}", tile_left, tmp_tile, tile_right );
        if format!("{}{}{}", tile_left, tmp_tile, tile_right ) == player.map.tile_set.ladder {
            // @TODO logic for changing scene
            // update map
            let modules = [player.status.get_status(), player.inventory.get_inventory_to_size(2)];
            let mut enter_scene_direction = 0;

            if player.key_event == KeyCode::Down {
                enter_scene_direction = 0;
            } else {
                enter_scene_direction = 3;
            }
            player.map.load_map_set_player_position("scene_ladder", enter_scene_direction, 3);
            player.map.update_str_map_with_modules(&modules);
            player.map.set_map_tile_set(LADDER_TILE_SET);

            process_move = false;
        }

        if process_move {
            // set the new player position
            player.map.map[new_row_coord][new_col_coord] = player.map.tile_set.player;
            player.map.map[previous_row_coord][previous_col_coord] = self.update_tile(player, tmp_tile, new_row_coord, new_col_coord);
            player.map.update_tile_below_player(tmp_tile, new_row_coord, new_col_coord);
            player.map.update_player_position();
        }
    }

    fn update_tile(&mut self, mut player: &mut Player, mut tmp_tile: char, new_row_coord: usize, new_col_coord: usize) -> char {
        let tile_set = &player.map.tile_set;

        if tmp_tile == tile_set.open_door {
            tmp_tile = tile_set.floor;
        }

        if player.map.tile_below_player == tile_set.open_door {
            tmp_tile = tile_set.open_door;
        }

        if player.map.tile_below_player == tile_set.closed_door_top {
            tmp_tile = tile_set.closed_door_top;
        }

        return tmp_tile;
    }
}