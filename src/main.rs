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
mod pathfinding;
mod player;
mod player_movement_data;
mod space;
mod status;
mod terrain_data;
mod tile_set;
mod vec2;
mod battle_system;
mod state;

type Map = Vec<Vec<Space>>;

use crate::game_client::GameClient;
use crate::space::Space;
use std::io::stdout;
use crossterm::event::{KeyCode, KeyEventKind};
use crossterm::{event, terminal, QueueableCommand};
use futures::lock::Mutex;
use futures::TryFutureExt;
use std::sync::Arc;
use vec2::Vec2;

use crate::battle_system::BattleSystem;
use crate::chat::Chat;
use crate::collision_engine::CollisionEngine;
use crate::map_factory::MapFactory;
use crate::map_manager::MapManager;
use crate::monster_generator::MonsterFactory;
use crate::monster_manager::MonsterManager;
use crate::player::Player;
use crate::tile_set::{DEFAULT_TILE_SET, LADDER_TILE_SET};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::state::State;

enum MovementType {
    Unable,
    Normal,
    LadderUp,
    LadderDown,
    LadderEnter,
    LadderExit,
    Battle,
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {

            Event::WindowEvent {
                ref event,
                window_id,
            }
            if window_id == state.window().id() => if !state.input(event) { // UPDATED!
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.render().expect("TODO: panic message");
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(run());
}

/*fn main() {
    env_logger::init(); // Necessary for logging within WGPU
    let event_loop = EventLoop::new(); // Loop provided by winit for handling window events
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Opens the window and starts processing events (although no events are handled yet)
    event_loop.run(move |event, _, control_flow| {});
}*/

/*
#[tokio::main]
async fn main() {
    let mut player = Arc::new(Mutex::new(Player::new()));
    let player_clone = Arc::clone(&player);
    let mut player_guard = player_clone.lock().await;

    let mut map_factory = MapFactory::new();
    let monster_factory = MonsterFactory::new();

    let terminal = Arc::new(Mutex::new(GameClient::new()));
    let terminal_clone = Arc::clone(&terminal);
    let mut terminal_guard = terminal_clone.lock().await;

    let monster_manager = Arc::new(Mutex::new(MonsterManager::new()));
    let monster_manager_clone = Arc::clone(&monster_manager);
    let mut monster_manager_guard = monster_manager_clone.lock().await;

    let mut map_manager = Arc::new(Mutex::new(MapManager::new()));
    let map_manager_clone = Arc::clone(&map_manager);
    let mut map_manager_guard = map_manager_clone.lock().await;

    map_manager_guard.add_map_set_player_position(
        &mut player_guard,
        "scene_ladder",
        Vec2::new(3, 2),
    );
    map_manager_guard.add_map_set_player_position(&mut player_guard, "map1", Vec2::new(5, 2));
    let new_map =
        map_factory.generate_map(&mut player_guard, 20, 20, Vec2::new(2, 1), "seedphrase");
    map_manager_guard.add_generated_map(new_map);
    map_manager_guard.add_map_set_player_position(&mut player_guard, "test_map", Vec2::new(10, 10));
    //map_manager_guard.add_map_set_player_position(&mut player_guard, "map3", Vec2::new(4, 5));
    map_manager_guard.add_map_set_player_position(&mut player_guard, "map2", Vec2::new(6, 2));
    map_manager_guard.load_map("map2", MovementType::Normal);

    let collision_engine = Arc::new(Mutex::new(CollisionEngine::new()));
    let collision_engine_clone = Arc::clone(&collision_engine);
    let mut collision_engine_guard = collision_engine_clone.lock().await;

    let chat = Arc::new(Mutex::new(Chat::new()));
    let mut chat_clone = Arc::clone(&chat);

    monster_manager_guard
        .spawn_monsters(&mut map_manager_guard, monster_factory);

    collision_engine_guard
        .update_player_vision(&mut map_manager_guard, &mut player_guard, Vec2::ZERO)
        .await;

    let mut stdout = stdout();
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    terminal_guard
        .print_terminal(&player_guard, &mut map_manager_guard, &mut chat_clone)
        .await;

    drop(terminal_guard);
    drop(map_manager_guard);
    drop(collision_engine_guard);
    drop(player_guard);
    drop(monster_manager_guard);

    tokio::spawn({
        async move {
            let mut chat_clone = Arc::clone(&chat);
            let collision_engine_clone = Arc::clone(&collision_engine);
            let monster_manager_clone = Arc::clone(&monster_manager);
            let player_clone = Arc::clone(&player);
            let mut map_manager_guard = Arc::clone(&map_manager);
            let terminal_clone = Arc::clone(&terminal);

            update_monsters_async(
                collision_engine_clone,
                &mut map_manager_guard,
                player_clone,
                monster_manager_clone,
                &mut chat_clone,
                terminal_clone,
            )
            .await;
        }
    });

    loop {
        match event::read().unwrap() {
            Event::Key(key_input) => {
                if key_input.kind == KeyEventKind::Press {
                    let mut player_guard = player_clone.lock().await;

                    match player_guard.key_event {
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }

                    player_guard.key_event = key_input.code;
                    let mut terminal_guard = terminal_clone.lock().await;
                    let mut map_manager_guard = map_manager_clone.lock().await;
                    let mut collision_engine_guard = collision_engine_clone.lock().await;
                    let mut monster_manager_guard = monster_manager_clone.lock().await;

                    //let mut chat_guard = chat_clone.lock().await;

                    let new_player_pos = collision_engine_guard
                        .try_move_player(&mut player_guard, &mut chat_clone)
                        .await;

                    let player_move_type = collision_engine_guard
                        .try_process_move(
                            &mut map_manager_guard,
                            &mut player_guard,
                            &mut chat_clone,
                            new_player_pos,
                        )
                        .await;
                    player_guard.previous_tile_below_player = player_guard.tile_below_player;

                    match player_move_type {
                        MovementType::Normal => {
                            collision_engine_guard
                                .update_player_position(
                                    &mut map_manager_guard,
                                    &mut player_guard,
                                    new_player_pos,
                                )
                                .await;
                        }
                        MovementType::LadderUp => {
                            map_manager_guard.load_map("scene_ladder", MovementType::LadderUp);
                            let ladder_entry_pos = Vec2::new(3, 2);
                            player_guard.update_tile_below_player(LADDER_TILE_SET.floor);
                            player_guard.previous_player_position = player_guard.position;
                            player_guard.position = ladder_entry_pos;
                        }
                        MovementType::LadderDown => {
                            map_manager_guard.load_map("scene_ladder", MovementType::LadderDown);
                            player_guard.position = Vec2::new(3, 2);
                        }
                        MovementType::LadderExit => {
                            map_manager_guard.load_map("map2", MovementType::Normal);
                            player_guard.update_tile_below_player(LADDER_TILE_SET.floor);
                            player_guard.position = player_guard.previous_player_position;
                            player_guard.tile_below_player =
                                player_guard.previous_tile_below_player;
                        }
                        MovementType::LadderEnter => {
                            map_manager_guard.load_map("map1", MovementType::Normal);
                        }
                        MovementType::Battle => {
                            let position = Vec2::new(new_player_pos.x, new_player_pos.y);
                            
                            if let Some(monster) = monster_manager_guard.get_monster_at_position(position) {
                                BattleSystem::start_battle(&mut player_guard, monster, &chat_clone).await;
                            }

                            drop(monster_manager_guard);
                        }

                        _ => {}
                    }

                    let terrain_data = map_factory
                        .generate_terrain(&mut map_manager_guard, new_player_pos, &mut chat_clone)
                        .await;

                    if let Some(terrain_data) = terrain_data {
                        map_manager_guard.update_current_map(terrain_data, &mut chat_clone);
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

// asynchronous function to update monsters
async fn update_monsters_async(
    collision_engine_clone: Arc<Mutex<CollisionEngine>>,
    map_manager_clone: &mut Arc<Mutex<MapManager>>,
    player_clone: Arc<Mutex<Player>>,
    monster_manager_clone: Arc<Mutex<MonsterManager>>,
    chat_clone: &mut Arc<Mutex<Chat>>,
    terminal_clone: Arc<Mutex<GameClient>>,
) {
    loop {
        let mut collision_engine_guard = collision_engine_clone.lock().await;
        let player_guard = player_clone.lock().await;
        let mut monster_manager_guard = monster_manager_clone.lock().await;
        let mut terminal_guard = terminal_clone.lock().await;
        let mut map_manager_guard = map_manager_clone.lock().await;

        let mut monsters_to_remove= Vec::<i32>::new();

        for monster in monster_manager_guard.get_monsters_mut().values_mut() {
            if !monster.is_alive {
                let mut map = map_manager_guard.get_mut_current_map();
                monsters_to_remove.push(monster.id);

                chat_clone.lock().await.process_debug_message(&format!("mon dead at pos: {:?}", monster.position), 3);
                map.map[monster.position.y][monster.position.x] = Space::new(DEFAULT_TILE_SET.floor);
            }
        }

        for monster_id in monsters_to_remove {
            monster_manager_guard.despawn(monster_id);
        }

        let mut new_monsters_pos = collision_engine_guard
            .try_move_monsters(
                &player_guard,
                &mut monster_manager_guard,
                &mut map_manager_guard,
                chat_clone,
            )
            .await;

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

        collision_engine_guard
            .update_player_vision(&mut map_manager_guard, &player_guard, Vec2::ZERO)
            .await;

        terminal_guard
            .print_terminal(&player_guard, &mut map_manager_guard, chat_clone)
            .await;

        drop(collision_engine_guard);
        drop(terminal_guard);
        drop(monster_manager_guard);
        drop(map_manager_guard);
        drop(player_guard);

        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}*/
