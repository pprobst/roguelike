//use bracket_lib::prelude::*;
use crate::components::{BaseStats, Position, SufferDamage};
use crate::map_gen::Map;
use specs::prelude::*;

/*
 *
 * damage.rs
 * ---------
 * Manages everything regarding damage.
 *
 */

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, SufferDamage>,
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteStorage<'a, BaseStats>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut damage, entities, _player, mut stats, mut map, position) = data;

        for (damage, _ent, victim_stats, pos) in (&damage, &entities, &mut stats, &position).join()
        {
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
        }
        damage.clear();
    }
}
