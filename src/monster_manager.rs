use crate::map_data::MapData;
use crate::map_manager::MapManager;
use crate::monster::Monster;
use crate::monster_generator::MonsterFactory;
use crate::space::Space;
use crate::tile_set::DEFAULT_TILE_SET;
use crate::Vec2;
use futures::lock::MutexGuard;
use rand::Rng;

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
        map_data: &mut MapData,
        monster_factory: &mut MonsterFactory,
    ) {
        let map_height = map_data.map_height;
        let map_width = map_data.map_width;

        let mut rng = rand::thread_rng();
        let mut spawn_one = false;

        for pos_y in 0..map_height {
            for pos_x in 0..map_width {
                let current_tile = &mut map_data.map[pos_y][pos_x];

                if !current_tile.is_solid && current_tile.tile == DEFAULT_TILE_SET.floor && /*spawn_one*/rng.gen_range(0..10) > 8
                {
                    /*if pos_y == 3 {*/
                    let new_monster = monster_factory
                        .generate_monster(Vec2::new(pos_x, pos_y), (self.monsters.len()) as i32);
                    self.monsters.push(new_monster);
                    *current_tile = Space::new(self.monsters[self.monsters.len() - 1].tile);
                    map_data
                        .tile_below_monsters
                        .insert(new_monster.id, DEFAULT_TILE_SET.floor);
                    map_data
                        .monster_positions
                        .insert(new_monster.id, new_monster.position);
                    spawn_one = true;
                    /*}*/
                }
            }
        }
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
