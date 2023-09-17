use crate::monster::Monster;
use crate::tile_set::{DEFAULT_TILE_SET, MONSTER_TILE_SET, MonsterTileSet};
use crate::Vec2;

#[derive(Clone)]
pub struct MonsterFactory {}

impl MonsterFactory {
    pub(crate) fn new() -> Self {
        MonsterFactory {}
    }

    pub fn generate_monster(&mut self, pos: Vec2, id: i32, monster_type: &'static str) -> Monster {
        Monster::new(monster_type, MonsterFactory::get_monster_name(monster_type),pos, id)
    }

    fn get_monster_name(monster_tile: &'static str) -> &'static str {
        if monster_tile == MONSTER_TILE_SET.snake {
            return "Snake";
        }
        "MISSINGNO"
    }
}
