use futures::lock::MutexGuard;
use std::collections::HashMap;

use crate::monster::Monster;
use crate::monster_generator::MonsterFactory;

use crate::tile_set::{DEFAULT_TILE_SET, MONSTER_TILE_SET};
use crate::{Monsters, Vec2};

use crate::map_manager::MapManager;
use rand::Rng;

use crate::space_factory::SpaceFactory;

#[derive(Clone)]
pub struct MonsterManager {
    monsters: Monsters,
}

impl MonsterManager {
    pub(crate) fn new() -> Self {
        MonsterManager {
            monsters: HashMap::<i32, Monster>::new(),
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
        let _spawn_one = false;
        let mut loop_limit = 0;

        for pos_y in 0..map_height {
            for pos_x in 0..map_width {
                let current_tile = &map_data.map[pos_y][pos_x];
                let mut monster_type = MONSTER_TILE_SET.snake;

                if rng.gen::<f64>() < 0.5 {
                    monster_type = MONSTER_TILE_SET.snake
                } else {
                    monster_type = MONSTER_TILE_SET.snake
                };

                if !current_tile.is_solid
                    && current_tile.tile_name == DEFAULT_TILE_SET.floor
                    && loop_limit != 2
                    && pos_x > 4
                /*spawn_onerng.gen_range(0..10) >= 9*/
                {
                    let mut new_monster = monster_factory.generate_monster(
                        Vec2::new(pos_x, pos_y),
                        (self.monsters.len()) as i32,
                        monster_type,
                    );

                    new_monster.tile_below = DEFAULT_TILE_SET.floor;
                    new_monster.position = Vec2::new(pos_x, pos_y);
                    map_data.map[pos_y][pos_x] = SpaceFactory::generate_space(new_monster.tile);
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

    pub(crate) fn cull_dead_monsters(&mut self) -> Vec<Vec2> {
        let mut monsters_to_cull = Vec::<i32>::new();
        let mut culled_monsters_positions = Vec::<Vec2>::new();

        for monster in self.get_monsters_mut().values_mut() {
            if !monster.is_alive {
                monsters_to_cull.push(monster.id);
                culled_monsters_positions.push(monster.position);
            }
        }
        for monster_id in monsters_to_cull {
            self.despawn(monster_id);
        }

        culled_monsters_positions
    }
}
