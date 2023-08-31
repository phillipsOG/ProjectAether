use crate::monster::Monster;
use crate::tile_set::MONSTER_TILE_SET;
use crate::Vec2;

#[derive(Clone)]
pub struct MonsterFactory {}

impl MonsterFactory {
    pub(crate) fn new() -> Self {
        MonsterFactory {}
    }

    pub fn generate_monster(&mut self, pos: Vec2, id: i32) -> Monster {
        Monster::new_set_position(MONSTER_TILE_SET.snake, pos, id)
    }
}
