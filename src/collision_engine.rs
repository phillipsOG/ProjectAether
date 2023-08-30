use std::collections::HashMap;

use crate::chat::Chat;
use crate::map_data::MapData;

use crate::player::Player;
use crate::tile_set::{DEFAULT_TILE_SET, LADDER_TILE_SET, MONSTER_TILE_SET};
use crate::{MovementType};
use crossterm::event::KeyCode;


use std::io;
use std::sync::Arc;
use futures::lock::{Mutex, MutexGuard};

use crate::monster_manager::MonsterManager;
use crate::space::Space;
use crate::Vec2;

#[derive(Clone)]
pub struct CollisionEngine {}

impl CollisionEngine {
    pub(crate) fn new() -> Self {
        CollisionEngine {}
    }

    pub(crate) fn move_player(
        &mut self,
        player: &Player,
        chat: &mut Chat,
    ) -> Vec2 {
        let mut current_position = Vec2::ZERO;
        match player.key_event {
            KeyCode::Up => {
                // Move up
                chat.process_chat_message("You walk up.");
                return Vec2::new(player.player_position.x, player.player_position.y - 1);
            }
            KeyCode::Down => {
                // Move down
                chat.process_chat_message("You walk down.");
                return Vec2::new(player.player_position.x, player.player_position.y + 1);
            }
            KeyCode::Left => {
                // Move left
                chat.process_chat_message("You walk left.");
                return Vec2::new(player.player_position.x - 1, player.player_position.y);
            }
            KeyCode::Right => {
                // Move right
                chat.process_chat_message("You walk right.");
                return Vec2::new(player.player_position.x + 1, player.player_position.y);
            }
            KeyCode::Tab => {
                println!("Please enter a command: ");

                let mut input = String::new();
                if let Err(_) = io::stdin().read_line(&mut input) {
                    chat.process_chat_message("Error reading command.");
                }
                // @TODO turn into actual command system
                if input.trim() == "nofog" {
                    if player.fog_of_war {
                        chat.process_chat_message("Removed fog of war.");
                        //player.fog_of_war = false;
                    } else {
                        chat.process_chat_message("Added back fog of war.");
                        //player.fog_of_war = true;
                    }
                } else {
                    chat.process_chat_message("Invalid command.");
                }
            }

            _ => {}
        }
        current_position = player.player_position;

        // Don't move
        return current_position;
    }

    pub(crate) fn process_move(
        &mut self,
        map_data: &MapData,
        player: &mut Player,
        chat: &mut Chat,
        new_player_pos: Vec2,
    ) -> MovementType {
        /*if let Some(map_data) = map_data_option {*/
            let tmp_tile = map_data.map[new_player_pos.y][new_player_pos.x].tile;
            let is_tile_solid = map_data.map[new_player_pos.y][new_player_pos.x].is_solid;
            let is_tile_traversable = map_data.map[new_player_pos.y][new_player_pos.x].is_traversable;
            let tile_set = map_data.tile_set.clone();

            let res = self.check_for_multi_tile(map_data.clone(), tmp_tile, new_player_pos);

            if res == tile_set.ladder && tile_set.name == DEFAULT_TILE_SET.name {
                if player.key_event == KeyCode::Up {
                    return MovementType::LadderUp;
                } else if player.key_event == KeyCode::Down {
                    return MovementType::LadderDown;
                }
            } else if res == tile_set.ladder && tile_set.name == LADDER_TILE_SET.name
            {
                if player.key_event == KeyCode::Up && player.player_position.y == 1 {
                    return MovementType::LadderEnter;
                } else if player.key_event == KeyCode::Down && player.player_position.y == 2 {
                    return MovementType::LadderExit;
                }
            }

            if tmp_tile == tile_set.key {
                chat.process_chat_message("You pick up a rusty key.");
                player.inventory.add_key(1);
                //map_data.map[new_player_pos.y][new_player_pos.x] = Space::new(DEFAULT_TILE_SET.floor);
            } else if tmp_tile == tile_set.closed_door_side
                || tmp_tile == tile_set.closed_door_top
            {
                if player.inventory.keys >= 1 {
                    player.inventory.remove_key(1);
                    chat.process_chat_message("You unlock the door using a rusty key.");
                    /*map_data.map[new_player_pos.y][new_player_pos.x] =
                        Space::new(DEFAULT_TILE_SET.open_door);*/
                } else {
                    chat.process_chat_message("You need a rusty key to open this door.");
                };
            }

            if tile_set.name == DEFAULT_TILE_SET.name {
                if !is_tile_traversable {
                    return MovementType::Unable;
                }
                if !is_tile_solid {
                    return MovementType::Normal;
                }
            } else if tile_set.name == LADDER_TILE_SET.name {
                if is_tile_traversable {
                    return MovementType::Normal;
                }
            /*}*/
        }
        return MovementType::Unable;
    }

    pub(crate) fn update_player_position(
        &mut self,
        map_data: &mut MapData,
        player: &mut Player,
        new_player_position: Vec2,
    ) {
        let tmp_tile = map_data.map[new_player_position.y][new_player_position.x].tile;
        let pos = player.player_position.clone();
        map_data.map[pos.y][pos.x] =
            Space::new(self.update_player_previous_tile(player, tmp_tile));
        player.player_position = new_player_position;
        player.tile_below_player = tmp_tile;
        map_data.set_player_position(new_player_position);
        player.update_tile_below_player(tmp_tile);
    }

    pub(crate) fn update_player_vision(
        &mut self,
        map_data: &mut MapData,
        player: &MutexGuard<Player>,
        _new_player_position: Vec2,
    ) {
        map_data.set_player_vision(player, _new_player_position);
    }

