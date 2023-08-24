use crossterm::event::KeyCode;

use crate::inventory::Inventory;
use crate::status::Status;

pub struct Player {
    pub key_event: KeyCode,
    pub previous_key_event: KeyCode,
    pub key_state: bool,
    pub inventory: Inventory,
    pub status: Status,
}

impl Player {
    pub(crate) fn new() -> Self {
        Player {
            key_event: KeyCode::Enter,
            previous_key_event: KeyCode::Null,
            key_state: false,
            inventory: Inventory::new(),
            status: Status::new(),
        }
    }
}
