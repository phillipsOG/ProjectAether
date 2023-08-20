use std::fs::File;
use std::io;
use std::io::{BufRead, stdout};
use std::path::Path;
use crossterm::{QueueableCommand, terminal};
use crate::space::Space;
use crate::tile_set::{DEFAULT_TILE_SET, TileSet};

type Map = Vec<Vec<Space>>;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize
}

impl Vec2 {
    pub const ZERO: Self = Self::new(0, 0);

    pub const fn new(x: usize, y: usize) -> Self {
        Self {x, y}
    }
}

pub struct MapData {
    pub map: Map,
    pub str_map: String,
    pub tile_set: TileSet,
    pub player_position: Vec2,
    pub previous_player_position: Vec2,
    pub tile_below_player: char,
    pub multi_tile_below_player: bool,
    pub current_floor: usize,
    pub map_width: usize, 
    pub map_height: usize
}

impl MapData {
    pub(crate) fn new() -> Self {
        MapData {
            map: Map::new(),
            str_map: String::new(),
            tile_set: DEFAULT_TILE_SET,
            player_position: Vec2::ZERO,
            previous_player_position: Vec2::ZERO,
            tile_below_player: '.',
            multi_tile_below_player: false,
            current_floor: 0,
            map_width: 0,
            map_height: 0,
        }
    }

    pub(crate) fn update_player_position(&mut self) {
        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                if c.tile == '@' {
                    self.player_position = Vec2::new(row_idx, col_idx);
                    break;
                }
            }
        }
    }

    pub(crate) fn set_player_position(&mut self, pos_x: usize, pos_y: usize) {

        let tile_set = &self.tile_set;
        let mut positions_to_modify = Vec::new();

        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                if row_idx == pos_x && col_idx == pos_y {
                    if self.map[row_idx][col_idx].tile != tile_set.wall && (row_idx == pos_x && col_idx == pos_y) {
                        positions_to_modify.push((pos_x, pos_y));
                    } else if self.map[row_idx+1][col_idx+1].tile != tile_set.wall && self.map[row_idx+1][col_idx+1].tile != tile_set.closed_door_side {
                        positions_to_modify.push((pos_x+1, pos_y+1));
                    }
                }
            }
        }

        self.player_position = Vec2::new(pos_x, pos_y);
        self.map[pos_x][pos_y].tile = tile_set.player;
        self.set_player_vision(Vec2::new(pos_x, pos_y));
    }

    pub(crate) fn set_player_vision(&mut self, player_pos: Vec2) {
        let vision_radius: isize = 2;

        for x in 0..self.map_width {
            for y in 0..self.map_height {
                self.map[x][y].is_visible = false;

                if self.map[x][y].is_solid || self.map[x][y].tile == DEFAULT_TILE_SET.open_door {

                } else {

                }
            }
        }

        macro_rules! calc_dir {
        ($x_offset:literal, $y_offset:literal) => {
            for i in 1..vision_radius+1 {
                let x = player_pos.x.wrapping_add((i * $x_offset) as usize);
                let y = player_pos.y.wrapping_add((i * $y_offset) as usize);
                if x < 0 || x > self.map_width || y < 0 || y > self.map_height {
                    break;
                }
                let tile = &mut self.map[x][y];
                tile.is_visible = true;

                if tile.is_solid && tile.tile != DEFAULT_TILE_SET.open_door {
                    break;
                }
            }
        };
    }
        calc_dir!(1, 0);  // Right
        calc_dir!(-1, 0); // Left
        calc_dir!(0, 1);  // Down
        calc_dir!(0, -1); // Up

        calc_dir!(1, 1);   // Right-Down
        calc_dir!(1, -1);  // Right-Up
        calc_dir!(-1, 1);  // Left-Down
        calc_dir!(-1, -1); // Left-Up
    }

    pub(crate) fn set_map_tile_set(&mut self, tile_set: TileSet) {
        self.tile_set = tile_set;
    }

    pub(crate) fn update_tile_below_player(&mut self, tile: char, x_coord: usize, y_coord: usize) {
        self.tile_below_player = tile;
    }

    pub(crate) fn print_map(&self) {
        let mut stdout = stdout();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();

        for tile in &self.map {
            let tile_line : String = tile.iter().map(|space| space.tile).collect();
            println!("{}", tile_line);
        }
    }

    fn print_map_with_module(&self, module: &[String]) {
        let mut stdout = stdout();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        let mut counter = 0;
        for tile in &self.map {
            let tile_line : String = tile.iter().map(|space| space.tile).collect();
            if counter <= module.len()-1 {
                println!("{}          {}", tile_line, module[counter]);
                counter += 1;
            } else {
                println!("{}", tile_line);
            }
        }
        println!("\n");
    }

    pub(crate) fn update_str_map_with_modules(&mut self, module: &[[String; 3]; 2]) {
        let mut counter = 0;
        self.str_map = String::new();
        for tile in &self.map {
            let tile_line : String = tile.iter().map(|space| space.tile).collect();
            if counter <= module.len() {
                self.str_map += &*format!("{}      {}      {}", tile_line, module[0][counter], module[1][counter]);
                counter += 1;
            } else {
                self.str_map += &*format!("{}", tile_line);
            }
            self.str_map += &*format!("\n");
        }
    }

    pub(crate) fn get_current_floor_to_size(&mut self, size: usize) -> [String; 3] {
        let mut module_pieces = [format!("FLOOR: {}", self.current_floor), String::new(), String::new()];

        for i in 1..size {
            module_pieces[i] = String::new();
        }

        module_pieces
    }

    pub(crate) fn get_tile_at_position(&self, position: Option<(usize, usize)>) -> char {
        if let Some((row, col)) = position {
            return self.map[row][col].tile;
        }
        ' '
    }
}

