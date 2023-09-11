use crate::player::Player;
use crate::space::Space;
use crate::tile_set::{TileSet, DEFAULT_TILE_SET, MONSTER_TILE_SET};
use crate::vec2::Vec2;
use crate::Map;
use crossterm::{terminal, QueueableCommand};

use std::io::stdout;

#[derive(Clone)]
pub struct MapData {
    pub map: Map,
    pub str_map: String,
    pub tile_set: TileSet,
    pub width: usize,
    pub height: usize,
}

impl MapData {
    pub(crate) fn new() -> Self {
        MapData {
            map: Map::new(),
            str_map: String::new(),
            tile_set: DEFAULT_TILE_SET,
            width: 0,
            height: 0,
        }
    }

    pub(crate) fn set_player_position(&mut self, pos: Vec2) {
        let tile_set = &self.tile_set;
        self.map[pos.y][pos.x] = Space::new(tile_set.player);
    }

    pub(crate) fn set_player_vision(&mut self, player: &Player, _player_pos: Vec2) {
        for y in 0..self.height {
            for x in 0..self.width {
                /*println!(
                    "height: {}, width: {}, map_height: {}",
                    self.map_height,
                    self.map_width,
                    self.map.len()
                );*/
                self.map[y][x].is_visible = player.fog_of_war;

                /*if self.map[x][y].is_solid || self.map[x][y].tile == DEFAULT_TILE_SET.open_door {
                } else {
                }*/
            }
        }

        self.calculate_vision_at_position(player, 1, 0);
        self.calculate_vision_at_position(player, -1, 0);
        self.calculate_vision_at_position(player, 0, 1);
        self.calculate_vision_at_position(player, 0, -1);

        self.calculate_vision_at_position(player, 1, 1);
        self.calculate_vision_at_position(player, 1, -1);
        self.calculate_vision_at_position(player, -1, 1);
        self.calculate_vision_at_position(player, -1, -1);
    }

    fn calculate_vision_at_position(&mut self, player: &Player, pos_x: i32, pos_y: i32) {
        let vision_radius: isize = 2; //set to 2

        for i in 1..vision_radius + 1 {
            let y = player.position.y.wrapping_add((pos_y * i as i32) as usize);
            let x = player.position.x.wrapping_add((pos_x * i as i32) as usize);

            if x >= self.width || y >= self.height {
                break;
            }

            let tile = &mut self.map[y][x];
            tile.is_visible = true; //if tile.tile == MONSTER_TILE_SET.snake { false } else { true};

            if tile.is_solid && tile.tile != DEFAULT_TILE_SET.open_door {
                break;
            }
        }
    }

    pub(crate) fn set_monster_position(&mut self, new_pos: Vec2, monster_type: char) {
        self.map[new_pos.y][new_pos.x] = Space::new(monster_type);
    }

    pub(crate) fn set_map_tile_set(&mut self, tile_set: TileSet) {
        self.tile_set = tile_set;
    }

    pub(crate) fn print_map(&self) {
        let mut stdout = stdout();
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        for tile in &self.map {
            let tile_line: String = tile.iter().map(|space| space.tile).collect();
            println!("{}", tile_line);
        }
    }

    fn print_map_with_module(&self, module: &[String]) {
        let mut stdout = stdout();
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        let mut counter = 0;
        for tile in &self.map {
            let tile_line: String = tile.iter().map(|space| space.tile).collect();
            if counter <= module.len() - 1 {
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
            let tile_line: String = tile.iter().map(|space| space.tile).collect();
            if counter <= module.len() {
                self.str_map += &*format!(
                    "{}      {}      {}",
                    tile_line, module[0][counter], module[1][counter]
                );
                counter += 1;
            } else {
                self.str_map += &*format!("{}", tile_line);
            }
            self.str_map += &*format!("\n");
        }
    }

    pub(crate) fn get_current_floor_to_size(&mut self, size: usize) -> [String; 3] {
        let mut module_pieces = [format!("FLOOR: {}", 0), String::new(), String::new()];

        for i in 1..size {
            module_pieces[i] = String::new();
        }

        module_pieces
    }

    pub(crate) fn get_tile_at_position(&self, position: Option<(usize, usize)>) -> char {
        if let Some((col, row)) = position {
            return self.map[col][row].tile;
        }
        ' '
    }
}
