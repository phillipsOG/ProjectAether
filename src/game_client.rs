use crate::chat::Chat;

use crate::map_data::MapData;

use crate::player::Player;
use crate::tile_set::DEFAULT_TILE_SET;

use crossterm::{terminal, QueueableCommand};

use futures::lock::Mutex;
use std::io::stdout;
use std::sync::Arc;

#[derive(Clone)]
pub struct GameClient {
    /*pub map_manager: MapManager,
    pub player: Player,
    pub collision_engine: CollisionEngine,
    pub chat: Chat,
    pub map_factory: MapFactory,
    pub monster_manager: MonsterManager,
    pub monster_factory: MonsterFactory,*/
}

impl GameClient {
    pub(crate) fn new() -> Self {
        GameClient {
            /*map_manager: MapManager::new(),
            player: Player::new(),
            collision_engine: CollisionEngine::new(),
            chat: Chat::new(),
            map_factory: MapFactory {},
            monster_manager: MonsterManager::new(),
            monster_factory: MonsterFactory::new(),*/
        }
    }

    pub(crate) async fn print_terminal(
        &self,
        player: &Player,
        map_data: &MapData,
        chat: &mut Arc<Mutex<Chat>>,
    ) {
        let mut stdout = stdout();
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        let mut str_map = String::new();

        let mut tmp_plr = player.clone();
        let mut tmp_chat = chat.lock().await;

        let modules = [
            tmp_plr.status.get_status(),
            tmp_plr
                .inventory
                .get_inventory_to_size(2, format!("FLOOR: {}", player.current_floor)),
        ];
        let mut counter = 0;
        for tile in &map_data.map {
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
    }
}
