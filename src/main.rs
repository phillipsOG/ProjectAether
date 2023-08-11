mod player;
mod chat;
mod map;
mod inventory;
mod status;
mod collision;

use crossterm::{cursor, event, QueueableCommand, terminal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::io::{self, BufRead};
use crate::collision::CollisionEngine;
use crate::player::Player;

fn main() {
    let mut player = Player::new();
    let mut collision_engine = CollisionEngine::new();

    player.map.load_map("src/map2.txt");
    collision_engine.update_player_position(player);
    player.print_terminal();

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    player.key_event = key_input.code;

                    collision_engine.process_input(player);
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