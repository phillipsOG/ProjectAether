use crate::map_data::Vec2;
use crate::monster::Monster;
use crate::tile_set::MONSTER_TILE_SET;

pub struct MonsterFactory {}

impl MonsterFactory {
    pub(crate) fn new() -> Self {
        MonsterFactory {}
    }

    pub fn generate_monster(&mut self, pos: Vec2) -> Monster {
        Monster::new_set_position(MONSTER_TILE_SET.snake, pos)
    }
}
