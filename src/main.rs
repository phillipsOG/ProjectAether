mod basic_colour;
mod battle_system;
mod camera;
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
mod renderer;
mod space;
mod space_factory;
mod status;
mod terrain_data;
mod tile_set;
mod vec2;

type Map = Vec<Vec<Space>>;

use crate::game_client::GameClient;
use crate::space::Space;
use crossterm::{terminal, QueueableCommand};
use futures::lock::Mutex;
use std::collections::HashMap;
use std::io::stdout;

use sdl2::event::Event;
use std::sync::Arc;
use std::time::Duration;
use vec2::Vec2;

use crate::battle_system::BattleSystem;
use crate::chat::Chat;
use crate::collision_engine::CollisionEngine;
use crate::map_factory::MapFactory;
use crate::map_manager::MapManager;
use crate::monster_generator::MonsterFactory;
use crate::monster_manager::MonsterManager;
use crate::player::Player;
use crate::tile_set::{DEFAULT_TILE_SET, MONSTER_TILE_SET};

use crate::basic_colour::COLOUR;
use crate::camera::Camera;
use crate::map_data::MapData;
use crate::monster::Monster;
use crate::renderer::Renderer;
use crate::space_factory::SpaceFactory;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;

type Monsters = HashMap<i32, Monster>;

const PLAYER_MOVEMENT_SPEED: i32 = 5;
const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

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

#[derive(PartialEq)]
enum RotationAngle {
    None,
    Degrees90,
    Degrees180,
    Degrees270,
}

// define grid parameters
const GRID_ROWS: usize = 4; // Number of rows
const GRID_COLS: usize = 4; // Number of columns
const CELL_SIZE: i32 = 1; // Size of each grid cell in pixels
const START_X: i32 = 0; // X-coordinate of the top-left corner of the grid
const START_Y: i32 = 0; // Y-coordinate of the top-left corner of the grid

