use futures::lock::{MutexGuard};


use crate::monster::Monster;
use crate::monster_generator::MonsterFactory;

use crate::tile_set::DEFAULT_TILE_SET;
use crate::Vec2;

use crate::map_manager::MapManager;
use rand::Rng;

use crate::space::Space;

type Monsters = Vec<Monster>;

#[derive(Clone)]
pub struct MonsterManager {
    monsters: Monsters,
}

impl MonsterManager {
    pub(crate) fn new() -> Self {
        MonsterManager { monsters: vec![] }
    }

    pub(crate) fn spawn_monsters(
        &mut self,
        map_manager_clone: &mut MutexGuard<MapManager>,
        mut monster_factory: MonsterFactory,
    ) {
        //let mut map_manager_guard = map_manager_clone.lock().await;
        let map_data = map_manager_clone.get_map_mut(0).expect("map data");
        let map_height = map_data.map_height;
        let map_width = map_data.map_width;

        let mut rng = rand::thread_rng();
        let mut spawn_one = false;

        for pos_y in 0..map_height {
            for pos_x in 0..map_width {
                let current_tile = map_data.map[pos_y][pos_x];

                if !current_tile.is_solid && current_tile.tile == DEFAULT_TILE_SET.floor && /*spawn_one*/rng.gen_range(0..10) > 8
                {
                    /*if pos_y == 3 {*/
                    let mut new_monster = monster_factory
                        .generate_monster(Vec2::new(pos_x, pos_y), (self.monsters.len()) as i32);
                    new_monster.tile_below_monster = DEFAULT_TILE_SET.floor;
                    new_monster.position = Vec2::new(pos_x, pos_y);
                    map_data.map[pos_y][pos_x] = Space::new(new_monster.tile);
                    self.monsters.push(new_monster);

                    spawn_one = true;
                    /*}*/
                }
            }
        }
        //drop(map_manager_guard);
    }

    pub(crate) fn get_monsters(self) -> Monsters {
        self.monsters
    }

    pub(crate) fn get_monsters_mut(&mut self) -> &mut Monsters {
        self.monsters.as_mut()
    }

    pub(crate) fn get_monster(&mut self, index: usize) -> Option<&Monster> {
        self.monsters.get(index)
    }

    pub(crate) fn get_monster_mut(&mut self, index: usize) -> Option<&mut Monster> {
        self.monsters.get_mut(index)
    }
}
