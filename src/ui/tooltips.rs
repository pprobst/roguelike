use super::{common::Popup, WINDOW_HEIGHT, WINDOW_WIDTH, X_OFFSET, Y_OFFSET};
use crate::components::{
    Armor, BaseStats, Description, Item, MeleeWeapon, MissileWeapon, Name, Position,
};
use crate::map_gen::Map;
use bracket_lib::prelude::*;
use specs::prelude::*;

/*
 *
 * tooltips.rs
 * -----------
 * Check information of entities on the game by hovering the mouse over them.
 * Easier than making a keyboard-based (l)ook system, as bracket-lib has mouse support.
 *
 */

pub fn show_tooltip(
    ecs: &World,
    term: &mut BTerm,
    draw_batch: &mut DrawBatch,
    min_x: i32,
    min_y: i32,
) {
    let map = ecs.fetch::<Map>();
    let mouse_real_pos = term.mouse_pos();
    let mut mouse_pos = mouse_real_pos;
    mouse_pos.0 += min_x - X_OFFSET;
    mouse_pos.1 += min_y + Y_OFFSET;

    let names = ecs.read_storage::<Name>();
    let descriptions = ecs.read_storage::<Description>();
    let positions = ecs.read_storage::<Position>();
    let stats = ecs.read_storage::<BaseStats>();
    let melee = ecs.read_storage::<MeleeWeapon>();
    let missile = ecs.read_storage::<MissileWeapon>();
    let armor = ecs.read_storage::<Armor>();
    let item = ecs.read_storage::<Item>();
    let entities = ecs.entities();

    let mut tooltips: Vec<Popup> = Vec::new();
    for (ent, name, descr, pos) in (&entities, &names, &descriptions, &positions).join() {
        let idx = map.idx(pos.x, pos.y);
        if mouse_pos.0 == pos.x && mouse_pos.1 == pos.y && map.is_visible(idx) {
            let mut ttip = Popup::new();
            ttip.add(name.name.to_string());
            ttip.add(descr.descr.to_string());
            if let Some(s) = stats.get(ent) {
                ttip.add(format!("\nHP: {}", s.health.hp));
            }
            if let Some(m) = melee.get(ent) {
                ttip.add(format!("\n{:?}\n\nDMG: {}", m.class, m.stats.base_damage));
            }
            if let Some(m) = missile.get(ent) {
                ttip.add(format!("\n{:?}\n\nDMG: {}", m.class, m.stats.base_damage));
            }
            if let Some(a) = armor.get(ent) {
                ttip.add(format!("\nDEF: {}", a.defense));
            }
            if let Some(t) = item.get(ent) {
                ttip.add(format!("\nTier: {}", t.tier));
            }
            tooltips.push(ttip);
        }
    }
    if tooltips.is_empty() {
        return;
    }

    for tooltip in tooltips.iter() {
        let mut x = mouse_real_pos.0 + 1;
        let mut y = mouse_real_pos.1 + 1;
        if x + tooltip.width() >= WINDOW_WIDTH {
            x = x - tooltip.width() + 5;
        }
        if y + tooltip.height() >= WINDOW_HEIGHT - Y_OFFSET {
            y = y - tooltip.height() - 1;
        }
        tooltip.render_tooltip(x, y, draw_batch);
    }
}
