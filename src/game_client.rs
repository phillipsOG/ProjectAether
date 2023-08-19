use std::io::stdout;
use crossterm::{QueueableCommand, terminal};
use crate::chat::Chat;
use crate::collision::CollisionEngine;
use crate::map_manager::MapManager;
use crate::player::Player;

pub struct GameClient {
    pub map_manager: MapManager,
    pub player: Player,
    pub collision_engine: CollisionEngine,
    pub chat: Chat
}

impl GameClient {
    pub(crate) fn new() -> Self {
        GameClient {
            map_manager: MapManager::new(),
            player: Player::new(),
            collision_engine: CollisionEngine::new(),
            chat: Chat::new()
        }
    }

    pub(crate) fn print_terminal(&mut self) {
        let mut stdout = stdout();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        let map = self.map_manager.get_map(self.map_manager.current_map_index);

        if let Some(map_data) = map {
            let modules = [self.player.status.get_status(), self.player.inventory.get_inventory_to_size(2, format!("FLOOR: {}", map_data.current_floor))];
            let mut counter = 0;
            for tile in &map_data.map {
                let tile_line: String = tile.iter().map(|space| space.tile).collect();
                if counter <= modules.len() {
                    println!("{}      {}      {}", tile_line, modules[0][counter], modules[1][counter]);
                    counter += 1;

                } else {
                    println!("{}", tile_line);
                }
            }

            println!("\n");
            self.chat.print_chat();
        }
    }

    pub(crate) fn print_map(&self) {
        let mut stdout = stdout();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        let map = self.map_manager.get_map(self.map_manager.current_map_index);
        if let Some(map_data) = map {
            for tile in &map_data.map {
                let tile_line: String = tile.iter().map(|space| space.tile).collect();
                println!("{}", tile_line);
            }
        }
    }
}