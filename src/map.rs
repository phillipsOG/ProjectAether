use std::fs::File;
use std::io;
use std::io::{BufRead, stdout};
use std::path::Path;
use crossterm::{QueueableCommand, terminal};

pub struct Map {
    pub map: Vec<Vec<char>>,
    pub str_map: String,
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
            tile_below_player: '.',
            multi_tile_below_player: false,
            previous_tile_x_coord: 0,
            previous_tile_y_coord: 0
        }
    }

    pub(crate) fn load_map(&mut self, map_name: &str) {
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

