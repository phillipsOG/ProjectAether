use crate::status::Status;
use crate::Vec2;

#[derive(Copy, Clone)]
pub struct Monster {
    pub id: i32,
    pub tile: char,
    pub status: Status,
    pub position: Vec2,
    pub tile_below_monster: char,
}

impl Monster {
    pub(crate) fn new(tile: char, id: i32) -> Self {
        Monster {
            tile,
            status: Status::new_monster(10, 3, 1),
            id: Monster::generate_id(id),
            position: Vec2::ZERO,
            tile_below_monster: ' ',
        }
    }

    pub(crate) fn new_set_position(tile: char, position: Vec2, id: i32) -> Self {
        Monster {
            tile,
            status: Status::new_monster(10, 3, 1),
            id: Monster::generate_id(id),
            position,
            tile_below_monster: ' ',
        }
    }

    fn generate_id(id: i32) -> i32 {
        id
    }

    pub(crate) fn update_tile_below_monster(&mut self, tile: char) {
        self.tile_below_monster = tile;
    }

    pub(crate) fn get_tile_below_monster(self) -> char {
        return self.tile_below_monster;
    }
}
