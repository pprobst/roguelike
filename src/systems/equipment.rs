use crate::components::{
    ActiveWeapon, Equipable, Equipment, Inventory, InventoryCapacity, Name, TryEquip, TryUnequip,
};
use crate::log::Log;
use crate::utils::colors::*;
use specs::prelude::*;

/*
 *
 * equipment.rs
 * -------------
 * Manages equipping stuff, and unequiping if needed.
 *
 */

pub struct EquipmentSystem {}

impl<'a> System<'a> for EquipmentSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Equipment>,
        WriteExpect<'a, Log>,
        ReadStorage<'a, Equipable>,
        WriteStorage<'a, InventoryCapacity>,
        WriteStorage<'a, Inventory>,
        WriteStorage<'a, TryEquip>,
        WriteStorage<'a, TryUnequip>,
        WriteStorage<'a, ActiveWeapon>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player,
            name,
            mut equips,
            mut log,
            equipable,
            mut capacity,
            mut inventory,
            mut try_equip,
            mut try_unequip,
            mut active_wpn,
        ) = data;

        let mut inventory_cap = capacity.get_mut(*player).unwrap();
        let white = color("BrightWhite", 1.0);

        for e in try_equip.join() {
            let to_equip_slot = &equipable.get(e.equipment.equip).unwrap().slot;
            let to_equip_name = &name.get(e.equipment.equip).unwrap().name;
            let to_equip_user = e.equipment.user;
            let mut to_unequip: Vec<Entity> = Vec::new();

            // Iterate through all already equipped itens.
            for (equip, name, equipab) in (&equips, &name, &equipable).join() {
                if equipab.slot == *to_equip_slot && equip.user == to_equip_user {
                    to_unequip.push(equip.equip);
                    if equip.user == *player {
                        log.add(format!("You unequip {}.", name.name), white);
                    }
                }
            }
            for ue in to_unequip {
                if let Some(_t) = active_wpn.get(ue) {
                    active_wpn.clear();
                }
                equips.remove(ue);
                inventory
                    .insert(
                        ue,
                        Inventory {
                            owner: to_equip_user,
                        },
                    )
                    .expect("FAILED inserting item in inventory.");
                inventory_cap.curr += 1;
            }
            inventory.remove(e.equipment.equip);
            inventory_cap.curr -= 1;
            equips
                .insert(
                    e.equipment.equip,
                    Equipment {
                        user: to_equip_user,
                        equip: e.equipment.equip,
                    },
                )
                .expect("FAILED equipping item.");

            if to_equip_user == *player {
                log.add(format!("You equip {}.", to_equip_name), white);
            }
        }

        try_equip.clear();

        for e in try_unequip.join() {
            let to_unequip = e.equipment.equip;
            if let Some(_t) = active_wpn.get(to_unequip) {
                active_wpn.clear();
            }
            if let Some(_e) = equipable.get(to_unequip) {
                equips.remove(to_unequip);
                if inventory.get(to_unequip).is_none() {
                    println!("EQUIP: {:?}", to_unequip);
                    inventory
                        .insert(to_unequip, Inventory { owner: *player })
                        .expect("FAILED inserting item in inventory.");
                    inventory_cap.curr += 1;
                } else {
                    inventory.remove(to_unequip);
                }
            }
        }

        try_unequip.clear();
    }
}
