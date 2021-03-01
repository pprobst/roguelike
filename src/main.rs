use bracket_lib::prelude::*;
use specs::prelude::*;

mod common;
mod state;
use state::{RunState, State};
mod components;
pub use components::*;
mod input;
mod killer;
mod log;
mod map_gen;
use map_gen::TileType;
mod player;
mod raws;
mod renderer;
mod rexloader;
pub use raws::load_raws;
mod spawner;
mod systems;
mod ui;
mod utils;

#[macro_use]
extern crate lazy_static;

pub const X_OFFSET: i32 = 20; // Left box
pub const Y_OFFSET: i32 = 10; // Bottom box
pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;
//pub const WINDOW_HEIGHT: i32 = 80+Y_OFFSET;
pub const TILE_WIDTH: i32 = 18;
pub const TILE_HEIGHT: i32 = 18;

pub const SHOW_MAP: bool = false;

embedded_resource!(FONT01, "../resources/fonts/Sapphos-square-16x16.png");
embedded_resource!(FONT02, "../resources/fonts/Anikki-square-16x16.png");

fn main() -> BError {
    link_resource!(FONT01, "resources/Sapphos-square-16x16.png");
    link_resource!(FONT02, "resources/Anikki-square-16x16.png");

    let term = BTermBuilder::new()
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_title("TCC")
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_font("Sapphos-square-16x16.png", 16, 16)
        .with_font("Anikki-square-16x16.png", 16, 16)
        .with_simple_console(WINDOW_WIDTH, WINDOW_HEIGHT, "Sapphos-square-16x16.png")
        .with_sparse_console(WINDOW_WIDTH, WINDOW_HEIGHT, "Anikki-square-16x16.png")
        .with_fps_cap(60.0)
        .build()?;

    //term.post_scanlines = true;

    // Load external files.
    rexloader::load_dungeons();
    raws::load_raws();

    let mut world = World::new();

    // Register the components (see components.rs).
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Mob>();
    world.register::<Name>();
    world.register::<Description>();
    world.register::<InventoryCapacity>();
    world.register::<Fov>();
    world.register::<Blocker>();
    world.register::<Health>();
    world.register::<Attack>();
    world.register::<BaseStats>();
    world.register::<SufferDamage>();
    world.register::<MeleeAttack>();
    world.register::<MissileAttack>();
    world.register::<MeleeWeapon>();
    world.register::<MissileWeapon>();
    world.register::<Ammunition>();
    world.register::<ActiveWeapon>();
    world.register::<TryReload>();
    world.register::<Target>();
    world.register::<Equipable>();
    world.register::<Equipment>();
    world.register::<TryEquip>();
    world.register::<TryUnequip>();
    world.register::<Item>();
    world.register::<Armor>();
    world.register::<Consumable>();
    world.register::<CollectItem>();
    world.register::<DropItem>();
    world.register::<ConsumeItem>();
    world.register::<Inventory>();
    world.register::<SelectedItem>();
    world.register::<SelectedPosition>();
    world.register::<Remains>();
    world.register::<Container>();
    world.register::<Contained>();

    // Create game state.
    let mut game_state = State::new(world);

    // Insert map into the ECS and generate it.
    let (height, width) = (80, 60);

    game_state
        .ecs
        .insert(map_gen::Map::new(height, width, TileType::Floor, None));

    game_state.ecs.insert(RandomNumberGenerator::new());

    game_state.generate_new_map(height, width);
    game_state.ecs.insert(Point::new(0, 0));
    let player = spawner::create_player(&mut game_state.ecs);
    game_state.ecs.insert(player);

    spawner::equip_player(&mut game_state.ecs);
    game_state.populate_map();

    // Insert initial state into the ECS.
    //game_state.ecs.insert(RunState::Start);
    if !SHOW_MAP {
        game_state.ecs.insert(RunState::Menu {
            menu_selection: ui::menu::MenuSelection::NewGame,
        });
    } else {
        game_state.ecs.insert(RunState::Start);
    }

    // Insert the Log into the ECS.
    game_state.ecs.insert(log::Log::new());

    bracket_lib::prelude::main_loop(term, game_state)
}
