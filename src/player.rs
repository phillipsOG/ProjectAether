use crossterm::event::KeyCode;
use crate::chat::Chat;
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

    pub(crate) fn print_terminal(&mut self) {
        //self.map.print_map_with_module(&self.status.get_status());
        let modules = [self.status.get_status(), self.inventory.get_inventory_to_size(2)];
        self.map.print_map_with_modules(&modules);
        self.chat.print_chat();
    }
}