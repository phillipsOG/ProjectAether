use rand::Rng;
use crate::map_manager::MapManager;
use crate::monster_generator::{Monster, MonsterGenerator};
use crate::space::Space;
use crate::tile_set::DEFAULT_TILE_SET;

type Monsters = Vec<Monster>;

pub struct MonsterManager {
    monsters: Monsters
}

impl MonsterManager {
    pub(crate) fn new() -> Self {
        MonsterManager {
            monsters: vec![],
        }
    }

    pub(crate) fn spawn_monsters(&mut self, map_manager: &mut MapManager, monster_generator: &mut MonsterGenerator) {
        let mut current_map = map_manager.get_map_mut(map_manager.current_map_index);

        if let Some(map_data) = current_map {
            let map_height = map_data.map_height;
            let map_width = map_data.map_width;

            let mut rng = rand::thread_rng();

            for pos_x in 0..map_height {
                for pos_y in 0..map_width {
                    let mut current_tile = &mut map_data.map[pos_x][pos_y];

                    if !current_tile.is_solid && current_tile.tile == DEFAULT_TILE_SET.floor && rng.gen_range(0..10) > 8 {
                        self.monsters.push(monster_generator.generate_monster());

                        *current_tile = Space::new(self.monsters[self.monsters.len()-1].monster);
                    }
                }
            }
        }
    }
}