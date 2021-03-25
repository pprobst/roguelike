use super::*;
use crate::components::{
    ActiveWeapon, Equipable, Equipment, Inventory, InventoryCapacity, Name, Player, TryEquip,
    TryUnequip,
};
use crate::log::Log;
use crate::utils::colors::*;

/*
 *
 * equipment.rs
 * -------------
 * Manages equipping stuff, and unequiping if needed.
 *
 */

#[system]
#[read_component(Name)]
#[read_component(Equipable)]
#[write_component(Equipment)]
#[write_component(InventoryCapacity)]
#[write_component(Inventory)]
#[write_component(TryEquip)]
#[write_component(TryUnequip)]
#[write_component(ActiveWeapon)]
pub fn equipment(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] log: &mut Log) {
    let white = color("BrightWhite", 1.0);
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();
    let mut inventory_cap = <&InventoryCapacity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();
    let already_equipped = <(&Equipment, &Name, &Equipable)>::query();

    <(Entity, &TryEquip)>::query()
        .iter(ecs)
        .for_each(|(ent, e)| {
            commands.remove_component::<TryEquip>(*ent);

            let e_equip = ecs.entry_ref(e.equipment.equip).unwrap();
            let to_equip_slot = e_equip.get_component::<Equipable>().unwrap().slot;
            let to_equip_name = e_equip.get_component::<Name>().unwrap().name;
            let to_equip_user = e.equipment.user;
            let mut to_unequip: Vec<Entity> = Vec::new();

            already_equipped
                .iter(ecs)
                .for_each(|(equip, name, equipab)| {
                    if equipab.slot == to_equip_slot && equip.user == to_equip_user {
                        to_unequip.push(equip.equip);
                        if equip.user == player {
                            log.add(format!("You unequip {}.", name.name), white);
                        }
                    }
                });

            for ue in to_unequip {
                let unequip_equip = ecs.entry_ref(ue).unwrap();
                if let Ok(_t) = unequip_equip.get_component::<ActiveWeapon>() {
                    commands.remove_component::<ActiveWeapon>(ue);
                }
                commands.remove_component::<Equipment>(ue);
                commands.add_component(
                    ue,
                    Inventory {
                        owner: to_equip_user,
                    },
                );
                inventory_cap.curr += 1;
            }
            commands.remove_component::<Inventory>(e.equipment.equip);
            inventory_cap.curr -= 1;
            commands.add_component(
                e.equipment.equip,
                Equipment {
                    user: to_equip_user,
                    equip: e.equipment.equip,
                },
            );

            if to_equip_user == player {
                log.add(format!("You equip {}.", to_equip_name), white);
            }
        });

    <(Entity, &TryUnequip)>::query()
        .iter(ecs)
        .for_each(|(ent, e)| {
            commands.remove_component::<TryUnequip>(*ent);

            let to_unequip = ecs.entry_ref(e.equipment.equip);
            if let Ok(to_unequip) = to_unequip {
                if let Ok(_) = to_unequip.get_component::<ActiveWeapon>() {
                    commands.remove_component::<ActiveWeapon>(e.equipment.equip);
                }
                if let Ok(_) = to_unequip.get_component::<Equipable>() {
                    commands.remove_component::<Equipment>(e.equipment.equip);
                    if let Err(_) = to_unequip.get_component::<Inventory>() {
                        // review this
                        println!("EQUIP: {:?}", e.equipment.equip);
                        commands.add_component(e.equipment.equip, Inventory { owner: player });
                        inventory_cap.curr += 1;
                    } else {
                        commands.remove_component::<Inventory>(e.equipment.equip);
                    }
                }
            }
        });
}
