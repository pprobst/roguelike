use super::*;
use crate::components::{Player, Fov, MeleeAttack, Mob, Position};
use crate::map_gen::Map;
use crate::state::RunState;
use bracket_lib::prelude::*;

/*
 *
 * ai.rs
 * -----
 * Manages the mobs' AIs.
 *
 */

#[system]
#[read_component(Fov)]
#[read_component(Mob)]
#[read_component(Position)]
#[read_component(Player)]
#[read_component(RunState)]
pub fn hostile_ai(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut ents = <(Entity, &Position, &Mob, &Fov)>::query();
    let player = <(&Entity, &Player)>::query()
                 .iter(ecs)
                 .find_map(|(entity, _player)| Some(*entity))
                 .unwrap();
    let player_pos = ecs.entry_ref(player).unwrap().get_component::<Position>().unwrap();
    let runstate = <&RunState>::query().iter(ecs).nth(0).unwrap();

    if *runstate != RunState::MobTurn {
        return;
    }

    ents.iter(ecs).for_each(|(entity, pos, _, fov)| {
        let d = DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
        if d < 1.2 {
            commands.push(((), MeleeAttack{
                target: player
            }));
        }
        else if fov.visible_pos.contains(&player_pos) {
                // TODO: if has missile weapon w/ ammo, first try missile attack while fleeing; else chase player.
                let mob_location = map.idx(pos.x, pos.y);
                let player_location = map.idx(player_pos.x, player_pos.y);
                let a_star = a_star_search(mob_location, player_location, map);
                if a_star.success && a_star.steps.len() > 1 {
                    // Previous position is now unblocked.
                    map.clear_blocker(pos.x, pos.y);
                    pos.x = a_star.steps[1] as i32 % map.width;
                    pos.y = a_star.steps[1] as i32 / map.width;
                    map.add_blocker(pos.x, pos.y);
                    fov.dirty = true;
                }
            }
    });
}
