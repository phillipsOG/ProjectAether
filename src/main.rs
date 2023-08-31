mod chat;
mod collision_engine;
mod game_client;
mod inventory;
mod map_data;
mod map_factory;
mod map_manager;
mod monster;
mod monster_generator;
mod monster_manager;
mod player;
mod player_movement_data;
mod space;
mod status;
mod terrain_data;
mod tile_set;
mod vec2;

type Map = Vec<Vec<Space>>;


use crate::game_client::GameClient;
use crate::space::Space;

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use futures::lock::Mutex;
use std::sync::Arc;
use std::time::Duration;
use vec2::Vec2;

use crate::chat::Chat;
use crate::collision_engine::CollisionEngine;

use crate::map_factory::MapFactory;
use crate::map_manager::MapManager;
use crate::monster_generator::MonsterFactory;
use crate::monster_manager::MonsterManager;
use crate::player::Player;

enum MovementType {
    Unable,
    Normal,
    LadderUp,
    LadderDown,
    LadderEnter,
    LadderExit,
}

#[tokio::main]
async fn main() {
    // Initialize your game components
    let player = Arc::new(Mutex::new(Player::new())); //Player::new();
    let player_clone = Arc::clone(&player);
    let mut player_guard = player_clone.lock().await;

    let _map_factory = MapFactory::new();
    let monster_factory = MonsterFactory::new();

    let terminal = Arc::new(Mutex::new(GameClient::new()));
    let terminal_clone = Arc::clone(&terminal);
    let terminal_guard = terminal_clone.lock().await;

    //let player = Arc::new(Mutex::new(player));
    //let player_clone = Arc::clone(&player);
    //let mut player_guard = player_clone.lock().await;
    //map_manager_guard.add_map_set_player_position(&mut player, "scene_ladder", Vec2::new(3, 2));
    let map_manager = Arc::new(Mutex::new(MapManager::new())); //MapManager::new();
    let map_manager_clone = Arc::clone(&map_manager);
    let mut map_manager_guard = map_manager_clone.lock().await;

    map_manager_guard.add_map_set_player_position(&mut player_guard, "map2", Vec2::new(6, 2));

    //map_manager_guard.add_map_set_player_position(&mut player, "map1", Vec2::new(5, 2));
    //let new_map = map_factory.generate_map(&mut player, 10, 10, Vec2::new(2, 1), "seedphrase");
    //map_manager_guard.add_generated_map(new_map);

    map_manager_guard.load_map("map2", MovementType::Normal);

    let monster_manager = Arc::new(Mutex::new(MonsterManager::new()));
    let monster_manager_clone = Arc::clone(&monster_manager);
    let mut monster_manager_guard = monster_manager_clone.lock().await;

    let collision_engine = Arc::new(Mutex::new(CollisionEngine::new())); //CollisionEngine::new();
    let collision_engine_clone = Arc::clone(&collision_engine);
    let mut collision_engine_guard = collision_engine_clone.lock().await;

    let chat = Arc::new(Mutex::new(Chat::new()));
    let mut chat_clone = Arc::clone(&chat);
    //let mut chat_guard = chat_clone.lock().await;

    monster_manager_guard.spawn_monsters(&mut map_manager_guard, monster_factory);
    collision_engine_guard
        .update_player_vision(&mut map_manager_guard, &player_guard, Vec2::ZERO)
        .await;

    terminal_guard
        .print_terminal(&player_guard, &mut map_manager_guard, &mut chat_clone)
        .await;

    /*tokio::spawn({
        async move {
            // for testing
            let chat_clone = Arc::clone(&chat);

            let collision_engine_clone = Arc::clone(&collision_engine);
            let monster_manager_clone = Arc::clone(&monster_manager);
            let player_clone = Arc::clone(&player);
            let mut map_manager_guard = Arc::clone(&map_manager);

            loop {
                update_monsters_async(
                    collision_engine_clone,
                    &mut map_manager_guard,
                    player_clone,
                    monster_manager_clone,
                    chat_clone,
                ).await;
                async_std::task::sleep(Duration::from_secs(1)).await;
            }
        }
    });*/
    tokio::spawn({
        async move {
            let mut chat_clone = Arc::clone(&chat);
            let mut terminal_clone = Arc::clone(&terminal);

            loop {
                update_monsters_async_test(&chat_clone, &terminal_clone).await;
                async_std::task::sleep(Duration::from_secs(1)).await;
            }
        }
    });

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {

                    match player_guard.key_event {
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                    player_guard.key_event = key_input.code;

                    let new_player_pos = collision_engine_guard
                        .move_player(&mut player_guard, &mut chat_clone)
                        .await;

                    let player_move_type = collision_engine_guard
                        .process_move(
                            &mut map_manager_guard,
                            &mut player_guard,
                            &mut chat_clone,
                            new_player_pos,
                        )
                        .await;

                    match player_move_type {
                        MovementType::Normal => {
                            collision_engine_guard.update_player_position(
                                &mut map_manager_guard,
                                &mut player_guard,
                                new_player_pos,
                            ).await;
                        }
                        _ => {}
                    }

                    collision_engine_guard
                        .update_player_vision(
                            &mut map_manager_guard,
                            &mut player_guard,
                            new_player_pos,
                        )
                        .await;

                    terminal_guard
                        .print_terminal(&player_guard, &mut map_manager_guard, &mut chat_clone)
                        .await;
                }
            }
            _ => {}
        }
    }
}

async fn update_monsters_async_test(chat: &Arc<Mutex<Chat>>, terminal: &Arc<Mutex<GameClient>>) {
    let mut chat_guard = chat.lock().await;
    chat_guard.process_chat_message("test");
        //drop(chat_guard);
    let mut terminal_guard = terminal.lock().await;

    //drop(terminal_guard);
}


// asynchronous function to update monsters
async fn update_monsters_async(
    _collision_engine: Arc<Mutex<CollisionEngine>>,
    _map_manager_clone: &mut Arc<Mutex<MapManager>>,
    _player: Arc<Mutex<Player>>,
    _monster_manager: Arc<Mutex<MonsterManager>>,
    chat: Arc<Mutex<Chat>>,
) {
    //let mut chat_guard = chat.lock().await;
    //chat_guard.process_chat_message("inside of monster state async tick update");
    //drop(chat_guard);
    /*let mut collision_engine_guard = _collision_engine.lock().await;
    drop(collision_engine_guard);*/
    /*let mut monster_manager_guard = monster_manager.lock().await;
    let mut player_guard = player.lock().await;
    let mut map_manager_guard = map_manager_clone.lock().await;*/
    /*

    let mut new_monsters_pos =
    collision_engine_guard.move_monsters(player_guard, &mut monster_manager_guard).await;

    let processed_monsters_positions = collision_engine_guard
        .process_monsters_move(
            &mut new_monsters_pos,
            &mut map_manager_guard,
            &mut monster_manager_guard,
        )
        .await;

    collision_engine_guard
        .update_monsters_position(
            &mut map_manager_guard,
            &mut monster_manager_guard,
            processed_monsters_positions,
        )
        .await;

    drop(collision_engine_guard);
    drop(monster_manager_guard);
    drop(map_manager_guard);*/


}
