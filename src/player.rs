use std::io::{stdout, Write};
use crossterm::event::KeyCode;
use crossterm::{QueueableCommand, terminal};
use crate::chat::Chat;
use crate::inventory::Inventory;
use crate::map_manager::MapManager;
use crate::status::Status;

pub struct Player
{
    pub key_event: KeyCode,
    pub previous_key_event: KeyCode,
    pub key_state: bool,
    pub chat: Chat,
    pub inventory: Inventory,
    pub status: Status
}

pub struct Game {
    pub map_manager: MapManager
}

impl Game {
    pub(crate) fn new() -> Self {
        //map_manager: MapManager::new()
    }

}

impl Player {
    pub(crate) fn new() -> Self {
        Player {
            key_event: KeyCode::Enter,
            previous_key_event: KeyCode::Null,
            key_state: false,
            chat: Chat::new(),
            inventory: Inventory::new(),
            status: Status::new()
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