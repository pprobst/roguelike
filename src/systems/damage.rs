use super::*;
use crate::components::{Player, BaseStats, Position, SufferDamage};
use crate::map_gen::Map;

/*
 *
 * damage.rs
 * ---------
 * Manages everything regarding damage.
 *
 */

#[system]
#[read_component(Position)]
#[read_component(Player)]
#[write_component(SufferDamage)]
#[write_component(BaseStats)]
pub fn damage(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    <(Entity, &mut SufferDamage, &mut BaseStats, &Position)>::query().iter_mut(ecs).for_each(|(ent, damage, victim_stats, pos)| {
        if !victim_stats.god {
            for dmg in damage.amount.iter() {
                //println!("{}", victim_stats.health.hp);
                victim_stats.health.hp -= dmg.0;
            }
        }
        // Victim is dead, so clear blocker.
        if victim_stats.health.hp <= 0 {
            map.clear_blocker(pos.x, pos.y);
        }
        damage.amount.clear();
        commands.remove_component::<SufferDamage>(*ent);
    });
}
