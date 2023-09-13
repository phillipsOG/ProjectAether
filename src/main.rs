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
mod renderer;

type Map = Vec<Vec<Space>>;

use crate::game_client::GameClient;
use crate::space::Space;
use std::io::stdout;
use crossterm::event::{KeyCode, KeyEventKind};
use crossterm::{event, terminal, QueueableCommand};
use futures::lock::Mutex;
use futures::TryFutureExt;
use std::sync::Arc;
use std::time::Duration;
use sdl2::event::Event;
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

use sdl2::pixels::Color;
//use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use crate::renderer::Renderer;

const PLAYER_MOVEMENT_SPEED: i32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum MovementType {
    Unable,
    Normal,
    LadderUp,
    LadderDown,
    LadderEnter,
    LadderExit,
    Battle,
}

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
    let monster_manager_guard = monster_manager_clone.lock().await;

    let map_manager = Arc::new(Mutex::new(MapManager::new()));
    let map_manager_clone = Arc::clone(&map_manager);
    let mut map_manager_guard = map_manager_clone.lock().await;

    map_manager_guard.add_map_set_player_position(&mut player_guard, "map2", Vec2::new(6, 2));
    map_manager_guard.load_map("map2", MovementType::Normal);

    let collision_engine = Arc::new(Mutex::new(CollisionEngine::new()));
    let collision_engine_clone = Arc::clone(&collision_engine);
    let mut collision_engine_guard = collision_engine_clone.lock().await;

    let chat = Arc::new(Mutex::new(Chat::new()));
    let mut chat_clone = Arc::clone(&chat);

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

    // prepare sdl
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // leading _ signifies unused var and will stop it being dropped as a temp val
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    // obviously the main game window
    let window = video_subsystem.window("ProjectAether", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    // draw to our window by building the canvas
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png").unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // main game loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
             match event {
                 Event::Quit {..} |

                 Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                     break 'running;
                }
                 _ => {}
             }

            canvas.clear();

            let mut player_guard = player_clone.lock().await;
            player_guard.key_event = event;

            let mut terminal_guard = terminal_clone.lock().await;
            let mut map_manager_guard = map_manager_clone.lock().await;
            let mut collision_engine_guard = collision_engine_clone.lock().await;

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
                _ => {}
            }

            collision_engine_guard
                .update_player_vision(
                    &mut map_manager_guard,
                    &mut player_guard,
                    new_player_pos,
                )
                .await;
        }

        // render
        Renderer::render_player(&mut canvas, Color::RGB(255, 255, 255), &texture, &player.lock().await).unwrap();
        //Renderer::render_map(&mut canvas, Color::RGB(255, 255, 255), &texture, &player.lock().await).unwrap();

        // call this last to present previous buffer data
        canvas.present();

        // time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    /*loop {
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
    }*/
}
