use super::*;
use crate::components::{
    ActiveWeapon, BaseStats, Equipment, MissileAttack, MissileWeapon, Name, Player, SufferDamage,
};
use crate::log::Log;
use crate::utils::colors::*;
use bracket_lib::prelude::RandomNumberGenerator;

/*
 *
 * missile.rs
 * ----------
 * Resposible for managing every missile (ranged) attack performed.
 *
 */

#[system]
#[read_component(BaseStats)]
#[read_component(Equipment)]
#[read_component(ActiveWeapon)]
#[read_component(Name)]
#[write_component(MissileAttack)]
#[write_component(SufferDamage)]
#[write_component(MissileWeapon)]
pub fn missile(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] log: &mut Log,
    #[resource] rng: &mut RandomNumberGenerator,
) {
    let white = color("BrightWhite", 1.0);
    let active_missile_wpn =
        <(&ActiveWeapon, &MissileWeapon, &Equipment, &Name)>::query().filter(component::<Player>());
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

    <(Entity, &MissileAttack, &BaseStats, &Name)>::query()
        .iter(ecs)
        .for_each(|(ent, missile, attacker_stats, name)| {
            commands.remove_component::<MissileAttack>(*ent);

            let attacker_hp = attacker_stats.health.hp;
            let missile_target = ecs.entry_ref(missile.target).unwrap();
            let victim_stats = missile_target.get_component::<BaseStats>().unwrap();
            let victim_hp = victim_stats.health.hp;

            if attacker_hp > 0 && victim_hp > 0 {
                active_missile_wpn
                    .iter(ecs)
                    .for_each(|(_, missile_wpn, equip, name_wpn)| {
                        if equip.user == *ent && missile_wpn.ammo.ammo > 0 {
                            let wpn_stats = &missile_wpn.stats;
                            let total_intended_damage = rng
                                .roll_dice(wpn_stats.dice_n, wpn_stats.dice_faces)
                                + wpn_stats.dice_bonus;
                            let damage = i32::max(0, total_intended_damage - victim_stats.defense);
                            missile_wpn.ammo.ammo -= 1;
                            let victim_name = missile_target.get_component::<Name>().unwrap();
                            log.add(
                                format!(
                                    "{} shoots {} with a {} for {} hp!",
                                    &name.name, &victim_name.name, &name_wpn.name, damage
                                ),
                                white,
                            );
                            SufferDamage::add_damage(
                                &commands,
                                missile.target,
                                damage,
                                *ent == player,
                            );
                        } else {
                            if *ent == player {
                                log.add(format!("No ammo for {}.", &name_wpn.name), white);
                            }
                        }
                    });
            }
        });
}
