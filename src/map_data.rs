use crate::space::Space;
use crate::tile_set::{TileSet, DEFAULT_TILE_SET, MONSTER_TILE_SET};
use crate::vec2::Vec2;
use crate::Map;
use crossterm::{terminal, QueueableCommand};
use std::collections::HashMap;
use std::io::stdout;

#[derive(Clone)]
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
    pub map_height: usize,
    pub fog_of_war: bool,
    pub monster_positions: HashMap<i32, Vec2>,
    pub tile_below_monsters: HashMap<i32, char>,
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
            fog_of_war: true,
            monster_positions: Default::default(),
            tile_below_monsters: Default::default(),
        }
    }

    pub(crate) fn set_player_position(&mut self, pos: Vec2) {
        let tile_set = &self.tile_set;
        self.map[pos.y][pos.x] = Space::new(tile_set.player);
        self.player_position = pos;
        self.set_player_vision(pos);
    }

    pub(crate) fn set_player_vision(&mut self, _player_pos: Vec2) {
        for y in 0..self.map_height {
            for x in 0..self.map_width {
                /*println!(
                    "height: {}, width: {}, map_height: {}",
                    self.map_height,
                    self.map_width,
                    self.map.len()
                );*/
                self.map[y][x].is_visible = self.fog_of_war;

                /*if self.map[x][y].is_solid || self.map[x][y].tile == DEFAULT_TILE_SET.open_door {
                } else {
                }*/
            }
        }

        self.calculate_vision_at_position(1, 0);
        self.calculate_vision_at_position(-1, 0);
        self.calculate_vision_at_position(0, 1);
        self.calculate_vision_at_position(0, -1);

        self.calculate_vision_at_position(1, 1);
        self.calculate_vision_at_position(1, -1);
        self.calculate_vision_at_position(-1, 1);
        self.calculate_vision_at_position(-1, -1);
    }

    fn calculate_vision_at_position(&mut self, pos_x: i32, pos_y: i32) {
        let vision_radius: isize = 2; //set to 2

        for i in 1..vision_radius + 1 {
            let y = self
                .player_position
                .y
                .wrapping_add((pos_y * i as i32) as usize);
            let x = self
                .player_position
                .x
                .wrapping_add((pos_x * i as i32) as usize);

            if x >= self.map_width || y >= self.map_height {
                break;
            }

            let tile = &mut self.map[y][x];
            tile.is_visible = true; //if tile.tile == MONSTER_TILE_SET.snake { false } else { true};

            if tile.is_solid && tile.tile != DEFAULT_TILE_SET.open_door {
                break;
            }
        }
    }

    pub(crate) fn set_monster_position(&mut self, monster_id: i32, new_pos: Vec2) {
        self.map[new_pos.y][new_pos.x] = Space::new(MONSTER_TILE_SET.snake);
        self.monster_positions.insert(monster_id, new_pos);
    }

    pub(crate) fn set_map_tile_set(&mut self, tile_set: TileSet) {
        self.tile_set = tile_set;
    }

    pub(crate) fn update_tile_below_player(&mut self, tile: char) {
        self.tile_below_player = tile;
    }

    pub(crate) fn update_tile_below_monster(&mut self, monster_id: i32, tile: char) {
        self.tile_below_monsters.insert(monster_id, tile);
    }

    pub(crate) fn get_tile_below_monster(&mut self, monster_id: i32) -> Option<&mut char> {
        self.tile_below_monsters.get_mut(&monster_id)
    }

    pub(crate) fn get_monster() {}

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
        let mut module_pieces = [
            format!("FLOOR: {}", self.current_floor),
            String::new(),
            String::new(),
        ];

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
