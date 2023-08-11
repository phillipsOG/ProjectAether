use std::thread::sleep;
use crossterm::event::KeyCode;
use crate::chat::Chat;
use crate::collision::Collision;
use crate::inventory::Inventory;
use crate::map::Map;
use crate::status::Status;

pub struct Player
{
    pub key_event: KeyCode,
    pub previous_key_event: KeyCode,
    pub key_state: bool,
    pub player_position: Option<(usize, usize)>,
    pub chat: Chat,
    pub map: Map,
    pub inventory: Inventory,
    pub status: Status
}

impl Player {
    pub(crate) fn new() -> Self {
        Player {
            key_event: KeyCode::Enter,
            previous_key_event: KeyCode::Null,
            key_state: false,
            player_position: None,
            chat: Chat::new(),
            map: Map::new(),
            inventory: Inventory::new(),
            status: Status::new()
        }
    }

    pub(crate) fn process_input(&mut self) {

        match self.key_event {
            KeyCode::Left => {
                self.move_player();
            }
            KeyCode::Right => {
                self.move_player();
            }
            KeyCode::Up => {
                self.move_player();
            }
            KeyCode::Down => {
                self.move_player();
            }
            KeyCode::Esc => {
                self.chat.process_chat_message("Pressed ESC & Exited the Game");
                self.previous_key_event = KeyCode::Esc;
                self.key_state = true;
            }
            _ => {}
        }
    }

    pub(crate) fn move_player(&mut self) {
        if let Some((row_idx, col_idx)) = self.player_position {
            let (new_row_idx, new_col_idx) = match self.key_event {
                KeyCode::Up => {
                    // Move up
                    self.chat.process_chat_message("You walk up.");
                    (row_idx - 1, col_idx)
                },
                KeyCode::Down => {
                    // Move down
                    self.chat.process_chat_message("You walk down.");
                    (row_idx + 1, col_idx)
                },
                KeyCode::Left => {
                    // Move left
                    self.chat.process_chat_message("You walk left.");
                    (row_idx, col_idx - 1)
                },
                KeyCode::Right => {
                    // Move right
                    self.chat.process_chat_message("You walk right.");
                    (row_idx, col_idx + 1)
                },
                _ => (row_idx, col_idx), // invalid direction, stay in place
            };

            // update the player position
            //let move_from = self.map[row_idx][col_idx];
            let move_to = self.map.map[new_row_idx][new_col_idx];

            // basic collision
            // @TODO create fn to handle the collision of other objects
            if self.process_move(move_to) {
                // set the new player position
                self.map.map[new_row_idx][new_col_idx] = '@';

                //set previous player tile
                self.map.map[row_idx][col_idx] = self.map.tile_set.floor;

                self.update_player_position();
            }
            //println!("Moved player from row: {}, column: {} to row: {}, column: {}", row_idx, col_idx, new_row_idx, new_col_idx);
        } else {
            println!("No '@' symbol found in the map.");
        }
    }

    pub(crate) fn update_player_position(&mut self) {
        let at_position: Option<(usize, usize)> = None;

        for (row_idx, row) in self.map.map.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if c == '@' {
                    self.player_position = Option::from((row_idx, col_idx));
                    break;
                }
            }
            if at_position.is_some() {
                break;
            }
        }
    }

    pub(crate) fn process_move(&mut self, move_to_tile: char) -> bool {
        if move_to_tile == self.map.tile_set.floor {
            return true;
        } else if move_to_tile == self.map.tile_set.wall {
            return false;
        } else if move_to_tile == self.map.tile_set.key {
            self.chat.process_chat_message("You pick up a rusty key.");
            self.inventory.add_key(1);
            return true;
        } else if move_to_tile == self.map.tile_set.door {
            return if self.inventory.keys >= 1 {
                self.inventory.remove_key(1);
                self.chat.process_chat_message("You unlock the door using a rusty key.");
                true
            } else {
                self.chat.process_chat_message("You need a rusty key to open this door.");
                false
            }
        }
        false
    }

    pub(crate) fn print_terminal(&mut self) {
        //self.map.print_map_with_module(&self.status.get_status());
        let modules = [self.status.get_status(), self.inventory.get_inventory_to_size(2)];
        self.map.print_map_with_modules(&modules);
        self.chat.print_chat();
    }
}