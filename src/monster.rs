use crate::status::Status;
use crate::Vec2;

#[derive(Copy, Clone)]
pub struct Monster {
    pub id: i32,
    pub tile: &'static str,
    pub name: &'static str,
    pub status: Status,
    pub position: Vec2,
    pub tile_below: &'static str,
    pub in_battle: bool,
    pub is_alive: bool
}

impl Monster {
    pub(crate) fn new(tile: &'static str, name: &'static str, position: Vec2, id: i32) -> Self {
        Monster {
            tile,
            name,
            status: Status::new_monster(10, 3, 1),
            id: Monster::generate_id(id),
            position,
            tile_below: " ",
            in_battle: false,
            is_alive: true
        }
    }

    fn generate_id(id: i32) -> i32 {
        id
    }

    pub(crate) fn update_tile_below_monster(&mut self, tile: &'static str) {
        self.tile_below = tile;
    }

    pub(crate) fn get_tile_below_monster(self) -> &'static str {
        return self.tile_below;
    }
}
