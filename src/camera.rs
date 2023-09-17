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

    pub(crate) fn update_camera_position_old(&mut self) {
        if let Some(object) = self.tracked_object {
            self.x = object.sprite_position.x - 800/2;
            self.y = object.sprite_position.y - 600/2;
        }
    }

    pub(crate) fn update_camera_position(&mut self, new_player_pos: Vec2, screen_width: u32, screen_height: u32) {
        if let Some(object) = self.tracked_object {
            self.x = object.sprite_position.x +(new_player_pos.x*new_player_pos.x*2) as i32  - screen_width as i32 /2;
            self.y = object.sprite_position.y +(new_player_pos.y*new_player_pos.y*2) as i32  - screen_height as i32 /2;
        }
    }
}
