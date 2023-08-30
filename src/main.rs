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
use crossterm::event::{Event, KeyEventKind};
use futures::lock::{Mutex, MutexGuard};
use std::sync::Arc;
use std::time::Duration;
use vec2::Vec2;

use crate::chat::Chat;
use crate::collision_engine::CollisionEngine;
use crate::map_data::MapData;
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
    let map_manager = MapManager::new();
    let mut player = Player::new();
    //let mut collision_engine = CollisionEngine::new();
    let mut chat = Chat::new();
    let mut map_factory = MapFactory::new();
    let monster_manager = MonsterManager::new();
    let mut monster_factory = MonsterFactory::new();
    let terminal = GameClient::new();

    /*;
    let mut map_manager_guard = map_manager.lock().await;*/
    // Acquire the initial map outside of the loop
    let map_manager = Arc::new(Mutex::new(map_manager));
    let mut map_manager_guard = map_manager.lock().await;

    //map_manager_guard.add_map_set_player_position(&mut player, "scene_ladder", Vec2::new(3, 2));
    map_manager_guard.add_map_set_player_position(&mut player, "map2", Vec2::new(6, 2));

    //map_manager_guard.add_map_set_player_position(&mut player, "map1", Vec2::new(5, 2));
    //let new_map = map_factory.generate_map(&mut player, 10, 10, Vec2::new(2, 1), "seedphrase");
    //map_manager_guard.add_generated_map(new_map);

    /*let map_mut = Arc::new(Mutex::new(
        map_manager.get_map_mut(map_manager.current_map_index),
    ));*/

    /*let map_index = map_manager_guard.current_map_index;
    let map_mut = Arc::new(Mutex::new(
        map_manager_guard
            .get_map_mut(map_index)
            .expect("Invalid map index"),
    ));*/
    let collision_engine = Arc::new(Mutex::new(CollisionEngine::new()));
    let monster_manager = Arc::new(Mutex::new(monster_manager));
    let mut monster_manager_guard = monster_manager.lock().await;
    let mut collision_engine_guard = collision_engine.lock().await;

    map_manager_guard.load_map("map2", MovementType::Normal);

    let map_index = map_manager_guard.current_map_index;
    let mut map_mut = map_manager_guard
        .get_map_mut(map_index)
        .expect("Invalid map index");

    monster_manager_guard.spawn_monsters(&mut chat, &mut map_mut, &mut monster_factory);
    collision_engine_guard.update_player_vision(&mut map_mut, &mut player, Vec2::ZERO);

    terminal.print_terminal(&mut player, &mut map_mut, &mut chat);

    //let map = map_manager_guard.get_map(map_index);

    drop(monster_manager_guard);
    drop(collision_engine_guard);

    /*tokio::spawn({
        let collision_engine = Arc::clone(&collision_engine);
        let map_mut = Arc::clone(&map_mut);
        let monster_manager = Arc::clone(&monster_manager);

        async move {
            let mut collision_engine = collision_engine.lock().await;
            let mut map_mut = map_mut.lock().await;
            let mut monster_manager = monster_manager.lock().await;
            update_monsters_async(&mut collision_engine, &mut map_mut, &mut monster_manager).await
        }
    });*/

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    player.key_event = key_input.code;

                    let mut collision_engine_guard = collision_engine.lock().await;

                    let new_player_pos = collision_engine_guard.move_player(&mut player, &mut chat);

                    let player_move_type = collision_engine_guard.process_move(
                        map_mut,
                        &mut player,
                        &mut chat,
                        new_player_pos,
                    );

                    match player_move_type {
                        MovementType::Normal => {
                            collision_engine_guard.update_player_position(
                                &mut map_mut,
                                &mut player,
                                new_player_pos,
                            );
                        }
                        MovementType::LadderUp => {
                            let mut map_manager_guard = map_manager.lock().await;
                            let m_data = map_manager_guard
                                .load_map("scene_ladder", MovementType::LadderDown)
                                .cloned();
                            if let Some(map) = m_data {
                                *map_mut = map;
                            }
                            drop(map_manager_guard);
                        }
                        /* MovementType::LadderDown => {
                            map_mut_guard = map_manager_guard
                                .load_map("scene_ladder", MovementType::LadderDown);
                        }
                        MovementType::LadderExit => {
                            map_mut_guard = map_manager_guard
                                .load_map("map2", MovementType::Normal);
                        }
                        MovementType::LadderEnter => {
                            map_mut_guard = map_manager_guard
                                .load_map("map1", MovementType::Normal);
                        }*/
                        _ => {}
                    }

                    let terrain_data =
                        map_factory.generate_terrain(&mut map_mut, new_player_pos, &mut chat);

                    if let Some(terrain_data) = terrain_data {
                        map_mut.map = terrain_data.map;
                        map_mut.map_height += terrain_data.height_increase;
                        map_mut.map_width += terrain_data.width_increase;
                    }

                    collision_engine_guard.update_player_vision(
                        &mut map_mut,
                        &mut player,
                        new_player_pos,
                    );
                    terminal.print_terminal(&mut player, &mut map_mut, &mut chat);

                    drop(collision_engine_guard);

                    if player.key_state {
                        break;
                    }
                }
            }
            _ => {}
        }
    }
}

// asynchronous function to update monsters
async fn update_monsters_async<'a>(
    collision_engine: &mut CollisionEngine,
    map: &'a mut MutexGuard<'a, &mut MapData>,
    player: &mut Player,
    monster_manager: &mut MonsterManager,
) {
    loop {
        let mut new_monsters_pos = collision_engine.move_monsters(player, monster_manager);

        let processed_monsters_positions =
            collision_engine.process_monsters_move(&mut new_monsters_pos, map, monster_manager);

        collision_engine.update_monsters_position(
            map,
            monster_manager,
            processed_monsters_positions,
        );

        // sleep for 1 second before the next update
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}
