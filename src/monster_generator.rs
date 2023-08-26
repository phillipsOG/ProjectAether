use crate::tile_set::MONSTER_TILE_SET;

pub struct MonsterGenerator {}

impl MonsterGenerator {
    pub(crate) fn new() -> Self {
        MonsterGenerator {}
    }

    pub fn generate_monster(&mut self) -> char {
        MONSTER_TILE_SET.snake
    }
}

#[derive(Clone)]
pub struct Monster {
    pub monster: char, 
    pub health: usize,
    pub str: usize,
    pub defence: usize
}

impl Monster {
    pub(crate) fn new(monster_tile: char) -> Self {
        Monster {
            monster: monster_tile,
            health: 10,
            str: 4,
            defence: 1,
        }
    }
}
