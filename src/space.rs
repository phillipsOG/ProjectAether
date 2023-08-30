use crate::tile_set::{DEFAULT_TILE_SET, LADDER_TILE_SET, MONSTER_TILE_SET};
use std::fmt;
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct Space {
    pub tile: char,
    pub is_visible: bool,
    pub is_solid: bool,
    pub is_traversable: bool,
}

impl Space {
    pub(crate) fn new(tile: char) -> Self {
        Space {
            tile,
            is_visible: false,
            is_solid: tile == DEFAULT_TILE_SET.wall
                || tile == DEFAULT_TILE_SET.closed_door_side
                || tile == DEFAULT_TILE_SET.closed_door_top
                || tile == MONSTER_TILE_SET.snake
                || tile == DEFAULT_TILE_SET.player,
            is_traversable: tile == DEFAULT_TILE_SET.floor
                || tile == DEFAULT_TILE_SET.open_door
                || tile == LADDER_TILE_SET.floor,
        }
    }

    pub(crate) fn from_char(tile: char) -> Self {
        Space {
            tile,
            is_visible: false,
            is_solid: tile == DEFAULT_TILE_SET.wall
                || tile == DEFAULT_TILE_SET.closed_door_side
                || tile == DEFAULT_TILE_SET.closed_door_top
                || tile == MONSTER_TILE_SET.snake
                || tile == DEFAULT_TILE_SET.player,
            is_traversable: tile == DEFAULT_TILE_SET.floor
                || tile == DEFAULT_TILE_SET.open_door
                || tile == LADDER_TILE_SET.floor,
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
