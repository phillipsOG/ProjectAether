use std::collections::HashMap;
use futures::lock::MutexGuard;

use crate::monster::Monster;
use crate::monster_generator::MonsterFactory;

use crate::tile_set::{DEFAULT_TILE_SET, MONSTER_TILE_SET};
use crate::Vec2;

use crate::map_manager::MapManager;
use rand::Rng;

use crate::space::Space;

type Monsters = HashMap<i32, Monster>;

#[derive(Clone)]
pub struct MonsterManager {
    monsters: Monsters,
}

impl MonsterManager {
    pub(crate) fn new() -> Self {

        MonsterManager {
            monsters: HashMap::<i32, Monster>::new()
        }
    }

    pub(crate) fn spawn_monsters(
        &mut self,
        map_manager_clone: &mut MutexGuard<MapManager>,
        mut monster_factory: MonsterFactory,
    ) {
        let map_index = map_manager_clone.current_map_index;
        let map_data = map_manager_clone.get_map_mut(map_index).expect("map data");
        let map_height = map_data.height;
        let map_width = map_data.width;

        let mut rng = rand::thread_rng();
        let mut spawn_one = false;
        let mut loop_limit = 0;

        for pos_y in 0..map_height {
            for pos_x in 0..map_width {
                let current_tile = map_data.map[pos_y][pos_x];
                let mut monster_type = MONSTER_TILE_SET.goblin;

                if rng.gen::<f64>() < 0.5 {
                    monster_type = MONSTER_TILE_SET.goblin
                } else {
                    monster_type = MONSTER_TILE_SET.snake
                };

                if !current_tile.is_solid
                    && current_tile.tile == DEFAULT_TILE_SET.floor
                    && loop_limit != 8
                /*spawn_onerng.gen_range(0..10) >= 9*/
                {
                    let mut new_monster = monster_factory
                        .generate_monster(Vec2::new(pos_x, pos_y), (self.monsters.len()) as i32, monster_type);

                    new_monster.tile_below = DEFAULT_TILE_SET.floor;
                    new_monster.position = Vec2::new(pos_x, pos_y);
                    map_data.map[pos_y][pos_x] = Space::new(new_monster.tile);
                    self.monsters.insert(new_monster.id, new_monster);
                    loop_limit += 1;
                }
            }
        }
    }

    pub(crate) fn get_monsters(self) -> Monsters {
        self.monsters
    }

    pub(crate) fn get_monsters_mut(&mut self) -> &mut Monsters {
        &mut self.monsters
    }

    pub(crate) fn get_monster(&mut self, id: &i32) -> Option<&Monster> {
        self.monsters.get(id)
    }

    pub(crate) fn get_monster_mut(&mut self, id: &i32) -> Option<&mut Monster> {
        self.monsters.get_mut(id)
    }

    pub(crate) fn get_monster_at_position(&mut self, position: Vec2) -> Option<&mut Monster> {
        for monster in self.monsters.values_mut() {
            if monster.position == position {
                return Some(monster);
            }
        }
        None
    }
    
    pub(crate) fn despawn(&mut self, monster_id: i32) {
        self.monsters.remove(&monster_id);
    }
}
