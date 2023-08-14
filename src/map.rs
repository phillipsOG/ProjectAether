use std::fs::File;
use std::io;
use std::io::{BufRead, stdout};
use std::path::Path;
use crossterm::{QueueableCommand, terminal};
use crate::{player, tile_set};
use crate::tile_set::{DEFAULT_TILE_SET, TileSet};

pub struct Map {
    pub map: Vec<Vec<char>>,
    pub str_map: String,
    pub tile_set: TileSet,
    pub player_position: Option<(usize, usize)>,
    pub tile_below_player: char,
    pub multi_tile_below_player: bool,
    pub previous_tile_x_coord: usize,
    pub previous_tile_y_coord: usize
}

impl Map {
    pub(crate) fn new() -> Self {
        Map {
            map: vec![vec![]],
            str_map: String::new(),
            tile_set: DEFAULT_TILE_SET,
            player_position: None,
            tile_below_player: '.',
            multi_tile_below_player: false,
            previous_tile_x_coord: 0,
            previous_tile_y_coord: 0
        }
    }

    pub(crate) fn load_map_set_player_position(&mut self, map_name: &str, pos_x: usize, pos_y: usize) {
        let mut map = "".to_owned();
        let map_name = format!("src/maps/{}.txt", map_name);

        map += "\n";
        if let Ok(lines) = self.read_lines(map_name) {
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
        self.set_player_position(pos_x, pos_y);
    }

    pub(crate) fn set_player_position(&mut self, pos_x: usize, pos_y: usize) {
        let tile_set = &self.tile_set;

        let mut positions_to_modify = Vec::new();

        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if row_idx == pos_x && col_idx == pos_y {
                    if self.map[row_idx][col_idx] != tile_set.wall && (row_idx == pos_x && col_idx == pos_y) {
                        positions_to_modify.push((pos_x, pos_y));
                    } else if self.map[row_idx+1][col_idx+1] != tile_set.wall && self.map[row_idx+1][col_idx+1] != tile_set.closed_door_side {
                        positions_to_modify.push((pos_x+1, pos_y+1));
                    }
                }
            }
        }

        for (mod_pos_x, mod_pos_y) in positions_to_modify {
            self.player_position = Some((mod_pos_x, mod_pos_y));
            self.map[mod_pos_x][mod_pos_y] = tile_set.player;
        }
    }

    pub(crate) fn update_player_position(&mut self) {
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

    pub(crate) fn set_map_tile_set(&mut self, tile_set: TileSet) {
        self.tile_set = tile_set;
    }

    pub(crate) fn update_tile_below_player(&mut self, tile: char, x_coord: usize, y_coord: usize) {
        self.tile_below_player = tile;
        self.previous_tile_x_coord = x_coord;
        self.previous_tile_y_coord = y_coord;
    }

    pub(crate) fn print_map(&self) {
        let mut stdout = stdout();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();

        for tile in &self.map {
            let tile_line : String = tile.iter().collect();
            println!("{}", tile_line);
        }
    }

    fn print_map_with_module(&self, module: &[String]) {
        let mut stdout = stdout();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        let mut counter = 0;
        for tile in &self.map {
            let tile_line : String = tile.iter().collect();
            if counter <= module.len()-1 {
                println!("{}          {}", tile_line, module[counter]);
                counter += 1;
            } else {
                println!("{}", tile_line);
            }
        }
        println!("\n");
    }

    /*pub(crate) fn print_map_with_modules(&self, module: &[[String; 3]; 2]) {
        let mut stdout = stdout();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        let mut counter = 0;
        for tile in &self.map {
            let tile_line : String = tile.iter().collect();
            if counter <= module.len() {
                println!("{}      {}      {}", tile_line, module[0][counter], module[1][counter]);
                counter += 1;
            } else {
                println!("{}", tile_line);
            }
        }
        println!("\n");
    }*/

    pub(crate) fn update_str_map_with_modules(&mut self, module: &[[String; 3]; 2]) {
        let mut counter = 0;
        self.str_map = String::new();
        for tile in &self.map {
            let tile_line : String = tile.iter().collect();
            if counter <= module.len() {
                self.str_map += &*format!("{}      {}      {}", tile_line, module[0][counter], module[1][counter]);
                counter += 1;
            } else {
                self.str_map += &*format!("{}", tile_line);
            }
            self.str_map += &*format!("\n");
        }
    }

    /* TODO place inside of a helper class */
    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

