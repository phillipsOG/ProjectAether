use crossterm::event::KeyCode;
use crate::player::Player;

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
                player.previous_key_event = KeyCode::Esc;
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

            // update the player position
            //let move_from = self.map[row_idx][col_idx];
            let move_to = player.map.map[new_row_idx][new_col_idx];

            // basic collision
            // @TODO create fn to handle the collision of other objects
            if self.process_move(player, move_to) {
                // set the new player position
                player.map.map[new_row_idx][new_col_idx] = '@';

                //set previous player tile
                player.map.map[row_idx][col_idx] = player.map.tile_set.floor;

                self.update_player_position(player);
            }
            //println!("Moved player from row: {}, column: {} to row: {}, column: {}", row_idx, col_idx, new_row_idx, new_col_idx);
        } else {
            println!("No '@' symbol found in the map.");
        }
    }

    pub(crate) fn process_move(&mut self, mut player: &mut Player, move_to_tile: char) -> bool {
        if move_to_tile == player.map.tile_set.floor {
            return true;
        } else if move_to_tile == player.map.tile_set.wall {
            return false;
        } else if move_to_tile == player.map.tile_set.key {
            player.chat.process_chat_message("You pick up a rusty key.");
            player.inventory.add_key(1);
            return true;
        } else if move_to_tile == player.map.tile_set.door {
            return if player.inventory.keys >= 1 {
                player.inventory.remove_key(1);
                player.chat.process_chat_message("You unlock the door using a rusty key.");
                true
            } else {
                player.chat.process_chat_message("You need a rusty key to open this door.");
                false
            }
        }
        false
    }

    pub(crate) fn update_player_position(&mut self, mut player: &mut Player) {
        let at_position: Option<(usize, usize)> = None;

        for (row_idx, row) in player.map.map.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if c == '@' {
                    player.player_position = Option::from((row_idx, col_idx));
                    break;
                }
            }
            if at_position.is_some() {
                break;
            }
        }
    }
}