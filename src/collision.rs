use crossterm::event::KeyCode;
use crate::player::Player;
use crate::tile_set;
use crate::tile_set::TILE_SET;

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
        if let Some((row_idx, col_idx)) = player.player_position {
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

            //println!("Moved player from row: {}, column: {} to row: {}, column: {}", row_idx, col_idx, new_row_idx, new_col_idx);
        } else {
            println!("No '@' symbol found in the map.");
        }
    }

    pub(crate) fn process_move(&mut self, mut player: &mut Player, previous_row_coord: usize, previous_col_coord: usize, new_row_coord: usize, new_col_coord: usize) {
        let tile_set = &tile_set::TILE_SET;
        let mut tmp_tile = player.map.map[new_row_coord][new_col_coord];

        /*
        let msg = format!("{}", tmp_tile);
        player.chat.process_chat_message(&msg);

        let prev_tile = player.map.map[previous_row_coord][previous_col_coord];
        let msg_ii = format!("{}", prev_tile);
        player.chat.process_chat_message(&msg_ii);*/

        let mut process_move = false;

        if tmp_tile ==  tile_set.floor
        {
            process_move = true;
        }
        else if tmp_tile == tile_set.wall
        {
            process_move = false;
        }
        else if tmp_tile == tile_set.key
        {
            player.chat.process_chat_message("You pick up a rusty key.");
            player.inventory.add_key(1);
            player.map.map[new_row_coord][new_col_coord] = tile_set.floor;
            process_move = false;
        }
        else if tmp_tile == tile_set.closed_door
        {
            if player.inventory.keys >= 1
            {
                player.inventory.remove_key(1);
                player.chat.process_chat_message("You unlock the door using a rusty key.");
                player.map.map[new_row_coord][new_col_coord] = tile_set.open_door;
                process_move = false;
            }
            else
            {
                player.chat.process_chat_message("You need a rusty key to open this door.");
                process_move = false;
            }
        }
        else if tmp_tile == tile_set.open_door
        {
            process_move = true;
        }

        /*let mut tile_left = player.map.map[new_row_coord -1][new_col_coord];
        let mut tile_right = player.map.map[new_row_coord +1][new_col_coord];
        let mut ladder = format!("{}{}{}", tile_left, new_tile, tile_right );
        player.chat.process_chat_message(&ladder);
        if format!("{}{}{}", tile_left, new_tile, tile_right ) == player.map.tile_set.ladder {
            player.chat.process_chat_message("Ladder?");
            process_move = false;
        }*/

        if process_move {
            // set the new player position
            player.map.map[new_row_coord][new_col_coord] = TILE_SET.player;
            player.map.map[previous_row_coord][previous_col_coord] = self.update_tile(player, tmp_tile, new_row_coord, new_col_coord);
            player.map.update_tile_below_player(tmp_tile, new_row_coord, new_col_coord);
            player.update_player_position();
        }
    }

    fn update_tile(&mut self, mut player: &mut Player, mut tmp_tile: char, new_row_coord: usize, new_col_coord: usize) -> char {
        let tile_set = &tile_set::TILE_SET;

        if tmp_tile == tile_set.open_door {
            tmp_tile = tile_set.floor;
        }

        if player.map.tile_below_player == tile_set.open_door {
            tmp_tile = tile_set.open_door;
        }

        return tmp_tile;
    }
}