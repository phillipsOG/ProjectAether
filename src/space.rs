use crate::tile_set::{DEFAULT_TILE_SET, LADDER_TILE_SET, MONSTER_TILE_SET};
use std::fmt;
use std::ops::Deref;
use sdl2::rect::{Point, Rect};

#[derive(Clone, Copy)]
pub struct Space {
    pub tile_name: &'static str,
    pub tile_sprite_position: Point,
    pub tile_height: u32,
    pub tile_width: u32,
    pub travel_cost: usize,
    pub is_visible: bool,
    pub is_solid: bool,
    pub is_traversable: bool,
    pub is_monster: bool,
    pub is_player: bool,
    pub is_occupied: bool,
    pub is_item: bool
}

impl Space {
    pub(crate) fn new(tile_name: &'static str) -> Self {
        Space {
            tile_name,
            tile_sprite_position: Point::new(0, 0),
            tile_height: 0,
            tile_width: 0,
            travel_cost: Space::calculate_tile_cost(tile_name),
            is_visible: false,
            is_solid: tile_name == DEFAULT_TILE_SET.wall
                || tile_name == DEFAULT_TILE_SET.closed_door_side
                || tile_name == DEFAULT_TILE_SET.closed_door_top
                || tile_name == MONSTER_TILE_SET.snake
                || tile_name == MONSTER_TILE_SET.goblin
                || tile_name == DEFAULT_TILE_SET.player,
            is_traversable: tile_name == DEFAULT_TILE_SET.floor
                || tile_name == DEFAULT_TILE_SET.open_door
                || tile_name == LADDER_TILE_SET.floor,
            is_monster: tile_name == MONSTER_TILE_SET.snake || tile_name == MONSTER_TILE_SET.goblin,
            is_player: tile_name == DEFAULT_TILE_SET.player,
            is_occupied: false,
            is_item: false,
        }
    }

    fn calculate_tile_cost(tile_name: &'static str) -> usize {
        if tile_name == DEFAULT_TILE_SET.floor {
            1
        } else if tile_name == DEFAULT_TILE_SET.wall {
            3
        } else {
            0
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tile_name)
    }
}

impl Deref for Space {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.tile_name
    }
}
