use crate::tile_set::DEFAULT_TILE_SET;
use std::fmt;
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct Space {
    pub tile: char,
    pub is_visible: bool,
    pub is_solid: bool,
}

impl Space {
    pub(crate) fn new(tile: char) -> Self {
        Space {
            tile,
            is_visible: false,
            is_solid: tile == DEFAULT_TILE_SET.wall
                || tile == DEFAULT_TILE_SET.closed_door_side
                || tile == DEFAULT_TILE_SET.closed_door_top,
        }
    }

    pub(crate) fn from_char(tile: char) -> Self {
        Space {
            tile,
            is_visible: false,
            is_solid: tile == DEFAULT_TILE_SET.wall
                || tile == DEFAULT_TILE_SET.closed_door_side
                || tile == DEFAULT_TILE_SET.closed_door_top,
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tile)
    }
}

impl Deref for Space {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.tile
    }
}
