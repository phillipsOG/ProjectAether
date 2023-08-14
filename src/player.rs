use std::io;
use std::io::{stdout, Write};
use crossterm::event::KeyCode;
use crossterm::{QueueableCommand, terminal};
use crate::chat::Chat;
use crate::inventory::Inventory;
use crate::map::Map;
use crate::status::Status;
use crate::tile_set;
use crate::tile_set::TileSet;

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

    pub(crate) fn set_player_position(&mut self, pos_x: usize, pos_y: usize) {
        let tile_set = &tile_set::TILE_SET;

        let mut positions_to_modify = Vec::new();

        for (row_idx, row) in self.map.map.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if row_idx == pos_x && col_idx == pos_y {
                    if self.map.map[row_idx][col_idx] != tile_set.wall && (row_idx == pos_x && col_idx == pos_y) {
                        positions_to_modify.push((pos_x, pos_y));
                    } else if self.map.map[row_idx+1][col_idx+1] != tile_set.wall && self.map.map[row_idx+1][col_idx+1] != tile_set.closed_door_side {
                        positions_to_modify.push((pos_x+1, pos_y+1));
                    }
                }
            }
        }

        for (mod_pos_x, mod_pos_y) in positions_to_modify {
            self.player_position = Some((mod_pos_x, mod_pos_y));
            self.map.map[mod_pos_x][mod_pos_y] = tile_set.player;
        }
    }

    /*pub(crate) fn set_player_position_for_scene(&mut self, ) {
        let tile_set = &tile_set::TILE_SET;

        let mut positions_to_modify = Vec::new();

        for (row_idx, row) in self.map.map.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if row_idx == pos_x && col_idx == pos_y {
                    if self.map.map[row_idx][col_idx] != tile_set.wall && (row_idx == pos_x && col_idx == pos_y) {
                        positions_to_modify.push((pos_x, pos_y));
                    } else if self.map.map[row_idx+1][col_idx+1] != tile_set.wall && self.map.map[row_idx+1][col_idx+1] != tile_set.closed_door_side {
                        positions_to_modify.push((pos_x+1, pos_y+1));
                    }
                }
            }
        }

        for (mod_pos_x, mod_pos_y) in positions_to_modify {
            self.player_position = Some((mod_pos_x, mod_pos_y));
            self.map.map[mod_pos_x][mod_pos_y] = tile_set.player;
        }
    }*/

    pub(crate) fn set_player_position_based_scene(&mut self, pos_x: usize, pos_y: usize) {
        let at_position: Option<(usize, usize)> = None;

        for (row_idx, row) in self.map.map.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if row_idx == pos_x && col_idx == pos_y {
                    self.player_position = Option::from((row_idx, col_idx));
                }
            }
        }
    }

    pub(crate) fn print_terminal(&mut self) {
        //self.map.print_map_with_module(&self.status.get_status());

        let mut stdout = stdout();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        println!("{}", self.map.str_map);

        self.chat.print_chat();
    }
}