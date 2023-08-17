mod player;
mod chat;
mod map;
mod inventory;
mod status;
mod collision;
mod tile_set;
mod map_manager;

use crossterm::{cursor, event, QueueableCommand, terminal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::io::{self, BufRead};
use crate::collision::CollisionEngine;
use crate::player::Player;

fn main() {
    let mut player = Player::new();
    let mut collision_engine = CollisionEngine::new();

    player.map_manager.load_map_set_player_position("map2", 1, 5);

    let map = player.map_manager.get_map_mut(player.map_manager.current_map_index);

    // update map
    if let Some(map_data) = map {
        let modules = [player.status.get_status(), player.inventory.get_inventory_to_size(2, format!("FLOOR: {}", map_data.current_floor))];
        map_data.update_str_map_with_modules(&modules);
        player.print_terminal();
    }

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    player.key_event = key_input.code;

                    collision_engine.process_input(&mut player);

                    player.print_terminal();

                    if player.key_state {
                        break;
                    }
                    else {
                    }
                }
            }
            _ => {}
        }
    }
}