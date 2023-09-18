use crate::basic_colour::COLOUR;
use crate::map_data::MapData;
use crate::player::Player;
use crate::tile_set::DEFAULT_TILE_SET;
use crate::{Direction, Map, Monsters};
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::vec2::Vec2;

pub(crate) struct Renderer {}

impl Renderer {
    pub(crate) fn render_player(
        canvas: &mut WindowCanvas,
        player: &Player,
        map_data: &MapData,
        _grid_x: i32,
        _grid_y: i32,
        camera_x: i32,
        camera_y: i32,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture("assets/bardo.png").unwrap();

        let (width, height) = canvas.output_size()?;

        //let screen_position = player.sprite_position + Point::new(width as i32 / 2, height as i32 / 2);

        let map_width = map_data.map[0].len() as i32;
        let map_height = map_data.map.len() as i32;
        let tile_width = 55;
        let tile_height = 60;

        for col in 0..map_width {
            for row in 0..map_height {
                let space = &map_data.map[row as usize][col as usize];
                if space.tile_name == DEFAULT_TILE_SET.player {
                    let (frame_width, frame_height) =
                        (player.sprite.width(), player.sprite.height());
                    let current_frame = Rect::new(
                        player.sprite.x() + frame_width as i32 * player.current_frame,
                        player.sprite.y()
                            + frame_height as i32
                                * Renderer::direction_spritesheet_row(player.direction),
                        frame_width,
                        frame_height,
                    );

                    let centered_row = (col * tile_width) + (width as i32 / 2)
                        - ((map_width * tile_width) / 2)
                        - camera_x;
                    let centered_col = (row * tile_height) + (height as i32 / 2)
                        - ((map_height * tile_height) / 2)
                        - camera_y;
                    let sprite = Rect::new(
                        centered_row,
                        centered_col,
                        tile_width as u32,
                        tile_height as u32,
                    );

                    canvas.copy(&texture, current_frame, sprite).unwrap();
                }
            }
        }

        Ok(())
    }

