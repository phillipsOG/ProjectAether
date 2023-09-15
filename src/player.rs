use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use crate::Direction;

use crate::inventory::Inventory;
use crate::status::Status;
use crate::vec2::Vec2;

#[derive(Clone)]
pub struct Player {
    pub key_event: Event,
    pub previous_key_event: Event,
    pub key_code: Keycode,
    pub key_state: bool,
    pub inventory: Inventory,
    pub status: Status,
    pub position: Vec2,
    pub previous_player_position: Vec2,
    pub tile_below_player: &'static str,
    pub previous_tile_below_player: &'static str,
    pub multi_tile_below_player: bool,
    pub current_floor: usize,
    pub fog_of_war: bool,
    pub is_alive: bool,
    pub direction: Direction,
    pub sprite_position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub current_frame: i32,
}

impl Player {
    pub(crate) fn new() -> Self {
        Player {
            key_event: Event::Unknown {timestamp: 0, type_: 0 },
            previous_key_event: Event::Unknown {timestamp: 0, type_: 0 },
            key_code: Keycode::Down,
            key_state: false,
            inventory: Inventory::new(),
            status: Status::new(),
            position: Vec2::ZERO,
            previous_player_position: Vec2::ZERO,
            tile_below_player: ".",
            previous_tile_below_player: ".",
            multi_tile_below_player: false,
            current_floor: 0,
            fog_of_war: true,
            is_alive: true,
            direction: Direction::Right,
            sprite_position: Point::new(0, 0),
            sprite: Rect::new(0, 0, 26, 36),
            speed: 0,
            current_frame: 0
        }
    }

    pub(crate) fn update_tile_below_player(&mut self, tile: &'static str) {
        self.tile_below_player = tile;
    }
}
