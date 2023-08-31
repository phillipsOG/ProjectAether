use std::io;
use crate::chat::Chat;

use crate::player::Player;
use crate::tile_set::DEFAULT_TILE_SET;

use crossterm::{terminal, QueueableCommand, ExecutableCommand};

use crate::map_manager::MapManager;
use futures::lock::{Mutex, MutexGuard};
use std::io::{stdout, Write};
use std::sync::Arc;
use crossterm::cursor::DisableBlinking;

#[derive(Clone)]
pub struct GameClient { }

impl GameClient {
    pub(crate) fn new() -> Self { GameClient { } }

    pub(crate) async fn print_terminal<'a>(
        &self,
        player: &Player,
        map_manager_clone: &mut MutexGuard<'a, MapManager>,
        chat: &mut Arc<Mutex<Chat>>,
    ) {
        let mut stdout = stdout();
        stdout.queue(crossterm::cursor::MoveTo(0, 0)).unwrap();
        stdout.flush().unwrap();
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();


        let mut str_map = String::new();

        io::stdout().execute(DisableBlinking).expect("disable blink?");

        let mut tmp_plr = player.clone();
        let mut tmp_chat = chat.lock().await;
        let map_guard = map_manager_clone
            .get_map(map_manager_clone.current_map_index)
            .expect("map data");
        let modules = [
            tmp_plr.status.get_status(),
            tmp_plr
                .inventory
                .get_inventory_to_size(2, format!("FLOOR: {}", player.current_floor)),
        ];

        let mut counter = 0;
        for tile in &map_guard.map {
            let tile_line: String = tile
                .iter()
                .map(|space| {
                    if space.is_visible || space.tile == DEFAULT_TILE_SET.player {
                        space.tile
                    } else {
                        ' ' //show no map tile at iteration if not visible or player
                    }
                })
                .collect();

            if counter <= modules.len() {
                str_map += &*format!(
                    "{}      {}      {}\n",
                    tile_line, modules[0][counter], modules[1][counter]
                );
                counter += 1;
            } else {
                //println!("{}", tile_line);
                str_map += &*format!("{}\n", tile_line);
            }
        }

        println!("{}", str_map);
        tmp_chat.print_chat();
        drop(tmp_chat);
        //drop(map_manager_guard);
    }
}
