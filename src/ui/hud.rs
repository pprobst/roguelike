use super::{Log, WINDOW_HEIGHT, WINDOW_WIDTH, X_OFFSET, Y_OFFSET};
use crate::components::{
    ActiveWeapon, BaseStats, EquipSlot, EquipSlot::*, Equipable, Equipment, MissileWeapon, Name,
};
use crate::utils::colors::*;
use bracket_lib::prelude::*;
use specs::prelude::*;

/*
 *
 * hud.rs
 * ------
 * Responsible for rendering and defining the player's HUD.
 *
 */

const X: i32 = WINDOW_WIDTH;
const Y: i32 = WINDOW_HEIGHT;
const MSG_HEIGHT_MIN: i32 = Y - Y_OFFSET + 1;
const MSG_HEIGHT_MAX: i32 = Y - 1;

/// Renders the UI skeleton.
pub fn boxes(draw_batch: &mut DrawBatch) {
    let black = color("Background", 1.0);
    let gray = color("BrightBlack", 1.0);

    draw_batch.draw_hollow_box(
        Rect::with_size(0, 0, X - 1, Y - 1),
        ColorPair::new(gray, black),
    ); // Screen borders
    draw_batch.draw_hollow_box(
        Rect::with_size(0, 0, X_OFFSET, Y - 1),
        ColorPair::new(gray, black),
    ); // Left box
    draw_batch.draw_hollow_box(
        Rect::with_size(X_OFFSET, Y - Y_OFFSET, X - X_OFFSET - 1, -Y_OFFSET + 1),
        ColorPair::new(gray, black),
    ); // Bottom box
    draw_batch.set(
        Point::new(X_OFFSET, Y - Y_OFFSET),
        ColorPair::new(gray, black),
        to_cp437('├'),
    );
    draw_batch.set(
        Point::new(X - 1, Y - Y_OFFSET),
        ColorPair::new(gray, black),
        to_cp437('┤'),
    );
    draw_batch.set(
        Point::new(X_OFFSET, Y - 1),
        ColorPair::new(gray, black),
        to_cp437('┴'),
    );
    draw_batch.set(
        Point::new(X_OFFSET, 0),
        ColorPair::new(gray, black),
        to_cp437('┬'),
    );
}

/// Renders the player's name and their possible stats.
pub fn name_stats(ecs: &World, draw_batch: &mut DrawBatch) {
    let black = color("Background", 1.0);
    let white = color("BrightWhite", 1.0);
    let red = color("Red", 1.0);
    let cyan = color("Cyan", 1.0);
    let med_red = color("BrightRed", 1.0);
    let player = ecs.fetch::<Entity>();
    let names = ecs.read_storage::<Name>();
    let stats = ecs.read_storage::<BaseStats>();

    let player_name = names.get(*player).unwrap();
    let pname = format!("{}", player_name.name);
    let player_stats = stats.get(*player).unwrap();
    let phealth = format!("{}/{}", player_stats.health.hp, player_stats.health.max_hp);

    let y = 3;
    let bar_end = X_OFFSET - 9;

    draw_batch.print_color(Point::new(3, y), pname, ColorPair::new(white, black));
    draw_batch.set(Point::new(1, y), ColorPair::new(cyan, black), to_cp437('»'));
    draw_batch.set(
        Point::new(1, y + 2),
        ColorPair::new(med_red, black),
        to_cp437('Ω'),
    );
    draw_batch.bar_horizontal(
        Point::new(3, y + 2),
        bar_end,
        player_stats.health.hp,
        player_stats.health.max_hp,
        ColorPair::new(red, black),
    );
    draw_batch.print_color(
        Point::new(bar_end + 4, y + 2),
        phealth,
        ColorPair::new(white, black),
    );

    let mut health_status = "• Fine";
    let mut health_status_color = RGB::from_hex(GRASS_GREEN).unwrap();
    if player_stats.health.hp < player_stats.health.max_hp / 10 + 2 {
        health_status = "• Danger";
        health_status_color = RGB::from_hex(MED_RED).unwrap();
    } else if player_stats.health.hp <= player_stats.health.max_hp / 2 {
        health_status = "• Wounded";
        health_status_color = RGB::named(ORANGE);
    }

    draw_batch.print_color(
        Point::new(2, y + 4),
        health_status,
        ColorPair::new(health_status_color, black),
    );
}

