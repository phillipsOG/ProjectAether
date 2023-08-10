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
    pub map: Vec<Vec<char>>,
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
            map: vec![vec![]],
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

            /*for i in 0..self.input_counter {
                println!("{}", self.chat[i]);
            }*/
            println!("{} x{}", self.chat[self.input_counter], self.repeat_key_counter);
        }
        else {
            println!("{}", self.chat[self.input_counter]);
            /*for i in 0..self.input_counter {
                println!("{}", self.chat[i]);
            }*/

            self.repeat_key_counter = 1;
            self.input_counter += 1;
        }
    }

    fn print_map(&self) {
        let mut stdout = stdout();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();

        for tile in &self.map {
            let tile_line : String = tile.iter().collect();
            println!("{}", tile_line);
        }
        println!("\n");
    }

    fn process_input(&mut self) {
        match self.key_event {
            KeyCode::Left => {
                move_player(self);
                text_director(self, "You walk left.");
            }
            KeyCode::Right => {
                move_player(self);
                text_director(self,"You walk right.");
            }
            KeyCode::Up => {
                move_player(self);
                text_director(self,"You walk up.");
            }
            KeyCode::Down => {
                move_player(self);
                text_director(self,"You walk down.");
            }
            KeyCode::Esc => {
                text_director(self,"Pressed ESC & Exited the Game");
                self.previous_key_event = KeyCode::Esc;
                self.key_state = true;
            }
            _ => {}
        }
    }

    fn update_player_position(&mut self) {
        let at_position: Option<(usize, usize)> = None;

        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if c == '@' {
                    self.player_position = Option::from((row_idx, col_idx));
                    break;
                }
            }
            if at_position.is_some() {
                break;
            }
        }
    }

    fn load_map(&mut self, name: &str) {
        let mut map = "".to_owned();
        map += "\n";
        if let Ok(lines) = read_lines(name) {
            for line in lines {
                if let Ok(tile) = line {
                    map += &tile;
                    map += "\n";
                }
            }
        }
        let map_lines: Vec<&str> = map.trim().lines().collect();

        // 2D rep of our ascii map
        self.map = map_lines.iter().map(|line| line.chars().collect()).collect();
    }
}

fn main() {
    //let stdout = stdout();
    //stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();

    let mut text_state = TextState::new();

    text_state.load_map("src/map1.txt");
    text_state.update_player_position();
    text_state.print_map();

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    text_state.key_event = key_input.code;

                    if text_state.input_counter == 8 {
                        text_state.input_counter = 0;
                        clear_chat(8);
                    }

                    text_state.process_input();
                    text_state.print_map();
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

fn text_director(mut input: &mut TextState, message: &str) {
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
}

fn move_player(mut input: &mut TextState) {

    if let Some((row_idx, col_idx)) = input.player_position {
        let (new_row_idx, new_col_idx) = match input.key_event {
            KeyCode::Up => (row_idx - 1, col_idx),    // Move up
            KeyCode::Down => (row_idx + 1, col_idx),  // Move down
            KeyCode::Left => (row_idx, col_idx - 1),  // Move left
            KeyCode::Right => (row_idx, col_idx + 1), // Move right
            _ => (row_idx, col_idx), // invalid direction, stay in place
        };

        // update the player position
        //let move_from = map_chars[row_idx][col_idx];
        let move_to = input.map[new_row_idx][new_col_idx];

        // basic collision
        // @TODO create fn to handle the collision of other objects
        if move_to != '#' {
            input.map[new_row_idx][new_col_idx] = '@'; // Set the new position
            input.map[row_idx][col_idx] = move_to;
        }

        input.update_player_position();
        //println!("Moved player from row: {}, column: {} to row: {}, column: {}", row_idx, col_idx, new_row_idx, new_col_idx);
    } else {
        println!("No '@' symbol found in the map.");
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