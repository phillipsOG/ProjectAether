use crate::chat::Chat;
use crate::collision_engine::CollisionEngine;
use crate::map_factory::MapFactory;
use crate::map_manager::MapManager;
use crate::player::Player;
use crate::tile_set::{DEFAULT_TILE_SET, MONSTER_TILE_SET};
use crate::Map;
use crossterm::{terminal, QueueableCommand};
use std::io::stdout;
use crate::monster_generator::MonsterFactory;
use crate::monster_manager::MonsterManager;

pub struct GameClient {
    pub map_manager: MapManager,
    pub player: Player,
    pub collision_engine: CollisionEngine,
    pub chat: Chat,
    pub map_factory: MapFactory,
    pub monster_manager: MonsterManager,
    pub monster_factory: MonsterFactory,
}

impl GameClient {
    pub(crate) fn new() -> Self {
        GameClient {
            map_manager: MapManager::new(),
            player: Player::new(),
            collision_engine: CollisionEngine::new(),
            chat: Chat::new(),
            map_factory: MapFactory {},
            monster_manager: MonsterManager::new(),
            monster_factory: MonsterFactory::new(),
        }
    }

    pub(crate) fn print_terminal(&mut self) {
        let mut stdout = stdout();
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        let map = self.map_manager.get_map(self.map_manager.current_map_index);
        let mut str_map = String::new();

        if let Some(map_data) = map {
            let modules = [
                self.player.status.get_status(),
                self.player
                    .inventory
                    .get_inventory_to_size(2, format!("FLOOR: {}", map_data.current_floor)),
            ];
            let mut counter = 0;
            for tile in &map_data.map {
                let tile_line: String = tile
                    .iter()
                    .map(|space| {
                        if space.is_visible || space.tile == DEFAULT_TILE_SET.player {
                            space.tile
                        } else {
                             ' ' //show no map tile at iteration if not visible or player
                         }
                    })
                    .collect();

                if counter <= modules.len() {
                    str_map += &*format!(
                        "{}      {}      {}\n",
                        tile_line, modules[0][counter], modules[1][counter]
                    );
                    counter += 1;
                } else {
                    //println!("{}", tile_line);
                    str_map += &*format!("{}\n", tile_line);
                }
            }

            println!("{}", str_map);
            self.chat.print_chat();
        }
    }

    pub(crate) fn print_terminal_with_map(&mut self, updated_map: &mut Map) {
        let mut stdout = stdout();
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        let mut str_map = String::new();

        let mut counter = 0;
        for tile in updated_map {
            let tile_line: String = tile
                .iter()
                .map(|space| {
                    if space.is_visible {
                        space.tile
                    } else if space.tile == DEFAULT_TILE_SET.player {
                        '@'
                    } else {
                        ' ' // space character
                    }
                })
                .collect();
            str_map += &*format!("{}\n", tile_line);
        }

        println!("{}", str_map);
        self.chat.print_chat();
    }

    pub(crate) fn print_map(&self) {
        let mut stdout = stdout();
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        let map = self.map_manager.get_map(self.map_manager.current_map_index);
        if let Some(map_data) = map {
            for tile in &map_data.map {
                let tile_line: String = tile.iter().map(|space| space.tile).collect();
                println!("{}", tile_line);
            }
        }
    }
}
