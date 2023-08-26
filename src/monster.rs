use crate::map_data::Vec2;
use crate::status::Status;

#[derive(Copy, Clone)]
pub struct Monster {
    pub tile: char,
    pub status: Status,
    pub position: Vec2
}

impl Monster {

    pub(crate) fn new(tile: char) -> Self {
        Monster {
            tile,
            status: Status::new_monster(10, 3, 1),
            position: Vec2::ZERO
        }
    }

    pub(crate) fn new_set_position(tile: char, position: Vec2) -> Self {
        Monster {
            tile,
            status: Status::new_monster(10, 3, 1),
            position
        }
    }
}
