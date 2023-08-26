use crate::status::Status;

pub struct Monster {
    pub monster: char,
    pub status: Status,
}

impl Monster {
    pub(crate) fn new(monster_tile: char) -> Self {
        Monster {
            monster: monster_tile,
            status: Status::new(),
        }
    }
}
