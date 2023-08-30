use crate::vec2::Vec2;

#[derive(Copy, Clone)]
pub struct MonsterData {
    pub id: i32,
    pub position: Vec2
}

impl MonsterData {
    pub(crate) fn new(id: i32, position: Vec2) -> Self {
        MonsterData {
            id,
            position
        }
    }
}