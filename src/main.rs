use std::io::{stdout};
use crossterm::{cursor, event, QueueableCommand, terminal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct PlayerState
{
    pub key_event: KeyCode,
    pub previous_key_event: KeyCode,
    pub key_state: bool,
    pub player_position: Option<(usize, usize)>,
    pub chat: Chat,
    pub map: Map,
    pub inventory: Inventory,
    pub status: Status
}

struct Chat {
    pub chat: [String; 9],
    pub input_counter: usize,
    pub is_repeat_message: bool,
    pub repeat_message_counter: i32,
}

impl Chat {
    fn new() -> Self {
        Chat {
            chat: [
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string()
            ],
            input_counter: 0,
            is_repeat_message: false,
            repeat_message_counter: 0,
        }
    }

    fn print_chat(&mut self) {
        for i in 0..self.input_counter {
            println!("{}", self.chat[i]);
        }
    }

    fn process_chat_message(&mut self, message: &str) {
        if self.input_counter == 8 {
            self.input_counter = 0;
        }

        if self.chat[self.input_counter] == message {
            self.is_repeat_message = true;
        } else {
            self.is_repeat_message = false;
        }

        if self.is_repeat_message == true {
            self.repeat_message_counter += 1;

            let repeat_suffix = format!("x{}", self.repeat_message_counter);
            let repeated_message = format!("{} {}", message, repeat_suffix);
            self.chat[self.input_counter] = repeated_message;

        } else {
            self.chat[self.input_counter] = message.parse().unwrap();
            self.input_counter += 1;
            self.repeat_message_counter = 1;

        }
        /*if self.chat[self.input_counter] != message {
            self.chat[self.input_counter] = message.parse().unwrap();
            self.input_counter += 1;
        } else {
            self.repeat_message_counter += 1;

            let repeat_suffix = format!("x{}", self.repeat_message_counter);
            let repeated_message = format!("{} {}", message, repeat_suffix);

            self.chat[self.input_counter] = repeated_message;
        }*/
    }

    fn print_processed_input(&mut self) {
        if self.is_repeat_message {
            println!("{} x{}", self.chat[self.input_counter], self.repeat_message_counter);
        } else {
            println!("{}", self.chat[self.input_counter]);
        }
    }
}

struct Map {
    pub map: Vec<Vec<char>>,
    pub tile_set: TileSet
}

impl Map {
    fn new() -> Self {
        Map {
            map: vec![vec![]],
            tile_set: TileSet::new(),
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

    fn print_map(&self) {
        let mut stdout = stdout();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();

        for tile in &self.map {
            let tile_line : String = tile.iter().collect();
            println!("{}", tile_line);
        }
        println!("\n");
    }
}

struct Inventory
{
    keys: i32
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            keys: 0,
        }
    }

    fn add_key(&mut self, amount: i32) {
        self.keys += amount;
    }

    fn remove_key(&mut self, amount: i32) {
        self.keys -= amount;
    }
}

struct Status
{
    pub health: i32,
    pub str: i32,
    pub def: i32
}

impl Status {
    fn new() -> Self {
        Status {
            health: 100,
            str: 3,
            def: 1,
        }
    }
}

struct TileSet {
    pub wall: char,
    pub door: char,
    pub key: char,
    pub floor: char
}

impl TileSet {
    fn new() -> Self {
        TileSet {
            wall: '#',
            door: '|',
            key: 'k',
            floor: '.'
        }
    }
}

impl PlayerState {
    fn new() -> Self {
        PlayerState {
            key_event: KeyCode::Enter,
            previous_key_event: KeyCode::Null,
            key_state: false,
            player_position: None,
            chat: Chat::new(),
            map: Map::new(),
            inventory: Inventory::new(),
            status: Status::new()
        }
    }

   /* fn print_processed_input(&mut self) {
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
                println!("{}", self.chat[self.input_counter-i]);
            }*/
            /*for tile in &self.chat {
                println!("{}", tile);
            }*/
            self.repeat_key_counter = 1;
            self.input_counter += 1;
        }
    }*/
    /* fn process_direction_input(&mut self, direction: &str) {
        let key_event = self.key_event;
        let previous_key_event = self.previous_key_event;

        self.chat[self.input_counter] = direction.parse().unwrap();

        if key_event == previous_key_event {
            self.is_repeat_message = true;
            self.repeat_key_counter += 1;

        } else {
            self.repeat_key_counter = 1;
            self.is_repeat_message = false;
        }

        self.previous_key_event = key_event;
    }*/

    fn process_input(&mut self) {
        match self.key_event {
            KeyCode::Left => {
                self.move_player();
            }
            KeyCode::Right => {
                self.move_player();
            }
            KeyCode::Up => {
                self.move_player();
            }
            KeyCode::Down => {
                self.move_player();
            }
            KeyCode::Esc => {
                self.chat.process_chat_message("Pressed ESC & Exited the Game");
                self.previous_key_event = KeyCode::Esc;
                self.key_state = true;
            }
            _ => {}
        }
    }

    fn update_player_position(&mut self) {
        let at_position: Option<(usize, usize)> = None;

        for (row_idx, row) in self.map.map.iter().enumerate() {
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

    fn move_player(&mut self) {
        if let Some((row_idx, col_idx)) = self.player_position {
            let (new_row_idx, new_col_idx) = match self.key_event {
                KeyCode::Up => {
                    // Move up
                    self.chat.process_chat_message("You walk up.");
                    (row_idx - 1, col_idx)
                },
                KeyCode::Down => {
                    // Move down
                    self.chat.process_chat_message("You walk down.");
                    (row_idx + 1, col_idx)
                },
                KeyCode::Left => {
                    // Move left
                    self.chat.process_chat_message("You walk left.");
                    (row_idx, col_idx - 1)
                },
                KeyCode::Right => {
                    // Move right
                    self.chat.process_chat_message("You walk right.");
                    (row_idx, col_idx + 1)
                },
                _ => (row_idx, col_idx), // invalid direction, stay in place
            };

            // update the player position
            //let move_from = self.map[row_idx][col_idx];
            let move_to = self.map.map[new_row_idx][new_col_idx];

            // basic collision
            // @TODO create fn to handle the collision of other objects
            if self.process_move(move_to) {
                // set the new player position
                self.map.map[new_row_idx][new_col_idx] = '@';

                //set previous player tile
                self.map.map[row_idx][col_idx] = self.map.tile_set.floor;

                self.update_player_position();
            }
            //println!("Moved player from row: {}, column: {} to row: {}, column: {}", row_idx, col_idx, new_row_idx, new_col_idx);
        } else {
            println!("No '@' symbol found in the map.");
        }
    }

    fn process_move(&mut self, move_to_tile: char) -> bool {
        if move_to_tile == self.map.tile_set.wall {
            return false;
        } else if move_to_tile == self.map.tile_set.key {
            self.chat.process_chat_message("You pick up a rusty key.");
            self.inventory.add_key(1);
            return true;
        } else if move_to_tile == self.map.tile_set.door {
            return if self.inventory.keys >= 1 {
                self.inventory.remove_key(1);
                self.chat.process_chat_message("You unlock the door using a rusty key.");
                true
            } else {
                self.chat.process_chat_message("You need a rusty key to open this door.");
                false
            }
        }
        true
    }
}

fn main() {
    //let stdout = stdout();
    //stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
    let mut player_state = PlayerState::new();

    player_state.map.load_map("src/map1.txt");
    player_state.update_player_position();
    player_state.map.print_map();

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    player_state.key_event = key_input.code;

                    player_state.process_input();
                    player_state.map.print_map();
                    player_state.chat.print_chat();

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