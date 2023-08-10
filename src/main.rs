use std::io::{stdout};
use crossterm::{cursor, event, QueueableCommand, terminal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct TextState
{
    pub key_event: KeyCode,
    pub previous_key_event: KeyCode,
    pub key_state: bool,
    pub is_repeat_message: bool,
    pub repeat_key_counter: i32,
    pub input_counter: usize,
    pub map: String,
    pub chat: [String; 8],
    pub player_position: Option<(usize, usize)>
}

impl TextState {
    fn new() -> Self {
        TextState {
            key_event: KeyCode::Enter,
            previous_key_event: KeyCode::Null,
            key_state: false,
            is_repeat_message: false,
            repeat_key_counter: 0,
            input_counter: 0,
            map: String::new(),
            chat: [
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string()
            ],
            player_position: None
        }
    }

    fn print_processed_input(&mut self) {
        if self.is_repeat_message {
             self.repeat_key_counter += 1;
            println!("{} x{}", self.chat[self.input_counter], self.repeat_key_counter);
        }
        else {
            println!("{}", self.chat[self.input_counter]);
            self.repeat_key_counter = 0;
            self.input_counter += 1;
        }
    }
}

fn main() {
    //let stdout = stdout();
    //stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();

    let mut text_state = TextState::new();

    let mut map = "".to_owned();
    map += "\n";
    if let Ok(lines) = read_lines("src/map1.txt") {
        for line in lines {
            if let Ok(tile) = line {
                map += &tile;
                map += "\n";
            }
        }
    }

    text_state.map = map;

    text_state = get_player_position(text_state);
    text_state = get_map(text_state);

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    text_state.key_event = key_input.code;

                    if text_state.input_counter == 8 {
                        text_state.input_counter = 0;
                        clear_chat(8);
                    }

                    text_state = process_input(text_state);
                    text_state = get_map(text_state);
                    text_state.print_processed_input();

                    if text_state.key_state {
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

fn get_map(input: TextState) -> TextState {
    let mut stdout = stdout();
    stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();

    println!("{}\n", input.map);

    input
}

fn process_input(mut input: TextState) -> TextState {
    match input.key_event {
        KeyCode::Left => {
            input = move_player(input);
            input = text_director(input, "You walk left.");
            input
        }
        KeyCode::Right => {
            input = move_player(input);
            input = text_director(input,"You walk right.");
            input
        }
        KeyCode::Up => {
            input = move_player(input);
            input = text_director(input,"You walk up.");
            input
        }
        KeyCode::Down => {
            input = move_player(input);
            input = text_director(input,"You walk down.");
            input
        }
        KeyCode::Esc => {
            println!("Pressed ESC & Exited the Game");
            input.previous_key_event = KeyCode::Esc;
            input.key_state = true;
            input
        }

        _ => input
    }
}

fn text_director(mut input: TextState, message: &str) -> TextState {
    let key_event = input.key_event;
    let previous_key_event = input.previous_key_event;

    input.chat[input.input_counter] = message.parse().unwrap();

    if key_event == previous_key_event {
        clear_chat(0);
        input.is_repeat_message = true;
    } else {
        input.is_repeat_message = false;
    }

    input.previous_key_event = key_event;
    input
}



fn get_player_position(mut input: TextState) -> TextState {
    let map_lines: Vec<&str> = input.map.trim().lines().collect();
    let at_position: Option<(usize, usize)> = None;

    for (row_idx, row) in map_lines.iter().enumerate() {
        for (col_idx, c) in row.chars().enumerate() {
            if c == '@' {
                input.player_position = Option::from((row_idx, col_idx));
                break;
            }
        }

        if at_position.is_some() {
            break;
        }
    }

    input
}

fn move_player(mut input: TextState) -> TextState {
    let map_lines: Vec<&str> = input.map.trim().lines().collect();

    if let Some((row_idx, col_idx)) = input.player_position {
        let (new_row_idx, new_col_idx) = match input.key_event {
            KeyCode::Up => (row_idx - 1, col_idx),    // Move up
            KeyCode::Down => (row_idx + 1, col_idx),  // Move down
            KeyCode::Left => (row_idx, col_idx - 1),  // Move left
            KeyCode::Right => (row_idx, col_idx + 1), // Move right
            _ => (row_idx, col_idx), // Invalid direction, stay in place
        };

        // 2D rep of our ascii map
        let mut map_chars: Vec<Vec<char>> = map_lines.iter().map(|line| line.chars().collect()).collect();

        // update the player position
        //let move_from = map_chars[row_idx][col_idx];
        let move_to = map_chars[new_row_idx][new_col_idx];

        // basic collision
        if move_to != '#' {
            map_chars[new_row_idx][new_col_idx] = '@'; // Set the new position
            map_chars[row_idx][col_idx] = move_to;
        }

        // convert the map back to a string
        input.map = map_chars
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        input = get_player_position(input);

        //println!("Moved player from row: {}, column: {} to row: {}, column: {}", row_idx, col_idx, new_row_idx, new_col_idx);
    } else {
        println!("No '@' symbol found in the map.");
    }

    input
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