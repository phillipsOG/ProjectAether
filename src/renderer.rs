use futures::lock::MutexGuard;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use crate::Direction;
use crate::map_data::MapData;
use crate::player::Player;

pub(crate) struct Renderer {

}

impl Renderer {


    pub(crate) fn render_player(
        canvas: &mut WindowCanvas,
        colour: Color,
        texture: &Texture,
        player: &MutexGuard<Player>
    ) -> Result<(), String> {

        let (width, height) = canvas.output_size()?;
        let (frame_width, frame_height) = player.sprite.size();
        let current_frame = Rect::new(
            player.sprite.x() + frame_width as i32 * player.current_frame,
            player.sprite.y() + frame_height as i32 * Renderer::direction_spritesheet_row(player.direction),
            frame_width,
            frame_height
        );

        let screen_position = player.sprite_position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);

        canvas.copy(texture, current_frame, screen_rect)?;

        Ok(())
    }

    pub(crate) fn render_map(
        canvas: &mut WindowCanvas,
        colour: Color,
        texture: &Texture,
        player: &MutexGuard<Player>
    ) -> Result<(), String> {

        let (width, height) = canvas.output_size()?;
        let (frame_width, frame_height) = player.sprite.size();
        let current_frame = Rect::new(
            player.sprite.x() + frame_width as i32 * player.current_frame,
            player.sprite.y() + frame_height as i32 * Renderer::direction_spritesheet_row(player.direction),
            frame_width,
            frame_height
        );

        let screen_position = player.sprite_position + Point::new(width as i32 / 3, height as i32 / 3);
        let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);

        canvas.copy(texture, current_frame, screen_rect)?;

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