#[tokio::main]
async fn main() {
    let player = Arc::new(Mutex::new(Player::new()));
    let player_clone = Arc::clone(&player);
    let mut player_guard = player_clone.lock().await;

    let mut map_factory = MapFactory::new();
    let monster_factory = MonsterFactory::new();

    let terminal = Arc::new(Mutex::new(GameClient::new()));
    let terminal_clone = Arc::clone(&terminal);
    let terminal_guard = terminal_clone.lock().await;

    let monster_manager = Arc::new(Mutex::new(MonsterManager::new()));
    let monster_manager_clone = Arc::clone(&monster_manager);
    let mut monster_manager_guard = monster_manager_clone.lock().await;

    let map_manager = Arc::new(Mutex::new(MapManager::new()));
    let map_manager_clone = Arc::clone(&map_manager);
    let mut map_manager_guard = map_manager_clone.lock().await;

    // we generate a gfx map, every gfx tile has a tile name and we render later tiles by name
    // this map is stored in a 2d vector of type Space which stores a point to their texture on the sprite sheet
    // we need to draw the walls/floors etc. and then draw the player on top
    let mut new_graphical_map =
        map_factory.generate_graphical_map(&mut player_guard, 20, 20, "seedphrase");

    // @TODO handle this properly
    let mut new_map = MapData::new();

    //copy the graphical map we made to the normal map the player uses to move/monsters
    new_map.map = new_graphical_map.clone();

    let spawn_pos = Vec2::new(7, 5);
    map_factory.generate_object(
        DEFAULT_TILE_SET.key,
        spawn_pos,
        &mut new_graphical_map,
        &mut new_map.map,
        RotationAngle::None,
    );

    let spawn_pos = Vec2::new(0, 0);
    map_factory.generate_object(
        DEFAULT_TILE_SET.room,
        spawn_pos,
        &mut new_graphical_map,
        &mut new_map.map,
        RotationAngle::None,
    );

    let spawn_pos = Vec2::new(5, 4);
    map_factory.generate_object(
        DEFAULT_TILE_SET.room,
        spawn_pos,
        &mut new_graphical_map,
        &mut new_map.map,
        RotationAngle::Degrees180,
    );

    let spawn_pos = Vec2::new(5, 9);
    map_factory.generate_object(
        DEFAULT_TILE_SET.wall_stack,
        spawn_pos,
        &mut new_graphical_map,
        &mut new_map.map,
        RotationAngle::None,
    );

    let spawn_pos = Vec2::new(10, 9);
    map_factory.generate_object(
        DEFAULT_TILE_SET.wall_stack,
        spawn_pos,
        &mut new_graphical_map,
        &mut new_map.map,
        RotationAngle::None,
    );

    let spawn_pos = Vec2::new(9, 13);
    map_factory.generate_object(
        DEFAULT_TILE_SET.wall_stack,
        spawn_pos,
        &mut new_graphical_map,
        &mut new_map.map,
        RotationAngle::Degrees90,
    );

    let spawn_pos = Vec2::new(13, 13);
    map_factory.generate_object(
        DEFAULT_TILE_SET.wall_stack,
        spawn_pos,
        &mut new_graphical_map,
        &mut new_map.map,
        RotationAngle::Degrees90,
    );

    let spawn_pos = Vec2::new(5, 9);
    map_factory.generate_object(
        DEFAULT_TILE_SET.wall_stack,
        spawn_pos,
        &mut new_graphical_map,
        &mut new_map.map,
        RotationAngle::Degrees90,
    );

    let spawn_pos = Vec2::new(3, 4);
    map_factory.generate_object(
        DEFAULT_TILE_SET.wall_stack,
        spawn_pos,
        &mut new_graphical_map,
        &mut new_map.map,
        RotationAngle::Degrees90,
    );

    let spawn_pos = Vec2::new(0, 4);
    map_factory.generate_object(
        DEFAULT_TILE_SET.room,
        spawn_pos,
        &mut new_graphical_map,
        &mut new_map.map,
        RotationAngle::None,
    );

    let spawn_pos = Vec2::new(2, 7);
    map_factory.generate_object(
        DEFAULT_TILE_SET.key,
        spawn_pos,
        &mut new_graphical_map,
        &mut new_map.map,
        RotationAngle::None,
    );

    let plr_spawn = Vec2::new(1, 6);
    player_guard.position = plr_spawn;
    player_guard.tile_below_player = DEFAULT_TILE_SET.floor;
    new_map.set_player_position(plr_spawn);

    new_map.width = new_map.map.len();
    new_map.height = if new_map.width > 0 {
        new_map.map[0].len()
    } else {
        0
    };

    map_manager_guard.add_generated_map(new_map);

    //map_manager_guard.add_map_set_player_position(&mut player_guard, "map2", Vec2::new(6, 2));
    map_manager_guard.load_map("gen_map", MovementType::Normal);

    monster_manager_guard.spawn_monsters(&mut map_manager_guard, monster_factory);

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

    drop(terminal_guard);
    drop(map_manager_guard);
    drop(collision_engine_guard);
    drop(monster_manager_guard);
    drop(player_guard);

    // prepare sdl
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // leading _ signifies unused var and will stop it being dropped as a temp val
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    // obviously the main game window
    let window = video_subsystem
        .window("ProjectAether", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    // draw to our window by building the canvas
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

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

    let mut cur_pos = plr_spawn;

    let mut display_grid_coordinates = false;

    // main game loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(COLOUR.black);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::LCtrl),
                    ..
                } => {
                    if display_grid_coordinates == false {
                        display_grid_coordinates = true;
                    } else {
                        display_grid_coordinates = false;
                    }
                }
                _ => {}
            }

            let mut player_guard = player_clone.lock().await;
            player_guard.key_event = event;

            let mut map_manager_guard = map_manager_clone.lock().await;
            let mut collision_engine_guard = collision_engine_clone.lock().await;
            let mut monster_manager_guard = monster_manager_clone.lock().await;

            let culled_monsters_positions = monster_manager_guard.cull_dead_monsters();
            for culled_monster_position in culled_monsters_positions {
                map_manager_guard.get_mut_current_map().map[culled_monster_position.y][culled_monster_position.x] =
                    SpaceFactory::generate_space(DEFAULT_TILE_SET.floor);
                new_graphical_map[culled_monster_position.y][culled_monster_position.x] =
                    SpaceFactory::generate_space(DEFAULT_TILE_SET.floor);
            }

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
                    cur_pos = new_player_pos;
                }
                MovementType::Battle => {
                    let position = Vec2::new(new_player_pos.x, new_player_pos.y);

                    if let Some(monster) = monster_manager_guard.get_monster_at_position(position) {
                        BattleSystem::start_battle(&mut player_guard, monster, &chat_clone).await;
                    }
                }
                _ => {}
            }

            collision_engine_guard
                .update_player_vision(&mut map_manager_guard, &mut player_guard, new_player_pos)
                .await;

            drop(map_manager_guard);
            drop(player_guard);
        }

        let player_guard = player_clone.lock().await;
        let mut map_manager_guard = map_manager_clone.lock().await;
        let mut monster_manager_guard = monster_manager_clone.lock().await;

        let mut camera = Camera::new(&player_guard); // create the camera with the player reference
        camera.update_camera_position(cur_pos, SCREEN_WIDTH, SCREEN_HEIGHT);

        let tile_width = 55;
        let tile_height = 60;

        let cell_x = 0;
        let cell_y = 0;

        let tiles_to_render = vec![
            DEFAULT_TILE_SET.wall,
            DEFAULT_TILE_SET.floor,
            DEFAULT_TILE_SET.closed_door_side,
            DEFAULT_TILE_SET.open_door,
            MONSTER_TILE_SET.snake,
            DEFAULT_TILE_SET.key
        ];

        let monsters_to_render = vec![MONSTER_TILE_SET.snake];

        // we pass in the normal map because the graphical map doesn't have tile visibility updated
        // renders the base map, walls and floor (can update this later for procedural gen)
        // doors closed/open, keys etc.
        // monsters, snakes, goblins, goons, ghouls etc.
        Renderer::render_tiles(
            &mut canvas,
            &mut new_graphical_map,
            &mut map_manager_guard.get_mut_current_map(),
            &tiles_to_render,
            cell_x,
            cell_y,
            tile_width,
            tile_height,
            camera.x,
            camera.y,
            display_grid_coordinates
        )
        .unwrap();
        Renderer::render_player(
            &mut canvas,
            &player_guard,
            &mut map_manager_guard.get_mut_current_map(),
            cell_x,
            cell_y,
            camera.x,
            camera.y,
        )
        .unwrap();
        Renderer::render_monsters_status(
            &mut canvas,
            &mut map_manager_guard.get_mut_current_map(),
            monster_manager_guard.get_monsters_mut(),
            &monsters_to_render,
            cell_x,
            cell_y,
            tile_width,
            tile_height,
            camera.x,
            camera.y,
        )
        .unwrap();

        let culled_monsters_positions = monster_manager_guard.cull_dead_monsters();
        for culled_monster_position in culled_monsters_positions {
            map_manager_guard.get_mut_current_map().map[culled_monster_position.y][culled_monster_position.x] =
                SpaceFactory::generate_space(DEFAULT_TILE_SET.floor);
            new_graphical_map[culled_monster_position.y][culled_monster_position.x] =
                SpaceFactory::generate_space(DEFAULT_TILE_SET.floor);
        }

        //drop(collision_engine_guard);
        drop(monster_manager_guard);
        drop(map_manager_guard);
        drop(player_guard);

        // call this last to present previous buffer data
        canvas.present();

        // time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
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
        let terminal_guard = terminal_clone.lock().await;
        let mut map_manager_guard = map_manager_clone.lock().await;

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

        // relic
        /*terminal_guard
        .print_terminal(&player_guard, &mut map_manager_guard, chat_clone)
        .await;*/

        drop(collision_engine_guard);
        drop(terminal_guard);
        drop(monster_manager_guard);
        drop(map_manager_guard);
        drop(player_guard);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1));
    }

}
