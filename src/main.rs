mod player;
mod chat;
mod map;
mod inventory;
mod status;
mod collision;

use crossterm::{cursor, event, QueueableCommand, terminal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::io::{self, BufRead};
use crate::collision::Collision;
use crate::player::Player;

fn main() {
    let mut player = Player::new();
    let mut collision_engine = Collision::new();

    player.map.load_map("src/map2.txt");
    player.update_player_position();
    player.print_terminal();

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    player.key_event = key_input.code;

                    player.process_input();
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