mod player;
mod chat;
mod map;
mod inventory;
mod status;
mod collision;
mod tile_set;
mod map_manager;
mod game_client;
mod player_movement_data;

use crossterm::{cursor, event, QueueableCommand, terminal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::io::{self, BufRead};
use crate::game_client::GameClient;

fn main() {
    let mut game_client = GameClient::new();

    game_client.map_manager.load_map_set_player_position("map2", 2, 6);
    game_client.print_terminal();

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    game_client.player.key_event = key_input.code;

                    let new_player_pos = game_client.collision_engine.move_player(&mut game_client.map_manager, &mut game_client.player, &mut game_client.chat);
                    let can_player_proceed = game_client.collision_engine.process_move(&mut game_client.player, &mut game_client.map_manager, &mut game_client.chat, new_player_pos);

                    if can_player_proceed {
                        game_client.collision_engine.update_player_position(&mut game_client.map_manager, new_player_pos);
                    }

                    game_client.print_terminal();
                    if game_client.player.key_state {
                        break;
                    }
                }
            }
            _ => {}
        }
    }
}