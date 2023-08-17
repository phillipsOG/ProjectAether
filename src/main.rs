mod player;
mod chat;
mod map;
mod inventory;
mod status;
mod collision;
mod tile_set;
mod map_manager;
mod game_client;

use crossterm::{cursor, event, QueueableCommand, terminal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::io::{self, BufRead};
use crate::game_client::GameClient;

fn main() {

    let mut game_client = GameClient::new();

    game_client.map_manager.load_map_set_player_position("map2", 2, 6);
    let map = game_client.map_manager.get_map_mut(game_client.map_manager.current_map_index);

    // update map
    if let Some(map_data) = map {
        let modules = [game_client.player.status.get_status(), game_client.player.inventory.get_inventory_to_size(2, format!("FLOOR: {}", map_data.current_floor))];
        map_data.update_str_map_with_modules(&modules);
        game_client.print_terminal();
    }

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    game_client.player.key_event = key_input.code;

                    game_client.collision_engine.process_input(&mut game_client.player, &mut game_client.map_manager, &mut game_client.chat);
                    game_client.print_terminal();

                    if game_client.player.key_state {
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