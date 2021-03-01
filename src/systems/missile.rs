use crate::components::{
    ActiveWeapon, BaseStats, Equipment, MissileAttack, MissileWeapon, Name, SufferDamage,
};
use crate::log::Log;
use crate::utils::colors::*;
use bracket_lib::prelude::RandomNumberGenerator;
use specs::prelude::*;

/*
 *
 * missile.rs
 * ----------
 * Resposible for managing every missile (ranged) attack performed.
 *
 */

pub struct MissileSystem {}

impl<'a> System<'a> for MissileSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, BaseStats>,
        ReadStorage<'a, Equipment>,
        ReadStorage<'a, ActiveWeapon>,
        WriteStorage<'a, MissileAttack>,
        WriteStorage<'a, SufferDamage>,
        WriteStorage<'a, MissileWeapon>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Log>,
        WriteExpect<'a, RandomNumberGenerator>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            base_stats,
            equipment,
            active_wpn,
            mut missile_attack,
            mut do_damage,
            mut missile_wpns,
            player,
            mut log,
            mut rng,
            names,
        ) = data;
        let white = color("BrightWhite", 1.0);

        for (entity, missile, attacker_stats, name) in
            (&entities, &missile_attack, &base_stats, &names).join()
        {
            let attacker_hp = attacker_stats.health.hp;
            let victim_stats = base_stats.get(missile.target).unwrap();
            let victim_hp = victim_stats.health.hp;

            if attacker_hp > 0 && victim_hp > 0 {
                for (_active_wpn, missile_wpn, equip, name_wpn) in
                    (&active_wpn, &mut missile_wpns, &equipment, &names).join()
                {
                    if equip.user == entity && missile_wpn.ammo.ammo > 0 {
                        let wpn_stats = &missile_wpn.stats;
                        let total_intended_damage = rng
                            .roll_dice(wpn_stats.dice_n, wpn_stats.dice_faces)
                            + wpn_stats.dice_bonus;
                        let damage = i32::max(0, total_intended_damage - victim_stats.defense);
                        missile_wpn.ammo.ammo -= 1;
                        let victim_name = names.get(missile.target).unwrap();
                        log.add(
                            format!(
                                "{} shoots {} with a {} for {} hp!",
                                &name.name, &victim_name.name, &name_wpn.name, damage
                            ),
                            white,
                        );
                        SufferDamage::add_damage(
                            &mut do_damage,
                            missile.target,
                            damage,
                            entity == *player,
                        );
                        break;
                    } else {
                        if entity == *player {
                            log.add(format!("No ammo for {}.", &name_wpn.name), white);
                        }
                    }
                }
            }
        }
        missile_attack.clear();
    }
}
