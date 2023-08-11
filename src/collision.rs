use crossterm::event::KeyCode;
use crate::player::Player;

pub struct CollisionEngine {

}

impl CollisionEngine {
    pub(crate) fn new() -> Self {
        CollisionEngine {

        }
    }

    pub(crate) fn process_input(&mut self, mut player: Player) {
        match player.key_event {
            KeyCode::Left => {
                player.move_player();
            }
            KeyCode::Right => {
                player.move_player();
            }
            KeyCode::Up => {
                player.move_player();
            }
            KeyCode::Down => {
                player.move_player();
            }
            KeyCode::Esc => {
                player.chat.process_chat_message("Pressed ESC & Exited the Game");
                player.previous_key_event = KeyCode::Esc;
                player.key_state = true;
            }
            _ => {}
        }
    }

    pub(crate) fn update_player_position(&mut self, mut player: Player) {
        let at_position: Option<(usize, usize)> = None;

        for (row_idx, row) in player.map.map.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if c == '@' {
                    player.player_position = Option::from((row_idx, col_idx));
                    break;
                }
            }
            if at_position.is_some() {
                break;
            }
        }
    }
}