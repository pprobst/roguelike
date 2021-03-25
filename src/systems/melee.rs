use super::*;
use crate::components::{
    ActiveWeapon, BaseStats, Equipment, MeleeAttack, MeleeWeapon, Name, Player, SufferDamage,
};
use crate::log::Log;
use crate::utils::colors::*;
use bracket_lib::prelude::RandomNumberGenerator;

/*
 *
 * melee.rs
 * --------
 * Resposible for managing every melee (physical) attack performed.
 *
 */

#[system]
#[read_component(BaseStats)]
#[read_component(Equipment)]
#[read_component(ActiveWeapon)]
#[read_component(MeleeWeapon)]
#[read_component(Name)]
#[write_component(MeleeAttack)]
#[write_component(SufferDamage)]
pub fn melee(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] log: &mut Log,
    #[resource] rng: &mut RandomNumberGenerator,
) {
    let white = color("BrightWhite", 1.0);
    let active_melee_wpn =
        <(&ActiveWeapon, &MeleeWeapon, &Equipment, &Name)>::query().filter(component::<Player>());
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

    <(Entity, &MeleeAttack, &BaseStats, &Name)>::query()
        .iter(ecs)
        .for_each(|(ent, melee, attacker_stats, name)| {
            commands.remove_component::<MeleeAttack>(*ent);

            let attacker_hp = attacker_stats.health.hp;
            let melee_target = ecs.entry_ref(melee.target).unwrap();
            let victim_stats = melee_target.get_component::<BaseStats>().unwrap();
            let victim_hp = victim_stats.health.hp;
            let victim_name = melee_target.get_component::<Name>().unwrap();
            let mut has_weapon_equipped = false;

            if attacker_hp > 0 && victim_hp > 0 {
                active_melee_wpn
                    .iter(ecs)
                    .for_each(|(_, melee_wpn, equip, name_wpn)| {
                        if equip.user == *ent {
                            has_weapon_equipped = true;
                            let wpn_stats = &melee_wpn.stats;
                            let total_intended_damage = rng
                                .roll_dice(wpn_stats.dice_n, wpn_stats.dice_faces)
                                + wpn_stats.dice_bonus;
                            let damage = i32::max(0, total_intended_damage - victim_stats.defense);
                            log.add(
                                format!(
                                    "{} hits {} with {} for {} hp!",
                                    &name.name, &victim_name.name, &name_wpn.name, damage
                                ),
                                white,
                            );
                            SufferDamage::add_damage(
                                &commands,
                                melee.target,
                                damage,
                                *ent == player,
                            );
                        }
                    });
                if !has_weapon_equipped {
                    let attack = &attacker_stats.attack;
                    let total_intended_damage =
                        rng.roll_dice(attack.dice_n, attack.dice_faces) + attack.dice_bonus;
                    let damage = i32::max(0, total_intended_damage - victim_stats.defense);
                    let physical_attack_names = vec![
                        "hits",
                        "bumps into",
                        "bites",
                        "kicks",
                        "punches",
                        "harms",
                        "assaults",
                        "attacks",
                        "hurts",
                        "spanks",
                        "strikes",
                        "beats up",
                        "slams",
                        "slaps",
                        "jumps on",
                    ];
                    log.add(
                        format!(
                            "{} {} {} for {} hp!",
                            &name.name,
                            rng.random_slice_entry(&physical_attack_names).unwrap(),
                            &victim_name.name,
                            damage
                        ),
                        white,
                    );
                    SufferDamage::add_damage(&commands, melee.target, damage, *ent == player);
                }
            }
        });
}
