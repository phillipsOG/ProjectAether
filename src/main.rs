mod player;
mod chat;
mod map_data;
mod inventory;
mod status;
mod collision;
mod tile_set;
mod map_manager;
mod game_client;
mod player_movement_data;
mod space;
mod map_factory;

use crossterm::{event};
use crossterm::event::{Event, KeyEventKind};
use crate::game_client::GameClient;
use crate::space::Space;

pub struct TerrainData {
    map: Map,
    height_increase: usize,
    width_increase: usize
}

impl TerrainData {
    pub(crate) fn new() -> Self {
        TerrainData {
            map: vec![vec![]],
            height_increase: 0,
            width_increase: 0,
        }
    }
}

type Map = Vec<Vec<Space>>;

enum PlayerMove {
    Unable,
    Normal,
    LadderUp,
    LadderDown,
    LadderEnter,
    LadderExit
}

fn main() {
    let mut game_client = GameClient::new();

    game_client.map_manager.add_map_set_player_position("scene_ladder", 2, 3);
    game_client.map_manager.add_map_set_player_position("map2", 2, 6);
    game_client.map_manager.add_map_set_player_position("map1", 2, 5);
    let new_map = game_client.map_factory.generate_map(7, 8, 1, 2);
    game_client.map_manager.add_generated_map(new_map);
    game_client.map_manager.load_map("map2", PlayerMove::Normal);

   game_client.print_terminal();

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    game_client.player.key_event = key_input.code;

                    let new_player_pos = game_client.collision_engine.move_player(&mut game_client.map_manager, &mut game_client.player, &mut game_client.chat);
                    let player_move_type = game_client.collision_engine.process_move(&mut game_client.player, &mut game_client.map_manager, &mut game_client.chat, new_player_pos);

                    match player_move_type {
                        PlayerMove::Normal => {
                            game_client.collision_engine.update_player_position(&mut game_client.map_manager, new_player_pos);
                        },
                        PlayerMove::LadderUp => {
                            game_client.map_manager.load_map("scene_ladder", PlayerMove::LadderUp);
                        },
                        PlayerMove::LadderDown => {
                            game_client.map_manager.load_map("scene_ladder", PlayerMove::LadderDown);
                        },
                        PlayerMove::LadderExit => {
                            game_client.map_manager.load_map("map2", PlayerMove::Normal);
                        },
                        PlayerMove::LadderEnter => {
                            game_client.map_manager.load_map("map1", PlayerMove::Normal);
                        },
                        _ => {}
                    }

                    //let terrain_data = game_client.map_factory.generate_terrain(&mut game_client.map_manager, new_player_pos, &mut game_client.chat);
                    //game_client.map_manager.update_current_map(terrain_data, &mut game_client.chat);
                    game_client.collision_engine.update_player_vision(&mut game_client.map_manager, new_player_pos);

                    game_client.print_terminal();/*.print_terminal_with_map(&mut updated_map);*/

                    if game_client.player.key_state {
                        break;
                    }
                }
            }
            _ => {}
        }
    }
}