use sdl2::rect::Point;
use crate::space::Space;
use crate::tile_set::{DEFAULT_TILE_SET, MONSTER_TILE_SET};

pub (crate) struct SpaceFactory;

impl SpaceFactory {
    pub (crate) fn generate_space(tile_name: &str) -> Space {
        if tile_name == DEFAULT_TILE_SET.wall {
            let mut new_space = Space::new(DEFAULT_TILE_SET.wall);
            new_space.tile_sprite_position = Point::new(160, 20);
            new_space.tile_width = 60;
            new_space.tile_height = 60;
            new_space
        }
        else if tile_name == DEFAULT_TILE_SET.floor {
            let mut new_space = Space::new(DEFAULT_TILE_SET.floor);
            new_space.tile_sprite_position = Point::new(180, 30);
            new_space.tile_width = 20;
            new_space.tile_height = 20;
            new_space
        }
        else if tile_name == DEFAULT_TILE_SET.closed_door_side {
            let mut new_space = Space::new(DEFAULT_TILE_SET.closed_door_side);
            new_space.tile_sprite_position = Point::new(475, 85);
            new_space.tile_width = 15;
            new_space.tile_height = 30;
            new_space
        }
        else if tile_name == DEFAULT_TILE_SET.open_door {
            let mut new_space = Space::new(DEFAULT_TILE_SET.open_door);
            new_space.tile_sprite_position = Point::new(510, 85);
            new_space.tile_width = 15;
            new_space.tile_height = 30;
            new_space
        }
        else if tile_name == MONSTER_TILE_SET.snake {
            let mut new_space = Space::new(MONSTER_TILE_SET.snake);
            new_space.tile_sprite_position = Point::new(495, 125);
            new_space.tile_width = 75;
            new_space.tile_height = 60;
            new_space
        }
        else if tile_name == DEFAULT_TILE_SET.key {
            let mut key = Space::new(DEFAULT_TILE_SET.key);
            key.tile_width = 45;
            key.tile_height = 60;
            key.tile_sprite_position = Point::new(460, 20);
            key
        }
        else {
            Space::new(DEFAULT_TILE_SET.floor)
        }
    }
}