    pub(crate) fn render_tile(
        canvas: &mut WindowCanvas,
        map: &mut Map,
        tile_to_render: &str,
        _grid_x: i32,
        _grid_y: i32,
        tile_width: i32,
        tile_height: i32,
        camera_x: i32,
        camera_y: i32,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .load_texture("assets/dimension.png")
            .unwrap();

        let (screen_width, screen_height) = canvas.output_size()?;
        let map_width = map[0].len() as i32;
        let map_height = map.len() as i32;

        for col in 0..map_width {
            for row in 0..map_height {
                let space = &map[row as usize][col as usize];

                let sprite = Rect::new(
                    space.tile_sprite_position.x,
                    space.tile_sprite_position.y,
                    space.tile_width,
                    space.tile_height,
                );

                let centered_row = (col * tile_width) + (screen_width as i32 / 2)
                    - ((map_width * tile_width) / 2)
                    - camera_x;
                let centered_col = (row * tile_height) + (screen_height as i32 / 2)
                    - ((map_height * tile_height) / 2)
                    - camera_y;

                let sprite_build = Rect::new(
                    centered_row,
                    centered_col,
                    tile_width as u32,
                    tile_height as u32,
                );

                // if the ascii tile is the player, than set the tile underneath the player as a floor tile
                if space.tile_name == tile_to_render {
                    // if tile is wall, than use a single wall sprite
                    if col == 0 {
                        let sprite = Rect::new(
                            space.tile_sprite_position.x,
                            space.tile_sprite_position.y,
                            space.tile_width,
                            space.tile_height,
                        );
                        canvas.copy(&texture, sprite, sprite_build).unwrap();
                    } else {
                        // if tile is an object i.e. key, then draw the floor then when we draw the tiles for the objects
                        // they will draw on-top of the floor
                        canvas.copy(&texture, sprite, sprite_build).unwrap();
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) fn render_tiles(
        canvas: &mut WindowCanvas,
        graphical_map: &mut Map,
        player_map: &mut MapData,
        tiles_to_render: &Vec<&str>,
        _grid_x: i32,
        _grid_y: i32,
        tile_width: i32,
        tile_height: i32,
        camera_x: i32,
        camera_y: i32,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .load_texture("assets/dimension.png")
            .unwrap();

        let (screen_width, screen_height) = canvas.output_size()?;
        let map_width = player_map.width as i32;
        let map_height = player_map.height as i32;

        for col in 0..map_height {
            for row in 0..map_width {
                let graphical_space = &graphical_map[col as usize][row as usize];
                let player_map_space = &player_map.map[col as usize][row as usize];

                let sprite = Rect::new(
                    graphical_space.tile_sprite_position.x,
                    graphical_space.tile_sprite_position.y,
                    graphical_space.tile_width,
                    graphical_space.tile_height,
                );

                let centered_row = (row * tile_width) + (screen_width as i32 / 2)
                    - ((map_width * tile_width) / 2)
                    - camera_x;
                let centered_col = (col * tile_height) + (screen_height as i32 / 2)
                    - ((map_height * tile_height) / 2)
                    - camera_y;

                let sprite_build = Rect::new(
                    centered_row,
                    centered_col,
                    tile_width as u32,
                    tile_height as u32,
                );

                for tile_name in tiles_to_render.as_slice() {
                    // check if the tile is under the player and is visible
                    if player_map_space.is_player {
                        // draw the tile under the player
                        canvas.copy(&texture, sprite, sprite_build).unwrap();
                    } else if graphical_space.tile_name == *tile_name && player_map_space.is_visible
                    {
                        if col == 0 {
                            canvas.copy(&texture, sprite, sprite_build).unwrap();
                        } else {
                            canvas.copy(&texture, sprite, sprite_build).unwrap();
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) fn render_objects(
        canvas: &mut WindowCanvas,
        map_data: &MapData,
        objects_to_render: &Vec<&str>,
        _grid_x: i32,
        _grid_y: i32,
        tile_width: i32,
        tile_height: i32,
        camera_x: i32,
        camera_y: i32,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .load_texture("assets/dimension.png")
            .unwrap();

        let (screen_width, screen_height) = canvas.output_size()?;
        let map_width = map_data.width as i32;
        let map_height = map_data.height as i32;

        for col in 0..map_height {
            for row in 0..map_width {
                let space = &map_data.map[col as usize][row as usize];

                for object_to_render in objects_to_render.as_slice() {
                    if space.tile_name == *object_to_render && space.is_visible {
                        let sprite = Rect::new(
                            space.tile_sprite_position.x,
                            space.tile_sprite_position.y,
                            space.tile_width,
                            space.tile_height,
                        );

                        let centered_row = (row * tile_width) + (screen_width as i32 / 2)
                            - ((map_width * tile_width) / 2)
                            - camera_x;
                        let centered_col = (col * tile_height) + (screen_height as i32 / 2)
                            - ((map_height * tile_height) / 2)
                            - camera_y;

                        let sprite_build = Rect::new(
                            centered_row,
                            centered_col,
                            tile_width as u32,
                            tile_height as u32,
                        );
                        canvas.copy(&texture, sprite, sprite_build).unwrap();
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) fn render_monsters(
        canvas: &mut WindowCanvas,
        map_data: &MapData,
        monsters: &mut Monsters,
        monsters_to_render: &Vec<&str>,
        _grid_x: i32,
        _grid_y: i32,
        tile_width: i32,
        tile_height: i32,
        camera_x: i32,
        camera_y: i32,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .load_texture("assets/dimension.png")
            .unwrap();

        let (screen_width, screen_height) = canvas.output_size()?;
        let map_width = map_data.width as i32;
        let map_height = map_data.height as i32;

        for monster in monsters.values_mut() {
            for col in 0..map_height {
                for row in 0..map_width {
                    let space = &map_data.map[col as usize][row as usize];
                    let space_position = Vec2::new(row as usize, col as usize);

                    for monster_to_render in monsters_to_render.as_slice() {
                        if space.tile_name == *monster_to_render
                            && space_position == monster.position
                            && space.is_visible
                        {
                            // MONSTER SPRITE
                            let sprite = Rect::new(
                                space.tile_sprite_position.x,
                                space.tile_sprite_position.y,
                                space.tile_width,
                                space.tile_height,
                            );

                            let centered_row = (row * tile_width) + (screen_width as i32 / 2)
                                - ((map_width * tile_width) / 2)
                                - camera_x;
                            let centered_col = (col * tile_height) + (screen_height as i32 / 2)
                                - ((map_height * tile_height) / 2)
                                - camera_y;

                            let sprite_build = Rect::new(
                                centered_row,
                                centered_col,
                                tile_width as u32,
                                tile_height as u32,
                            );
                            canvas.copy(&texture, sprite, sprite_build).unwrap();
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) fn render_monsters_status(
        canvas: &mut WindowCanvas,
        map_data: &MapData,
        monsters: &mut Monsters,
        monsters_to_render: &Vec<&str>,
        _grid_x: i32,
        _grid_y: i32,
        tile_width: i32,
        tile_height: i32,
        camera_x: i32,
        camera_y: i32,
    ) -> Result<(), String> {
        let (screen_width, screen_height) = canvas.output_size()?;
        let map_width = map_data.width as i32;
        let map_height = map_data.height as i32;

        let texture_creator = canvas.texture_creator();
        let ttf_context = sdl2::ttf::init().unwrap();

        for monster in monsters.values_mut() {
            for col in 0..map_height {
                for row in 0..map_width {
                    let space = &map_data.map[col as usize][row as usize];
                    let space_position = Vec2::new(row as usize, col as usize);

                    for monster_to_render in monsters_to_render.as_slice() {
                        if space.tile_name == *monster_to_render
                            && space_position == monster.position
                            && space.is_visible
                        {
                            // MONSTER HEALTH
                            let centered_row = (row * tile_width) + (screen_width as i32 / 2)
                                - ((map_width * tile_width) / 2)
                                - camera_x;
                            let centered_col = (col * tile_height) + (screen_height as i32 / 2)
                                - ((map_height * tile_height) / 2)
                                - camera_y;

                            // calculate the position for drawing the health bar above the monster
                            let health_bar_x = centered_row;
                            let health_bar_y = centered_col - 10; // adjust the position as needed

                            // Calculate the width of the health bar based on the monster's health
                            let max_health = monster.status.max_health as f32;
                            let current_health = monster.status.current_health as f32;
                            let health_percentage = max_health / current_health;
                            let health_bar_width = (tile_width as f32 * health_percentage) as u32;

                            // define the health bar's dimensions
                            let health_bar_height = 7; // adjust the height as needed

                            let mut hp_bar_colour = COLOUR.white;

                            if health_percentage > 0.7 {
                                hp_bar_colour = COLOUR.green;
                            } else if health_percentage > 0.4 {
                                hp_bar_colour = COLOUR.yellow;
                            } else {
                                hp_bar_colour = COLOUR.red;
                            }

                            // draw the filled health bar rectangle with the specified color
                            let health_bar_rect = Rect::new(
                                health_bar_x,
                                health_bar_y,
                                health_bar_width,
                                health_bar_height,
                            );
                            canvas.set_draw_color(hp_bar_colour);
                            canvas.fill_rect(health_bar_rect).expect("colour fill");
                            canvas.draw_rect(health_bar_rect).expect("hp bar");

                            // render the monster's name above the health bar
                            let font_path = "assets/spindles_end.ttf";
                            let font_size = 18;

                            let font = ttf_context.load_font(font_path, font_size)?;
                            let text_surface =
                                font.render(&monster.name).blended(COLOUR.white).unwrap();
                            let text_texture = texture_creator
                                .create_texture_from_surface(&text_surface)
                                .unwrap();
                            let text_width = text_surface.width() as i32;
                            let text_height = text_surface.height() as i32;
                            let text_position = Rect::new(
                                health_bar_x + (health_bar_width as i32 / 2) - (text_width / 2),
                                health_bar_y - text_height,
                                text_width as u32,
                                text_height as u32,
                            );
                            canvas.copy(&text_texture, None, text_position).unwrap();
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn direction_spritesheet_row(direction: Direction) -> i32 {
        use self::Direction::*;
        match direction {
            Up => 3,
            Down => 0,
            Left => 1,
            Right => 2,
        }
    }
}