pub fn show_equipped(ecs: &World, draw_batch: &mut DrawBatch) {
    let equips = ecs.read_storage::<Equipment>();
    let equipables = ecs.read_storage::<Equipable>();
    let names = ecs.read_storage::<Name>();
    let active_wpn = ecs.read_storage::<ActiveWeapon>();
    let player = ecs.fetch::<Entity>();
    let entities = ecs.entities();

    let black = color("Background", 1.0);
    let white = color("BrightWhite", 1.0);
    let gray = color("BrightBlack", 1.0);
    let mut melee_color = white;
    let mut ranged_color = white;

    let mut equipment: Vec<(&str, EquipSlot)> = vec![
        ("None", Weapon1),
        ("None", Weapon2),
        ("None", Head),
        ("None", Torso),
        ("None", Hands),
        ("None", Legs),
        ("None", Feet),
        ("None", Back),
        ("None", Floating),
    ];

    let mut ammo = "".to_string();
    for (equip, equipable, name, ent) in (&equips, &equipables, &names, &entities).join() {
        if equip.user == *player {
            match equipable.slot {
                Weapon1 => {
                    equipment[0].0 = &name.name;
                    if let Some(_t) = active_wpn.get(ent) {
                        melee_color = color("Cyan", 1.0);
                    }
                }
                Weapon2 => {
                    equipment[1].0 = &name.name;
                    let missile_wpn = ecs.read_storage::<MissileWeapon>();
                    let wpn = missile_wpn.get(ent).unwrap();
                    ammo = format!("{}/{}", wpn.ammo.ammo, wpn.ammo.max_ammo);
                    if let Some(_t) = active_wpn.get(ent) {
                        ranged_color = color("Cyan", 1.0);
                    }
                }
                Head => equipment[2].0 = &name.name,
                Torso => equipment[3].0 = &name.name,
                Hands => equipment[4].0 = &name.name,
                Legs => equipment[5].0 = &name.name,
                Feet => equipment[6].0 = &name.name,
                Back => equipment[7].0 = &name.name,
                _ => equipment[8].0 = &name.name,
            }
        }
    }

    let y = 10;
    draw_batch.print_color(Point::new(0, y), "╞═ MELEE", ColorPair::new(gray, black));
    draw_batch.print_color(
        Point::new(3, y + 1),
        equipment[0].0,
        ColorPair::new(melee_color, black),
    );
    draw_batch.print_color(
        Point::new(0, y + 3),
        format!("╞═ RANGED {}", ammo),
        ColorPair::new(gray, black),
    );
    draw_batch.print_color(
        Point::new(3, y + 4),
        equipment[1].0,
        ColorPair::new(ranged_color, black),
    );
    draw_batch.print_color(Point::new(0, y + 6), "╞═ HEAD", ColorPair::new(gray, black));
    draw_batch.print_color(
        Point::new(3, y + 7),
        equipment[2].0,
        ColorPair::new(white, black),
    );
    draw_batch.print_color(
        Point::new(0, y + 9),
        "╞═ TORSO",
        ColorPair::new(gray, black),
    );
    draw_batch.print_color(
        Point::new(3, y + 10),
        equipment[3].0,
        ColorPair::new(white, black),
    );
    draw_batch.print_color(
        Point::new(0, y + 12),
        "╞═ HANDS",
        ColorPair::new(gray, black),
    );
    draw_batch.print_color(
        Point::new(3, y + 13),
        equipment[4].0,
        ColorPair::new(white, black),
    );
    draw_batch.print_color(
        Point::new(0, y + 15),
        "╞═ LEGS",
        ColorPair::new(gray, black),
    );
    draw_batch.print_color(
        Point::new(3, y + 16),
        equipment[5].0,
        ColorPair::new(white, black),
    );
    draw_batch.print_color(
        Point::new(0, y + 18),
        "╞═ FEET",
        ColorPair::new(gray, black),
    );
    draw_batch.print_color(
        Point::new(3, y + 19),
        equipment[6].0,
        ColorPair::new(white, black),
    );
    draw_batch.print_color(
        Point::new(0, y + 21),
        "╞═ BACK",
        ColorPair::new(gray, black),
    );
    draw_batch.print_color(
        Point::new(3, y + 22),
        equipment[7].0,
        ColorPair::new(white, black),
    );
    draw_batch.print_color(
        Point::new(0, y + 24),
        "╞═ FLOATING",
        ColorPair::new(gray, black),
    );
    draw_batch.print_color(
        Point::new(3, y + 25),
        equipment[8].0,
        ColorPair::new(white, black),
    );
}

/// Renders messages from the log structure.
pub fn game_log(ecs: &World, draw_batch: &mut DrawBatch) {
    let log = ecs.fetch::<Log>();
    let mut y = MSG_HEIGHT_MIN;
    let bg = color("Background", 1.0);

    for &(ref msg, color) in log.messages.iter().rev() {
        //println!("{}", msg);
        //println!("{}, {}", y, Y-Y_OFFSET-2);
        if y < MSG_HEIGHT_MAX {
            draw_batch.print_color(Point::new(X_OFFSET + 1, y), msg, ColorPair::new(color, bg));
        }
        y += 1;
    }
}
