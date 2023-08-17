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
        //self.map.print_map_with_module(&self.status.get_status());
        let map = self.map_manager.get_map(self.map_manager.current_map_index);
        if let Some(map_data) = map {
            let mut stdout = stdout();
            stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
            println!("{}", map_data.str_map);

            self.chat.print_chat();
        }
    }
}