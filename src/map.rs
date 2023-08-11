use std::io::stdout;
use crossterm::{QueueableCommand, terminal};
use crate::read_lines;

pub struct Map {
    pub map: Vec<Vec<char>>,
    pub tile_set: TileSet
}

impl Map {
    pub(crate) fn new() -> Self {
        Map {
            map: vec![vec![]],
            tile_set: TileSet::new(),
        }
    }

    pub(crate) fn load_map(&mut self, name: &str) {
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

    pub(crate) fn print_map_with_modules(&self, module: &[[String; 3]; 2]) {
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
    }
}

pub struct TileSet {
    pub wall: char,
    pub door: char,
    pub key: char,
    pub floor: char,
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