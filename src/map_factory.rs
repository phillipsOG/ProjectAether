use crate::chat::Chat;
use crate::map_data::MapData;

use crate::player::Player;
use crate::space::Space;
use crate::terrain_data::TerrainData;
use crate::tile_set::DEFAULT_TILE_SET;
use crate::Vec2;

use crate::map_manager::MapManager;
use futures::lock::{Mutex, MutexGuard};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct MapFactory {}

impl MapFactory {
    pub(crate) fn new() -> Self {
        MapFactory {}
    }

    pub(crate) fn generate_map(
        &mut self,
        player: &mut Player,
        height: usize,
        width: usize,
        pos: Vec2,
        seed_phrase: &str,
    ) -> MapData {
        let mut map = "".to_owned();

        let mut hasher = DefaultHasher::new();
        seed_phrase.hash(&mut hasher);
        let seed = hasher.finish();

        // create a seeded RNG using the generated seed

        for pos_y in 0..height {
            for pos_x in 0..width {
                let mut rng = rand::thread_rng();
                let y: f64 = rng.gen(); // generates a float between 0 and 1

                let tile =
                    if pos_y == 0 && pos_x > 0 || pos_y > 0 && pos_x == 0 || pos_y == height - 1 {
                        DEFAULT_TILE_SET.wall
                    } else {
                        // generate a random value based on the seeded RNG
                        // for example, generating floor tiles with a certain probability
                        if y < 0.8 {
                            DEFAULT_TILE_SET.floor
                        } else if y < 0.6 {
                            DEFAULT_TILE_SET.key
                        } else {
                            DEFAULT_TILE_SET.wall
                        }
                    };
                map += &*format!("{}", tile);
            }
            map += &*format!("\n");
        }

        let map_lines: Vec<&str> = map.trim().lines().collect();
        let mut new_map = MapData::new();
        new_map.map = map_lines
            .iter()
            .map(|line| line.chars().map(Space::new).collect())
            .collect();

        player.position = pos;
        player.tile_below_player = DEFAULT_TILE_SET.floor;
        new_map.set_player_position(pos);

        new_map.width = new_map.map.len();
        new_map.height = if new_map.width > 0 {
            new_map.map[0].len()
        } else {
            0
        };

        return new_map;
    }

    pub(crate) async fn generate_terrain<'a>(
        &mut self,
        map_manager_guard: &mut MutexGuard<'a, MapManager>,
        new_player_position: Vec2,
        chat_clone: &mut Arc<Mutex<Chat>>,
    ) -> Option<TerrainData> {
        let mut terrain_data = TerrainData::new();

        /*let mut hasher = DefaultHasher::new();
        "seedphrase".hash(&mut hasher);
        let seed = hasher.finish();
        let mut rng = StdRng::seed_from_u64(seed);*/
        let mut chat = chat_clone.lock().await;
        let map_index = map_manager_guard.current_map_index;
        let map = map_manager_guard.get_map(map_index).expect("map data");
        if new_player_position.x >= map.width - 1 {
            if new_player_position.x >= 20 && new_player_position.y >= 10 {
                terrain_data.width_increase = 10;
                terrain_data.height_increase = 10;
            } else {
                terrain_data.width_increase = 1;
            }
        } else if new_player_position.y >= map.height - 1 {
            terrain_data.height_increase = 1;
        } else {
            return None;
        }

        let mut updated_map_data = self.get_new_map_size(
            new_player_position,
            map.height + terrain_data.height_increase,
            map.width + terrain_data.width_increase,
        );

        for (pos_y, row) in map.map.iter().enumerate() {
            for (pos_x, _space) in row.iter().enumerate() {
                if pos_y < map.width && pos_x == 0 || pos_x > map.height && pos_y == 0 {
                    updated_map_data[pos_y][pos_x] = Space::new(DEFAULT_TILE_SET.wall);
                } else {
                    updated_map_data[pos_y][pos_x] = _space.clone();
                }

                if new_player_position.x >= map.width - terrain_data.width_increase {
                    //chat.process_chat_message("spawn building here");
                    updated_map_data[pos_y + terrain_data.height_increase]
                        [pos_x + terrain_data.width_increase] =
                        self.generate_terrain_building(pos_y, pos_x);
                }
                if terrain_data.width_increase == 10 {}
            }
        }
        terrain_data.map = updated_map_data;

        return Some(terrain_data);
    }

    fn generate_terrain_building(&mut self, _pos_y: usize, _pos_x: usize) -> Space {
        let mut map = "".to_owned();

        map += "\n";
        if let Ok(lines) = self.read_lines("src/maps/map1.txt") {
            for line in lines {
                if let Ok(tile) = line {
                    map += &tile;
                    map += "\n";
                }
            }
        }
        let map_lines: Vec<&str> = map.trim().lines().collect();
        let mut structure = MapData::new();
        structure.map = map_lines
            .iter()
            .map(|line| line.chars().map(Space::new).collect())
            .collect();
        structure.height = structure.map.len();
        structure.width = if structure.height > 0 {
            structure.map[0].len()
        } else {
            0
        };

        for (pos_y, row) in structure.map.iter().enumerate() {
            for (pos_x, _space) in row.iter().enumerate() {
                if pos_y + 2 == _pos_y && _pos_x == pos_x {
                    return structure.map[pos_y][pos_x];
                }
            }
        }
        return Space::new(DEFAULT_TILE_SET.floor);
    }

    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn get_new_map_size(
        &self,
        new_player_position: Vec2,
        map_height: usize,
        map_width: usize,
    ) -> Vec<Vec<Space>> {
        if new_player_position.y >= map_height - 1 {
            return vec![vec![Space::new('.'); map_width]; map_height + 1];
        } else if new_player_position.x >= map_width - 1 {
            return vec![vec![Space::new('.'); map_width + 1]; map_height];
        }

        let new_map_data = vec![vec![Space::new('.'); map_width]; map_height];
        new_map_data
    }
}
