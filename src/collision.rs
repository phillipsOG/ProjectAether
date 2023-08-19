use std::fmt::format;
use std::iter::Map;
use crossterm::event::KeyCode;
use crate::player::Player;
use crate::chat::Chat;
use crate::map::{MapData, Vec2};
use crate::map_manager::MapManager;
use crate::PlayerMove;
use crate::tile_set::{DEFAULT_TILE_SET, LADDER_TILE_SET};

pub struct CollisionEngine { }

impl CollisionEngine {
    pub(crate) fn new() -> Self {
        CollisionEngine {}
    }

    pub(crate) fn move_player(&mut self, map_manager: &mut MapManager, mut player: &mut Player, chat: &mut Chat) -> Vec2 {
        let map = map_manager.get_map(map_manager.current_map_index);

        if let Some(map_data) = map {
            match player.key_event {
                KeyCode::Up => {
                    // Move up
                    chat.process_chat_message("You walk up.");
                    return Vec2::new(map_data.player_position.x -1, map_data.player_position.y)
                },
                KeyCode::Down => {
                    // Move down
                    chat.process_chat_message("You walk down.");
                    return Vec2::new(map_data.player_position.x +1, map_data.player_position.y)
                },
                KeyCode::Left => {
                    // Move left
                    chat.process_chat_message("You walk left.");
                    return Vec2::new(map_data.player_position.x, map_data.player_position.y -1)
                },
                KeyCode::Right => {
                    // Move right
                    chat.process_chat_message("You walk right.");
                    return Vec2::new(map_data.player_position.x, map_data.player_position.y +1)
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
            let mut tmp_tile = map_data.map[new_player_pos.x][new_player_pos.y].tile;

            let res = self.check_for_multi_tile(map_data, tmp_tile, new_player_pos.x, new_player_pos.y);
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
                if player.key_event == KeyCode::Up && map_data.player_position.x == 1
                {
                    return PlayerMove::LadderEnter;
                }
                else if player.key_event == KeyCode::Down && map_data.player_position.x == 2
                {
                    return PlayerMove::LadderExit;
                }
            }

            if tmp_tile == map_data.tile_set.floor
            {
                return PlayerMove::Normal
            }
            else if tmp_tile == map_data.tile_set.wall
            {
                return PlayerMove::Unable
            }
            else if tmp_tile == map_data.tile_set.key
            {
                chat.process_chat_message("You pick up a rusty key.");
                player.inventory.add_key(1);
                map_data.map[new_player_pos.x][new_player_pos.y].tile = map_data.tile_set.floor;
                return PlayerMove::Unable
            }
            else if tmp_tile == map_data.tile_set.closed_door_side || tmp_tile == map_data.tile_set.closed_door_top
            {
                return if player.inventory.keys >= 1
                {
                    player.inventory.remove_key(1);
                    chat.process_chat_message("You unlock the door using a rusty key.");
                    map_data.map[new_player_pos.x][new_player_pos.y].tile = map_data.tile_set.open_door;
                    PlayerMove::Unable
                } else {
                    chat.process_chat_message("You need a rusty key to open this door.");
                    PlayerMove::Unable
                }
            }
            else if tmp_tile == map_data.tile_set.open_door
            {
                return PlayerMove::Normal
            }
        }
        return PlayerMove::Unable;
    }

    pub(crate) fn update_player_position(&mut self, mut map_manager: &mut MapManager, new_player_position: Vec2) {
        let map = map_manager.get_map_mut(map_manager.current_map_index);
        // set the new player position
        if let Some(map_data) = map {
            let tmp_tile = map_data.map[new_player_position.x][new_player_position.y].tile;
            map_data.map[map_data.player_position.x][map_data.player_position.y].tile = self.update_tile(map_data, tmp_tile);
            map_data.map[new_player_position.x][new_player_position.y].tile = map_data.tile_set.player;

            map_data.map[new_player_position.x+1][new_player_position.y].is_visible = true;
            map_data.map[new_player_position.x-1][new_player_position.y].is_visible = true;
            map_data.map[new_player_position.x][new_player_position.y+1].is_visible = true;
            map_data.map[new_player_position.x][new_player_position.y-1].is_visible = true;
            map_data.map[new_player_position.x+1][new_player_position.y+1].is_visible = true;
            map_data.map[new_player_position.x-1][new_player_position.y+1].is_visible = true;
            map_data.map[new_player_position.x+1][new_player_position.y-1].is_visible = true;
            map_data.map[new_player_position.x-1][new_player_position.y-1].is_visible = true;

            map_data.update_player_position();
            map_data.update_tile_below_player(tmp_tile, new_player_position.x, new_player_position.y);
        }
    }

    pub(crate) fn process_move_ladder(&mut self, mut player: &mut Player, map_manager: &mut MapManager, previous_row_coord: usize, previous_col_coord: usize, new_row_coord: usize, new_col_coord: usize) {
        let mut process_move = true;
        let map = map_manager.get_map_mut(map_manager.current_map_index);
        if let Some(map_data) = map {
            let mut tmp_tile = map_data.map[new_row_coord][new_col_coord].tile;
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
                } else {
                    process_move = true;
                }
            }

            if process_move {
                map_data.map[previous_row_coord][previous_col_coord].tile = self.update_tile(map_data, tmp_tile);
                map_data.map[new_row_coord][new_col_coord].tile = map_data.tile_set.player;
                map_data.update_player_position();
            }
        }
    }

    fn check_for_multi_tile(&mut self, map_data: &MapData, tmp_tile: char, current_x: usize, current_y: usize) -> String {
        for (row_idx, row) in map_data.map.iter().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                if c.tile == '@' {
                    let mut tile_left = map_data.map[current_x][current_y - 1].tile;
                    let mut tile_right = map_data.map[current_x][current_y + 1].tile;
                    let mut next_tile = format!("{}{}{}", tile_left, tmp_tile, tile_right );

                    //player.chat.process_chat_message(&next_tile);
                    if next_tile == DEFAULT_TILE_SET.ladder && map_data.tile_set.name != DEFAULT_TILE_SET.ladder {
                        return format!("{}", DEFAULT_TILE_SET.ladder);
                    }
                }
            }
        }

        return "".to_string() // Return value as needed
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