use crate::player::Player;
use crate::vec2::Vec2;

pub(crate) struct Camera<'a> {
    pub(crate) x: i32,
    pub(crate) y: i32,
    tracked_object: Option<&'a Player>,
}

impl<'a> Camera<'a> {
    pub(crate) fn new(player: &'a Player) -> Self {
        Camera {
            x: 0,
            y: 0,
            tracked_object: Some(player),
        }
    }

    pub(crate) fn update_camera_position(
        &mut self,
        new_player_pos: Vec2,
        screen_width: u32,
        screen_height: u32,
        zoom_factor: f32
    ) {
        if let Some(object) = self.tracked_object {
            self.x = object.sprite_position.x + (new_player_pos.x * new_player_pos.x * 2) as i32
                - screen_width as i32 / 2;
            self.y = object.sprite_position.y + 240 + (new_player_pos.y * new_player_pos.y * 2) as i32
                - screen_height as i32 / 2;
        }

        /*if let Some(object) = self.tracked_object {
            // Calculate the scaled screen width and height
            let scaled_screen_width = (screen_width as f32 / zoom_factor) as i32;
            let scaled_screen_height = (screen_height as f32 / zoom_factor) as i32;

            // Calculate the camera position based on the player's position and zoom factor
            self.x = (object.sprite_position.x as f32 * zoom_factor) as i32 - scaled_screen_width / 2
                + (new_player_pos.x as f32 * zoom_factor) as i32;
            self.y = (object.sprite_position.y as f32 * zoom_factor) as i32 - scaled_screen_height / 2
                + (new_player_pos.y as f32 * zoom_factor) as i32;
        }*/

    }
}
