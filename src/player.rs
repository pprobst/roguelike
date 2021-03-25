use super::{
    map_gen::{common::count_neighbor_tile_entity, Map, TileType},
    utils::directions::*,
    ActiveWeapon, CollectItem, Container, EquipSlot, Equipable, Equipment, Fov, Item, MeleeAttack,
    MissileAttack, MissileWeapon, Mob, Player, Position, RunState, SelectedPosition, State, Target,
    TryReload,
};
use crate::log::Log;
use crate::utils::colors::*;
use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::*;
use std::cmp::Ordering;

/*
 *
 * player.rs
 * ---------
 * Contains all the actions performed by the player.
 *
 */

/// Tries to move the player, performing melee attacks if needed.
pub fn move_player(dir: Direction, gs: &mut State) {
    let map = gs.resources.get::<Map>().unwrap();
    let mut cb = CommandBuffer::new(&gs.ecs);

    <(Entity, &mut Player, &mut Position, &mut Fov)>::query()
        .iter_mut(&mut gs.ecs)
        .for_each(|(player_ent, _, pos, fov)| {
            let dir_x = dir.delta_x as i32;
            let dir_y = dir.delta_y as i32;
            let dest = map.idx(pos.x + dir_x, pos.y + dir_y);

            // Tries melee if you're trying to move into an occupied tile.
            for ents in map.entities[dest].iter() {
                for ent in ents.iter() {
                    if let Ok(_t) = gs.ecs.entry_ref(*ent).unwrap().get_component::<Mob>() {
                        println!("Attacking enemy.");
                        cb.add_component(*player_ent, MeleeAttack { target: *ent });
                    }
                }
            }

            if !map.tiles[dest].block {
                pos.x = pos.x + dir_x;
                pos.y = pos.y + dir_y;
                //let mut player_pos = ecs.write_resource::<Point>();
                //player_pos.x = pos.x;
                //player_pos.y = pos.y;
                //println!("New pos: {:?}", *player_pos);
                fov.dirty = true;
            }
        });
    cb.flush(&mut gs.ecs, &mut gs.resources);
}

fn get_weapon(ecs: &World, ent: Entity, wpn_slot: EquipSlot) -> Option<Entity> {
    let wpn = <(Entity, &Equipment, &Equipable)>::query()
        .iter(ecs)
        .filter(|(_, equip, slot)| slot.slot == wpn_slot && equip.user == ent)
        .map(|(ent, _, _)| ent)
        .cloned()
        .last();

    return wpn;
}

/// Checks if ent can shoot a missile weapon, that is, if the weapon is selected and has ammo.
fn can_shoot(ecs: &World, ent: Entity) -> bool {
    let wpn = get_weapon(ecs, ent, EquipSlot::Weapon2);
    match wpn {
        Some(w) => {
            let entry = ecs.entry_ref(w).unwrap();
            if let Ok(_t) = entry.get_component::<ActiveWeapon>() {
                if entry.get_component::<MissileWeapon>().unwrap().ammo.ammo > 0 {
                    return true;
                }
            }
        }
        None => return false,
    }
    false
}