    fn check_for_multi_tile(
        &mut self,
        map_data: MapData,
        tmp_tile: char,
        new_player_position: Vec2,
    ) -> String {
        for (_col_idx, col) in map_data.map.iter().enumerate() {
            for (_row_idx, c) in col.iter().enumerate() {
                if c.tile == DEFAULT_TILE_SET.player {
                    // if the players position x is greater than available x pos then don't check code
                    if new_player_position.x > 0 {
                        let tile_left = map_data.map[new_player_position.y]
                            .get(new_player_position.x - 1)
                            .map(|space| space.tile)
                            .unwrap_or_default();
                        let tile_right = map_data.map[new_player_position.y]
                            .get(new_player_position.x + 1)
                            .map(|space| space.tile)
                            .unwrap_or_default();
                        let next_tile = format!("{}{}{}", tile_left, tmp_tile, tile_right);
                        if next_tile == DEFAULT_TILE_SET.ladder
                            && map_data.tile_set.name != DEFAULT_TILE_SET.ladder
                        {
                            return format!("{}", DEFAULT_TILE_SET.ladder);
                        }
                    }
                }
            }
        }

        return "".to_string();
    }

    pub(crate) fn move_monsters(
        &mut self,
        player: &Player,
        monster_manager: &mut MonsterManager,
    ) -> HashMap<i32, Vec2> {
        let monsters = monster_manager.get_monsters_mut();
        let mut new_monsters_position = HashMap::<i32, Vec2>::new();

        for mon_index in 0..monsters.len() {
            let monster = monsters.get_mut(mon_index);
            if let Some(m_data) = monster {
                let cur_monster_pos = m_data.position;
                let mut new_pos = Vec2::ZERO;

                if cur_monster_pos.x < player.player_position.x {
                    new_pos = Vec2::new(cur_monster_pos.x + 1, cur_monster_pos.y);
                } else if cur_monster_pos.x > player.player_position.x {
                    new_pos = Vec2::new(cur_monster_pos.x - 1, cur_monster_pos.y);
                } else if cur_monster_pos.y < player.player_position.y {
                    new_pos = Vec2::new(cur_monster_pos.x, cur_monster_pos.y + 1);
                } else if cur_monster_pos.y > player.player_position.y {
                    new_pos = Vec2::new(cur_monster_pos.x, cur_monster_pos.y - 1);
                }
                new_monsters_position.insert(m_data.id, new_pos);
            }
        }

        new_monsters_position
    }

    pub(crate) fn process_monsters_move(
        &mut self,
        new_monsters_position: &mut HashMap<i32, Vec2>,
        map_data: &MapData,
        monster_manager: &mut MutexGuard<MonsterManager>,
    ) -> HashMap<i32, Vec2> {
        let monsters = monster_manager.get_monsters_mut();
        let mut processed_monsters_move = HashMap::<i32, Vec2>::new();

        for index in 0..monsters.len() {
            let monster = monsters.get_mut(index).unwrap();

            if monster.tile == MONSTER_TILE_SET.snake {
                let new_enemy_pos = new_monsters_position[&monster.id];

                let is_tile_solid = map_data.map[new_enemy_pos.y][new_enemy_pos.x].is_solid;
                let is_tile_traversable =
                    map_data.map[new_enemy_pos.y][new_enemy_pos.x].is_traversable;

                if !is_tile_traversable {
                    continue;
                }

                if is_tile_solid {
                    continue;
                }

                processed_monsters_move.insert(monster.id, new_enemy_pos);
            }
        }

        processed_monsters_move
    }

    pub(crate) fn update_monsters_position(
        &mut self,
        map_data: &mut MapData,
        monster_manager: &mut MutexGuard<MonsterManager>,
        processed_monsters_positions: HashMap<i32, Vec2>,
    ) {
        let monsters = monster_manager.get_monsters_mut();

        for mon_index in 0..monsters.len() {
            let monster = monster_manager.get_monster_mut(mon_index);
            if let Some(md) = monster {
                if let Some(new_mons_pos) = processed_monsters_positions.get(&md.id) {
                    let tmp_tile = map_data.map[new_mons_pos.y][new_mons_pos.x];

                    let tile_below_monster = md.get_tile_below_monster();
                    map_data.map[md.position.y][md.position.x] = Space::new(
                        self.update_monster_previous_tile(tile_below_monster, tmp_tile.tile),
                    );
                    md.tile_below_monster = tmp_tile.tile;
                    md.position = *new_mons_pos;
                }
            }
        }
    }

    fn update_player_previous_tile(&mut self, player: &mut Player, mut tmp_tile: char) -> char {
        let tile_set = DEFAULT_TILE_SET;

        if tmp_tile == tile_set.open_door {
            tmp_tile = tile_set.floor;
        }

        if player.tile_below_player == tile_set.open_door {
            tmp_tile = tile_set.open_door;
        }

        if player.tile_below_player == tile_set.closed_door_top {
            tmp_tile = tile_set.closed_door_top;
        }

        tmp_tile
    }

    fn update_monster_previous_tile(&mut self, tile_below_monster: char, tmp_tile: char) -> char {
        let tile_set = DEFAULT_TILE_SET;

        let mut updated_tile = tmp_tile;

        if tmp_tile == tile_set.open_door {
            updated_tile = tile_set.floor;
        }

        if tile_below_monster == tile_set.open_door {
            updated_tile = tile_set.open_door;
        }

        if tile_below_monster == tile_set.closed_door_top {
            updated_tile = tile_set.closed_door_top;
        }

        updated_tile
    }
}
