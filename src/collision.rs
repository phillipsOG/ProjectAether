use std::io;
use std::io::stdin;
use crossterm::event::KeyCode;
use crate::player::Player;
use crate::chat::Chat;
use crate::map_data::{MapData, Vec2};
use crate::map_manager::MapManager;
use crate::PlayerMove;
use crate::tile_set::{DEFAULT_TILE_SET, LADDER_TILE_SET};

pub struct CollisionEngine { }

impl CollisionEngine {
    pub(crate) fn new() -> Self {
        CollisionEngine {

        }
    }

    pub(crate) fn move_player(&mut self, map_manager: &mut MapManager, player: &mut Player, chat: &mut Chat) -> Vec2 {
        let map = map_manager.get_map_mut(map_manager.current_map_index);
        if let Some(map_data) = map {
            match player.key_event {
                KeyCode::Up => {
                    // Move up
                    chat.process_chat_message("You walk up.");
                    return Vec2::new(map_data.player_position.x, map_data.player_position.y-1)
                },
                KeyCode::Down => {
                    // Move down
                    chat.process_chat_message("You walk down.");
                    return Vec2::new(map_data.player_position.x, map_data.player_position.y+1)
                },
                KeyCode::Left => {
                    // Move left
                    chat.process_chat_message("You walk left.");
                    return Vec2::new(map_data.player_position.x-1, map_data.player_position.y)
                },
                KeyCode::Right => {
                    // Move right
                    chat.process_chat_message("You walk right.");
                    return Vec2::new(map_data.player_position.x+1, map_data.player_position.y)
                },
                KeyCode::Tab => {
                    println!("Please enter a command: ");

                    let mut input = String::new();
                    if let Err(_) = io::stdin().read_line(&mut input) {
                        chat.process_chat_message("Error reading command.");
                    }

                    // @TODO turn into actual command system
                    if input.trim() == "nofog" {
                        if map_data.fog_of_war {
                            chat.process_chat_message("Removed fog of war.");
                            map_data.fog_of_war = false;
                        } else {
                            chat.process_chat_message("Added back fog of war.");
                            map_data.fog_of_war = true;
                        }
                    } else {
                        chat.process_chat_message("Invalid command.");
                    }
                },
                KeyCode::Esc => {
                    player.previous_key_event = KeyCode::Esc;
                    player.key_state = true
                }
                _ => {}
            }
        }
        // Don't move
        return Vec2::new(0, 0)
    }

    pub(crate) fn process_move(&mut self, player: &mut Player, map_manager: &mut MapManager, chat: &mut Chat, new_player_pos: Vec2) -> PlayerMove {
        let map = map_manager.get_map_mut(map_manager.current_map_index);

        if let Some(map_data) = map {
            let tmp_tile = map_data.map[new_player_pos.y][new_player_pos.x].tile;
            let is_tile_solid = map_data.map[new_player_pos.y][new_player_pos.x].is_solid;

            if map_data.tile_set.name == LADDER_TILE_SET.name {
                if tmp_tile == LADDER_TILE_SET.closed_door_side {
                    return  PlayerMove::Unable;
                }
            }
            else if is_tile_solid && !(tmp_tile == DEFAULT_TILE_SET.closed_door_side || tmp_tile == DEFAULT_TILE_SET.closed_door_top || tmp_tile == DEFAULT_TILE_SET.open_door) {
                return PlayerMove::Unable
            }

            let res = self.check_for_multi_tile(map_data, tmp_tile, new_player_pos);
            if res == map_data.tile_set.ladder && map_data.tile_set.name == DEFAULT_TILE_SET.name {
                if player.key_event == KeyCode::Up
                {
                    return PlayerMove::LadderUp;
                }
                else if player.key_event == KeyCode::Down
                {
                    return PlayerMove::LadderDown;
                }
            } else if res == map_data.tile_set.ladder && map_data.tile_set.name == LADDER_TILE_SET.name {
                if player.key_event == KeyCode::Up && map_data.player_position.y == 1
                {
                    return PlayerMove::LadderEnter;
                }
                else if player.key_event == KeyCode::Down && map_data.player_position.y == 2
                {
                    return PlayerMove::LadderExit;
                }
            }

            if tmp_tile == map_data.tile_set.key
            {
                chat.process_chat_message("You pick up a rusty key.");
                player.inventory.add_key(1);
                map_data.map[new_player_pos.y][new_player_pos.x].tile = map_data.tile_set.floor;
                return PlayerMove::Unable
            }
            else if tmp_tile == map_data.tile_set.closed_door_side || tmp_tile == map_data.tile_set.closed_door_top
            {
                return if player.inventory.keys >= 1
                {
                    player.inventory.remove_key(1);
                    chat.process_chat_message("You unlock the door using a rusty key.");
                    map_data.map[new_player_pos.y][new_player_pos.x].tile = map_data.tile_set.open_door;
                    PlayerMove::Unable
                } else {
                    chat.process_chat_message("You need a rusty key to open this door.");
                    PlayerMove::Unable
                }
            }
        }
        return PlayerMove::Normal;
    }

    pub(crate) fn update_player_position(&mut self, map_manager: &mut MapManager, new_player_position: Vec2) {
        let map = map_manager.get_map_mut(map_manager.current_map_index);
        if let Some(map_data) = map {
            let tmp_tile = map_data.map[new_player_position.y][new_player_position.x].tile;
            map_data.map[map_data.player_position.y][map_data.player_position.x].tile = self.update_tile(map_data, tmp_tile);
            map_data.map[new_player_position.y][new_player_position.x].tile = map_data.tile_set.player;
            map_data.update_player_position();
            map_data.update_tile_below_player(tmp_tile);
        }
    }

    pub(crate) fn update_player_vision(&mut self, map_manager: &mut MapManager, _new_player_position: Vec2) {
        let map = map_manager.get_map_mut(map_manager.current_map_index);
        if let Some(map_data) = map {
            map_data.set_player_vision(map_data.player_position);
        }
    }

    fn check_for_multi_tile(&mut self, map_data: &MapData, tmp_tile: char, new_player_position: Vec2) -> String {
        for (_col_idx, col) in map_data.map.iter().enumerate() {
            for (_row_idx, c) in col.iter().enumerate() {
                if c.tile == '@' {
                    let y = map_data.player_position.y;
                    let x = map_data.player_position.x;

                    if y > map_data.map_height || x > map_data.map_width || x < 0 || y < 0 {
                        let tile_left = map_data.map[new_player_position.y][new_player_position.x -1].tile;
                        let tile_right = map_data.map[new_player_position.y][new_player_position.x +1].tile;
                        let next_tile = format!("{}{}{}", tile_left, tmp_tile, tile_right );
                        if next_tile == DEFAULT_TILE_SET.ladder && map_data.tile_set.name != DEFAULT_TILE_SET.ladder {
                            return format!("{}", DEFAULT_TILE_SET.ladder);
                        }
                    }
                }
            }
        }

        return "".to_string()
    }

    fn update_tile(&mut self, map_data: &MapData, mut tmp_tile: char) -> char {
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