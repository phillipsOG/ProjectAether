use std::collections::{HashMap, HashSet};

use crate::chat::Chat;
use crate::map_data::MapData;

use crate::player::Player;
use crate::tile_set::{DEFAULT_TILE_SET, LADDER_TILE_SET, MONSTER_TILE_SET};
use crate::MovementType;
use crossterm::event::KeyCode;

use crate::map_manager::MapManager;
use futures::lock::{Mutex, MutexGuard};
use std::io;
use std::sync::Arc;
use crate::monster::Monster;

use crate::monster_manager::MonsterManager;
use crate::pathfinding::Pathfinding;
use crate::space::Space;
use crate::Vec2;

#[derive(Clone)]
pub struct CollisionEngine {}

struct MonsterPositionSet {
    pub current_position: Vec2,
    pub new_position: Vec2,
}

impl MonsterPositionSet {
    pub(crate) fn new(current_position: Vec2, new_position: Vec2) -> Self {
        MonsterPositionSet {
            current_position,
            new_position,
        }
    }
}

impl CollisionEngine {
    pub(crate) fn new() -> Self {
        CollisionEngine {}
    }

    pub(crate) async fn try_move_player(
        &mut self,
        player: &mut Player,
        chat: &mut Arc<Mutex<Chat>>,
    ) -> Vec2 {
        let mut current_position = Vec2::ZERO;
        let mut chat_guard = chat.lock().await;
        match player.key_event {
            KeyCode::Up => {
                // Move up
                chat_guard.process_chat_message("You walk up.");
                return Vec2::new(player.position.x, player.position.y - 1);
            }
            KeyCode::Down => {
                // Move down
                chat_guard.process_chat_message("You walk down.");
                return Vec2::new(player.position.x, player.position.y + 1);
            }
            KeyCode::Left => {
                // Move left
                chat_guard.process_chat_message("You walk left.");
                return Vec2::new(player.position.x - 1, player.position.y);
            }
            KeyCode::Right => {
                // Move right
                chat_guard.process_chat_message("You walk right.");
                return Vec2::new(player.position.x + 1, player.position.y);
            }
            KeyCode::Tab => {
                println!("Please enter a command: ");

                let mut input = String::new();
                if let Err(_) = io::stdin().read_line(&mut input) {
                    chat_guard.process_chat_message("Error reading command.");
                }
                // @TODO turn into actual command system
                if input.trim() == "nofog" {
                    if player.fog_of_war {
                        chat_guard.process_chat_message("Removed fog of war.");
                        player.fog_of_war = false;
                    } else {
                        chat_guard.process_chat_message("Added back fog of war.");
                        player.fog_of_war = true;
                    }
                } else {
                    chat_guard.process_chat_message("Invalid command.");
                }
            }

            _ => {}
        }
        current_position = player.position;
        // don't move
        return current_position;
    }

