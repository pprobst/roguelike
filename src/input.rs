use super::{
    map_gen::Map,
    player::*,
    state::{RunState, State},
    utils::directions::*,
};
use bracket_lib::prelude::*;

/*
 *
 * inputs.rs
 * ---------
 * Contains all the valid inputs for a given game state.
 *
 */

/// Valid inputs while playing normally.
pub fn player_input(gs: &mut State, term: &mut BTerm) -> RunState {
    //let input_mutex = &INPUT.lock();
    //let lshift_down = input_mutex.is_key_pressed(VirtualKeyCode::LShift);
    match term.key {
        None => return RunState::Waiting,
        Some(key) => match key {
            // Move East (E).
            VirtualKeyCode::L | VirtualKeyCode::Numpad6 | VirtualKeyCode::Right => {
                move_player(EAST, &mut gs.ecs)
            }
            // Move West (W).
            VirtualKeyCode::H | VirtualKeyCode::Numpad4 | VirtualKeyCode::Left => {
                move_player(WEST, &mut gs.ecs)
            }
            // Move North (N).
            VirtualKeyCode::K | VirtualKeyCode::Numpad8 | VirtualKeyCode::Up => {
                move_player(NORTH, &mut gs.ecs)
            }
            // Move South (S).
            VirtualKeyCode::J | VirtualKeyCode::Numpad2 | VirtualKeyCode::Down => {
                move_player(SOUTH, &mut gs.ecs)
            }
            // Move Northeast (NE).
            VirtualKeyCode::U | VirtualKeyCode::Numpad9 => move_player(NORTHEAST, &mut gs.ecs),
            // Move Northwest (NW).
            VirtualKeyCode::Y | VirtualKeyCode::Numpad7 => move_player(NORTHWEST, &mut gs.ecs),
            // Move Southeast (SE).
            VirtualKeyCode::N | VirtualKeyCode::Numpad3 => move_player(SOUTHEAST, &mut gs.ecs),
            // Move Southwest (SW).
            VirtualKeyCode::B | VirtualKeyCode::Numpad1 => move_player(SOUTHWEST, &mut gs.ecs),

            // Use missile weapon.
            VirtualKeyCode::F => return choose_target(&mut gs.ecs, false),

            // Switch readied weapon.
            VirtualKeyCode::Z => return switch_weapon(&mut gs.ecs),

            // Pickup item.
            VirtualKeyCode::G => return collect_item(&mut gs.ecs),

            // Access inventory.
            VirtualKeyCode::I => {
                term.key = None;
                return RunState::Inventory;
            }

            // Equipment management.
            VirtualKeyCode::E => {
                term.key = None;
                return RunState::Equipment;
            }

            // Reload ranged weapon.
            VirtualKeyCode::R => return reload_weapon(&mut gs.ecs),

            VirtualKeyCode::Space => return context_action(&mut gs.ecs),

            // Wait (skip turn).
            VirtualKeyCode::Period => return RunState::PlayerTurn,

            _ => return RunState::Waiting,
        },
    }
    RunState::PlayerTurn
}

/// Valid inputs while in Targeting mode.
pub fn targeting_input(gs: &mut State, term: &mut BTerm) -> RunState {
    match term.key {
        None => return RunState::Targeting,
        Some(key) => match key {
            VirtualKeyCode::K | VirtualKeyCode::Numpad8 | VirtualKeyCode::Up => {
                return choose_target(&mut gs.ecs, true)
            }

            VirtualKeyCode::J | VirtualKeyCode::Numpad2 | VirtualKeyCode::Down => {
                return choose_target(&mut gs.ecs, false)
            }

            // Use missile weapon.
            VirtualKeyCode::F => missile_attack(&mut gs.ecs),

            // Cancel targeting mode.
            VirtualKeyCode::Escape => return reset_targeting(&mut gs.ecs),

            _ => return RunState::Targeting,
        },
    }
    RunState::PlayerTurn
}

/// Valid inputs while in Targeting mode.
pub fn action_dir_input(gs: &mut State, term: &mut BTerm) -> RunState {
    let ppos = **(&gs.ecs.fetch::<Point>());
    let mut map = gs.ecs.fetch_mut::<Map>();
    match term.key {
        None => return RunState::ChooseActionDir,
        Some(key) => match key {
            VirtualKeyCode::Period | VirtualKeyCode::Numpad5 => {
                return check_near(&gs.ecs, ppos, &mut map);
            }
            // Move East (E).
            VirtualKeyCode::L | VirtualKeyCode::Numpad6 | VirtualKeyCode::Right => {
                return check_near(&gs.ecs, ppos + EAST, &mut map);
            }
            // Move West (W).
            VirtualKeyCode::H | VirtualKeyCode::Numpad4 | VirtualKeyCode::Left => {
                return check_near(&gs.ecs, ppos + WEST, &mut map);
            }
            // Move North (N).
            VirtualKeyCode::K | VirtualKeyCode::Numpad8 | VirtualKeyCode::Up => {
                return check_near(&gs.ecs, ppos + NORTH, &mut map);
            }
            // Move South (S).
            VirtualKeyCode::J | VirtualKeyCode::Numpad2 | VirtualKeyCode::Down => {
                return check_near(&gs.ecs, ppos + SOUTH, &mut map);
            }
            // Move Northeast (NE).
            VirtualKeyCode::U | VirtualKeyCode::Numpad9 => {
                return check_near(&gs.ecs, ppos + NORTHEAST, &mut map);
            }
            // Move Northwest (NW).
            VirtualKeyCode::Y | VirtualKeyCode::Numpad7 => {
                return check_near(&gs.ecs, ppos + NORTHWEST, &mut map);
            }
            // Move Southeast (SE).
            VirtualKeyCode::N | VirtualKeyCode::Numpad3 => {
                return check_near(&gs.ecs, ppos + SOUTHEAST, &mut map);
            }
            // Move Southwest (SW).
            VirtualKeyCode::B | VirtualKeyCode::Numpad1 => {
                return check_near(&gs.ecs, ppos + SOUTHWEST, &mut map);
            }

            // Cancel targeting mode.
            VirtualKeyCode::Escape => return RunState::Waiting,

            _ => return RunState::ChooseActionDir,
        },
    }
}
