use crate::status::Status;
use crate::Vec2;

#[derive(Copy, Clone)]
pub struct Monster {
    pub id: i32,
    pub tile: char,
    pub status: Status,
    pub position: Vec2,
}

impl Monster {
    pub(crate) fn new(tile: char, id: i32) -> Self {
        Monster {
            tile,
            status: Status::new_monster(10, 3, 1),
            id: Monster::generate_id(id),
            position: Vec2::ZERO,
        }
    }

    pub(crate) fn new_set_position(tile: char, position: Vec2, id: i32) -> Self {
        Monster {
            tile,
            status: Status::new_monster(10, 3, 1),
            id: Monster::generate_id(id),
            position,
        }
    }

    fn generate_id(id: i32) -> i32 {
        id
    }
}