    pub(crate) async fn try_process_move<'a>(
        &mut self,
        map_manager_clone: &mut MutexGuard<'a, MapManager>,
        player: &mut Player,
        chat: &mut Arc<Mutex<Chat>>,
        new_player_pos: Vec2,
    ) -> MovementType {
        let map_index = map_manager_clone.current_map_index;
        let map = map_manager_clone.get_map_mut(map_index).expect("map data");

        let space = map.map[new_player_pos.y][new_player_pos.x];
        let tmp_tile = map.map[new_player_pos.y][new_player_pos.x].tile;
        let is_tile_solid = map.map[new_player_pos.y][new_player_pos.x].is_solid;
        let is_tile_traversable = map.map[new_player_pos.y][new_player_pos.x].is_traversable;
        let tile_set = map.tile_set.clone();
        let mut chat_guard = chat.lock().await;
        let res = self.check_for_multi_tile(map, tmp_tile, new_player_pos);

        if space.is_monster {
            return MovementType::Battle
        }

        if res == tile_set.ladder && tile_set.name == DEFAULT_TILE_SET.name {
            if player.key_event == KeyCode::Up {
                return MovementType::LadderUp;
            } else if player.key_event == KeyCode::Down {
                return MovementType::LadderDown;
            }
        } else if res == tile_set.ladder && tile_set.name == LADDER_TILE_SET.name {
            if player.key_event == KeyCode::Up && player.position.y == 1 {
                return MovementType::LadderEnter;
            } else if player.key_event == KeyCode::Down && player.position.y == 2 {
                return MovementType::LadderExit;
            }
        }

        if tmp_tile == tile_set.key {
            chat_guard.process_chat_message("You pick up a rusty key.");
            player.inventory.add_key(1);
            map.map[new_player_pos.y][new_player_pos.x] = Space::new(DEFAULT_TILE_SET.floor);
        } else if tmp_tile == tile_set.closed_door_side || tmp_tile == tile_set.closed_door_top {
            if player.inventory.keys >= 1 {
                player.inventory.remove_key(1);
                chat_guard.process_chat_message("You unlock the door using a rusty key.");
                map.map[new_player_pos.y][new_player_pos.x] =
                    Space::new(DEFAULT_TILE_SET.open_door);
            } else {
                chat_guard.process_chat_message("You need a rusty key to open this door.");
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
        }
        drop(chat_guard);
        return MovementType::Unable;
    }

    fn is_tile_monster(&self, monster: char) -> bool {
        let monster_variants = [MONSTER_TILE_SET.snake, MONSTER_TILE_SET.goblin];

        for monster_variant in monster_variants {
            if monster_variant == monster {
                return true;
            }
        }
        false
    }

    pub(crate) async fn update_player_position<'a>(
        &mut self,
        map_manager_clone: &mut MutexGuard<'a, MapManager>,
        player: &mut Player,
        new_player_position: Vec2,
    ) {
        let map_index = map_manager_clone.current_map_index;
        let map = map_manager_clone.get_map_mut(map_index).expect("map data");
        let tmp_tile = map.map[new_player_position.y][new_player_position.x].tile;
        let pos = player.position.clone();
        map.map[pos.y][pos.x] = Space::new(self.update_player_previous_tile(player, tmp_tile));
        player.position = new_player_position;
        player.tile_below_player = tmp_tile;
        map.set_player_position(new_player_position);
        player.update_tile_below_player(tmp_tile);
    }

    pub(crate) async fn update_player_vision<'a>(
        &mut self,
        map_manager_clone: &mut MutexGuard<'a, MapManager>,
        player: &Player,
        _new_player_position: Vec2,
    ) {
        //let mut map_manager_guard = map_manager_clone.lock().await;
        let map_index = map_manager_clone.current_map_index;
        let map_data = map_manager_clone.get_map_mut(map_index).expect("map data");
        map_data.set_player_vision(player, _new_player_position);
    }

    fn check_for_multi_tile(
        &mut self,
        map_data: &MapData,
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

    pub(crate) async fn try_move_monsters<'a>(
        &mut self,
        player: &MutexGuard<'a, Player>,
        monster_manager: &mut MonsterManager,
        map_guard: &mut MutexGuard<'a, MapManager>,
        chat: &mut Arc<Mutex<Chat>>,
    ) -> HashMap<i32, Vec2> {
        let mut new_monsters_position = HashMap::<i32, Vec2>::new();
        let map_index = map_guard.current_map_index;
        if let Some(map_data) = map_guard.get_map_mut(map_index) {
            for monster in monster_manager.get_monsters_mut().values_mut() {

                let cur_monster_pos = monster.position;
                let mut new_pos = cur_monster_pos;

                if !monster.in_battle {
                    // essentially acts as the tile radius for the monster searching for the player
                    let radius = 10;
                    new_pos = Pathfinding::find_shortest_path(
                        &map_data.map,
                        cur_monster_pos,
                        player.position,
                        radius,
                    )
                        .await;

                    if new_pos == cur_monster_pos {
                        // if new pos is the same as cur mon pos than no path found to the player within given radius
                        // make the monster wander
                        new_pos = Pathfinding::wander(cur_monster_pos, &map_data.map);
                        chat.lock().await.process_debug_message("monster is wandering", 0);
                    } else {
                        chat.lock().await.process_debug_message("monster stopped wandering", 0);
                    }
                }

                new_monsters_position.insert(monster.id, new_pos);
            }
        }
        new_monsters_position
    }

    pub(crate) async fn process_monsters_move<'a>(
        &mut self,
        new_monsters_position: &mut HashMap<i32, Vec2>,
        map_manager_clone: &mut MutexGuard<'a, MapManager>,
        monster_manager: &mut MutexGuard<'a, MonsterManager>,
    ) -> HashMap<i32, Vec2> {
        let mut processed_monsters_move = HashMap::<i32, Vec2>::new();

        for monster in monster_manager.get_monsters_mut().values_mut() {
            if let Some(new_enemy_pos) = new_monsters_position.get_mut(&monster.id) {
                let map_index = map_manager_clone.current_map_index;
                if let Some(map_data) = map_manager_clone.get_map_mut(map_index) {
                    let tmp_tile = map_data.map[new_enemy_pos.y][new_enemy_pos.x];

                    if tmp_tile.is_occupied {
                        continue;
                    }

                    if !tmp_tile.is_traversable {
                        continue;
                    }

                    if tmp_tile.is_solid {
                        continue;
                    }

                    processed_monsters_move.insert(monster.id, *new_enemy_pos);
                }
            }
        }
        self.remove_duplicate_monster_positions(processed_monsters_move)
    }

    fn remove_duplicate_monster_positions(
        &self,
        new_monsters_position: HashMap<i32, Vec2>,
    ) -> HashMap<i32, Vec2> {
        let mut non_duplicate_positions = HashMap::<i32, Vec2>::new();
        let mut previous_positions: HashSet<Vec2> = HashSet::new();

        for (key, pos) in new_monsters_position {
            let new_position = pos;

            if !previous_positions.contains(&new_position) {
                previous_positions.insert(new_position.clone());
                non_duplicate_positions.insert(key, new_position);
            }
        }

        non_duplicate_positions
    }

    pub(crate) async fn update_monsters_position<'a>(
        &mut self,
        map_manager_clone: &mut MutexGuard<'a, MapManager>,
        monster_manager: &mut MutexGuard<'a, MonsterManager>,
        processed_monsters_positions: HashMap<i32, Vec2>,
    ) {
        for monster in monster_manager.get_monsters_mut().values_mut() {
            if let Some(new_mons_pos) = processed_monsters_positions.get(&monster.id) {
                let map_index = map_manager_clone.current_map_index;
                let map_data = map_manager_clone.get_map_mut(map_index).expect("map data");
                let tmp_tile = map_data.map[new_mons_pos.y][new_mons_pos.x].tile;

                map_data.map[monster.position.y][monster.position.x] = Space::new(self.update_monster_previous_tile(monster, tmp_tile));
                monster.position = *new_mons_pos;
                monster.tile_below = tmp_tile;

                let mut updated_space = Space::new(monster.tile);
                updated_space.is_occupied = true;

                map_data.map[new_mons_pos.y][new_mons_pos.x] = updated_space;
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

    fn update_monster_previous_tile(&mut self, monster: &mut Monster, mut tmp_tile: char) -> char {
        let tile_set = DEFAULT_TILE_SET;

        if tmp_tile == tile_set.open_door {
            tmp_tile = tile_set.floor;
        }

        if monster.tile_below == tile_set.open_door {
            tmp_tile = tile_set.open_door;
        }

        if monster.tile_below == tile_set.closed_door_top {
            tmp_tile = tile_set.closed_door_top;
        }

        tmp_tile
    }
}