/// Cycles between the player's visible targets.
pub fn choose_target(gs: &mut State, up: bool) -> RunState {
    let player = <(Entity, &Player)>::query()
        .iter(&gs.ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

    let mut log = gs.resources.get_mut::<Log>().unwrap();

    if !can_shoot(&gs.ecs, player) {
        log.add(
            format!("You can't use your ranged weapon."),
            color("BrightWhite", 1.0),
        );
        return RunState::Waiting;
    }

    let vis_targets = visible_targets(&gs, true);
    //let mut targets = ecs.write_storage::<Target>();
    //let entities = ecs.entities();

    // Just return a waiting state if there aren't any visible targets.
    if vis_targets.len() < 1 {
        return RunState::Waiting;
    }

    let mut cb = CommandBuffer::new(&gs.ecs);
    let mut curr_target: Option<Entity> = None;

    <(Entity, &Target)>::query()
        .iter(&gs.ecs)
        .for_each(|(ent, _)| {
            cb.remove_component::<Target>(*ent);
            curr_target = Some(*ent);
        });

    if let Some(curr_target) = curr_target {
        // If there's already a target selected...
        let mut idx = 0;
        for (i, target) in vis_targets.iter().enumerate() {
            // Get index from current target.
            if target.0 == curr_target {
                idx = i;
            }
        }
        if !up && idx > 0 {
            let tgt = vis_targets[idx - 1];
            cb.add_component(tgt.0, Target { covered: tgt.2 });
        } else {
            if idx + 1 > vis_targets.len() - 1 {
                idx = 0;
            } else {
                idx += 1;
            }
            let tgt = vis_targets[idx];
            cb.add_component(tgt.0, Target { covered: tgt.2 });
        }
    } else {
        // If there's not a target select already, select the first closest visible.
        let first_target = vis_targets[0];
        cb.add_component(
            first_target.0,
            Target {
                covered: first_target.2,
            },
        );
    }

    cb.flush(&mut gs.ecs, &mut gs.resources);

    RunState::Targeting
}

/// Performs a missile (ranged) attack to the selected entity.
pub fn missile_attack(ecs: &mut World) {
    let mut curr_target: Option<Entity> = None;

    <(Entity, &Target)>::query().iter(ecs).for_each(|(ent, _)| {
        if let Some(entry) = ecs.entry(*ent) {
            entry.remove_component::<Target>();
            curr_target = Some(*ent);
        }
    });

    if let Some(target) = curr_target {
        let player = <(Entity, &Player)>::query()
            .iter(ecs)
            .find_map(|(entity, _player)| Some(*entity))
            .unwrap();
        if let Ok(t) = ecs.entry_ref(target).unwrap().get_component::<Target>() {
            println!("Target OK");
            if !t.covered {
                let player_entry = ecs.entry(player).unwrap();
                player_entry.add_component(MissileAttack { target });
            }
        }
    }
}

/// Cancels targeting, returning a Waiting state.
pub fn reset_targeting(ecs: &mut World) -> RunState {
    <(Entity, &Target)>::query()
        .iter(ecs)
        .for_each(|(ent, target)| {
            if let Some(entry) = ecs.entry(*ent) {
                entry.remove_component::<Target>();
            }
        });
    RunState::Waiting
}

/// Returns all the visible and/or hittable targets in the player's FOV ordered by distance to the player (cresc.).
fn visible_targets(gs: &State, hittable: bool) -> Vec<(Entity, f32, bool)> {
    let mut visible_targets: Vec<(Entity, f32, bool)> = Vec::new(); // (entity, distance, map_idx)
    let map = gs.resources.get::<Map>().unwrap();
    let ecs = &gs.ecs;

    <(Entity, &Player, &Fov, &Position)>::query()
        .iter(ecs)
        .for_each(|(e, player, fov, ppos)| {
            let player_entry = ecs.entry_ref(*e).unwrap();
            for pos in fov.visible_pos.iter() {
                let idx = map.idx(pos.x, pos.y);
                for ents in map.entities[idx].iter() {
                    for ent in ents.iter() {
                        if let Ok(_t) = ecs.entry_ref(*ent).unwrap().get_component::<Mob>() {
                            let mobpos = Point::new(pos.x, pos.y);
                            let mut covered = false;
                            if hittable {
                                let points = line2d_vector(*ppos, mobpos);
                                for pt in points.iter().take(points.len() - 1) {
                                    let i = map.idx(pt.x, pt.y);
                                    // if there's a blocker in the aim line, you can't hit the entity.
                                    if map.tiles[i].block {
                                        covered = true;
                                    }
                                }
                            }
                            let dist = DistanceAlg::Pythagoras.distance2d(mobpos, *ppos);
                            visible_targets.push((*ent, dist, covered));
                        }
                    }
                }
            }
        });

    visible_targets.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
    visible_targets
}

/// Switches between the two readied weapons.
pub fn switch_weapon(ecs: &mut World) -> RunState {
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

    let wpns = <(Entity, &Equipment, &Equipable)>::query()
        .iter(ecs)
        .filter(|(_, equip, slot)| {
            (slot.slot == EquipSlot::Weapon1 || slot.slot == EquipSlot::Weapon2)
                && equip.user == player
        })
        .map(|(ent, _, _)| ent)
        .collect::<Vec<_>>();

    if wpns.len() > 1 {
        let wpn_0 = ecs.entry(*wpns[0]).unwrap();
        let wpn_1 = ecs.entry(*wpns[1]).unwrap();
        let weapon_to_switch = if let Ok(_t) = wpn_0.get_component::<ActiveWeapon>() {
            wpn_0.remove_component::<ActiveWeapon>();
            wpns[1]
        } else {
            wpn_1.remove_component::<ActiveWeapon>();
            wpns[0]
        };
        ecs.entry(*weapon_to_switch)
            .unwrap()
            .add_component(ActiveWeapon {});
        return RunState::PlayerTurn;
    }

    RunState::Waiting
}

/// Reload ranged weapon.
pub fn reload_weapon(ecs: &World) -> RunState {
    let player_ent = ecs.fetch::<Entity>();
    let active_wpn = ecs.read_storage::<ActiveWeapon>();
    let missile_wpn = ecs.read_storage::<MissileWeapon>();
    let mut try_reload = ecs.write_storage::<TryReload>();

    let wpn = get_weapon(ecs, *player_ent, EquipSlot::Weapon2);

    match wpn {
        Some(w) => {
            if let Some(_t) = active_wpn.get(w) {
                let ranged_wpn = missile_wpn.get(w).unwrap();
                if ranged_wpn.ammo.ammo < ranged_wpn.ammo.max_ammo {
                    try_reload
                        .insert(*player_ent, TryReload { weapon: w })
                        .expect("Reload insertion failed");
                    return RunState::PlayerTurn;
                } else {
                    return RunState::Waiting;
                }
            }
            return RunState::Waiting;
        }
        _ => return RunState::Waiting,
    }
}

enum PossibleContexts {
    Nothing,
    Door,
    Container,
    //ExitLevel,
}

/// Does a contextual action (i.e. opens a door if there's one nearby, talk, etc).
pub fn context_action(ecs: &mut World) -> RunState {
    let ppos = **(&ecs.fetch::<Point>());
    let mut map = ecs.fetch_mut::<Map>();

    // If the player is over the ">", go to the next level.
    if map.is_exit(map.idx_pt(ppos)) {
        return RunState::NextLevel;
    }

    let tile_list = vec![TileType::OpenDoor, TileType::ClosedDoor];
    let possible_count_dir = count_neighbor_tile_entity(&map, ppos, tile_list, true);

    // One tile/entity.
    if possible_count_dir.0 == 0 {
        return RunState::Waiting;
    } else if possible_count_dir.0 == 1 {
        if possible_count_dir.1 >= 8 {
            return check_near(&ecs, ppos, &mut map);
        } else {
            return check_near(&ecs, ppos + dir_idx(possible_count_dir.1), &mut map);
        }
    } else {
        // Multiple tiles/entities, so choose dir.
        return RunState::ChooseActionDir;
    }
}

pub fn check_near(ecs: &World, pt: Point, map: &mut Map) -> RunState {
    //let selected_pt = *pt + dir_idx(i);
    //let idx = map.idx_pt(selected_pt);
    let mut context = PossibleContexts::Nothing;
    let idx = map.idx_pt(pt);
    let tile = map.tiles[idx].ttype;

    // Check for entities (e.g. containers).
    match &map.entities[idx] {
        Some(ents) => {
            for ent in ents.iter() {
                let containers = ecs.read_storage::<Container>();
                let c = containers.get(*ent);
                if let Some(_c) = c {
                    let mut selected_pos = ecs.write_storage::<SelectedPosition>();
                    selected_pos
                        .insert(*ent, SelectedPosition { pos: pt })
                        .expect("Could not select position.");
                    context = PossibleContexts::Container;
                }
            }
        }
        None => {}
    }

    // Check for tiles (e.g. doors).
    match tile {
        TileType::ClosedDoor => {
            try_door(TileType::ClosedDoor, map, idx);
            context = PossibleContexts::Door;
        }
        TileType::OpenDoor => {
            try_door(TileType::OpenDoor, map, idx);
            context = PossibleContexts::Door;
        }
        _ => {}
    }

    match context {
        PossibleContexts::Door => {
            let mut fov = ecs.write_storage::<Fov>();
            let ents = ecs.entities();
            for (_ent, fov) in (&ents, &mut fov).join() {
                fov.dirty = true;
            }
            return RunState::PlayerTurn;
        }
        PossibleContexts::Container => {
            return RunState::AccessContainer;
        }
        _ => return RunState::Waiting,
    }
}

fn try_door(ttype: TileType, map: &mut Map, idx: usize) {
    if ttype == TileType::ClosedDoor {
        map.paint_tile(idx, TileType::OpenDoor);
    } else {
        map.paint_tile(idx, TileType::ClosedDoor);
    }

    map.reveal(idx);
}

/// Picks up item from the player's current position.
pub fn collect_item(ecs: &mut World) -> RunState {
    let ents = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let player_ent = ecs.fetch::<Entity>();
    let ppos = ecs.fetch::<Point>();

    let item_to_collect: Option<Entity> =
        (&ents, &items, &positions)
            .join()
            .find_map(|(ent, _item, pos)| {
                if pos.x == ppos.x && pos.y == ppos.y {
                    return Some(ent);
                }
                return None;
            });

    match item_to_collect {
        Some(item) => {
            let mut collect = ecs.write_storage::<CollectItem>();
            CollectItem::add_collect(&mut collect, item, *player_ent);
            RunState::PlayerTurn
        }
        None => RunState::Waiting,
    }
}
