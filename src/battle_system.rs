use crate::chat::Chat;
use crate::monster::Monster;
use crate::player::Player;
use futures::lock::{Mutex, MutexGuard};
use std::sync::Arc;

pub(crate) struct BattleSystem {}

impl BattleSystem {
    pub(crate) async fn start_battle<'a>(
        // map: &MapData,
        player: &mut MutexGuard<'a, Player>,
        monster: &mut Monster,
        chat: &Arc<Mutex<Chat>>,
    ) {
        //monster.in_battle = true;
        monster.status.max_health -= player.status.str;

        chat.lock().await.process_debug_message(
            &format!("monster has: {} hp left", monster.status.max_health),
            1,
        );

        if monster.status.max_health.is_negative() {
            chat.lock()
                .await
                .process_debug_message("monster has no hp left", 1);
            monster.is_alive = false;
        }
    }
}
