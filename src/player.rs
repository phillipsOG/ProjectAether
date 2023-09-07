use crossterm::event::KeyCode;

use crate::inventory::Inventory;
use crate::status::Status;
use crate::vec2::Vec2;

#[derive(Clone)]
pub struct Player {
    pub key_event: KeyCode,
    pub previous_key_event: KeyCode,
    pub key_state: bool,
    pub inventory: Inventory,
    pub status: Status,
    pub position: Vec2,
    pub previous_player_position: Vec2,
    pub tile_below_player: char,
    pub previous_tile_below_player: char,
    pub multi_tile_below_player: bool,
    pub current_floor: usize,
    pub fog_of_war: bool,
}

impl Player {
    pub(crate) fn new() -> Self {
        Player {
            key_event: KeyCode::Enter,
            previous_key_event: KeyCode::Null,
            key_state: false,
            inventory: Inventory::new(),
            status: Status::new(),
            position: Vec2::ZERO,
            previous_player_position: Vec2::ZERO,
            tile_below_player: '.',
            previous_tile_below_player: '.',
            multi_tile_below_player: false,
            current_floor: 0,
            fog_of_war: true,
        }
    }

    pub(crate) fn update_tile_below_player(&mut self, tile: char) {
        self.tile_below_player = tile;
    }
}
