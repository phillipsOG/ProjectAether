mod player;
mod chat;
mod map;
mod inventory;
mod status;

use std::io::{stdout};
use crossterm::{cursor, event, QueueableCommand, terminal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::player::PlayerState;

fn main() {
    let mut player_state = PlayerState::new();

    player_state.map.load_map("src/map2.txt");
    player_state.update_player_position();
    player_state.print_terminal();

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    player_state.key_event = key_input.code;

                    player_state.process_input();
                    player_state.print_terminal();

                    if player_state.key_state {
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

fn clear_chat(amount: u16)
{
    let mut stdout = stdout();
    stdout.queue(cursor::MoveToPreviousLine(amount)).unwrap();
    stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
}

/* TODO place inside of a helper class */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}