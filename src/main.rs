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
    game_client.print_terminal();

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    game_client.player.key_event = key_input.code;

                    game_client.collision_engine.process_input(
                        &mut game_client.player,
                        &mut game_client.map_manager,
                        &mut game_client.chat
                    );

                    // check if the scene transition flag is set
                    if game_client.map_manager.should_transition {
                        game_client.map_manager.load_map_set_player_position(
                            &game_client.target_map,
                            game_client.target_position.0,
                            game_client.target_position.1,
                        );

                        // reset the scene transition flags
                        game_client.should_transition = false;
                    }

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