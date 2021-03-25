use super::*;
use crate::components::{AmmoType, Ammunition, Inventory, MissileWeapon, Name, Player, TryReload};
use crate::log::Log;
use crate::utils::colors::*;

/*
 *
 * missile.rs
 * ----------
 * Resposible for managing every missile (ranged) attack performed.
 *
 */

#[system]
#[read_component(Inventory)]
#[read_component(Name)]
#[write_component(MissileWeapon)]
#[write_component(TryReload)]
#[write_component(Ammunition)]
pub fn weapon_reload(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] log: &mut Log) {
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();
    let ent_inventory = <(Entity, &Inventory)>::query();

    <(Entity, &TryReload)>::query()
        .iter(ecs)
        .for_each(|(ent, reload)| {
            commands.remove_component::<TryReload>(*ent);

            let reload_weapon = ecs.entry_ref(reload.weapon);
            if let Ok(reload_weapon) = reload_weapon {
                if let Ok(w) = reload_weapon.get_component::<MissileWeapon>() {
                    let ammo_type = &w.ammo.ammo_type;
                    ent_inventory.iter(ecs).for_each(|(e, _)| {
                        if let Ok(amm) = ecs.entry_ref(*e).unwrap().get_component::<Ammunition>() {
                            if amm.ammo_type == *ammo_type && amm.ammo > 0 {
                                match ammo_type {
                                    AmmoType::_32 => {
                                        amm.ammo -= 1;
                                        if amm.ammo == 0 {
                                            commands.remove(*e);
                                        }
                                        w.ammo.ammo += 1;
                                        if ent == &player {
                                            log.add(
                                                format!(
                                                    "You reload the {}.",
                                                    reload_weapon
                                                        .get_component::<Name>()
                                                        .unwrap()
                                                        .name,
                                                ),
                                                color("BrightWhite", 1.0),
                                            );
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    });
                }
            }
        });
}
