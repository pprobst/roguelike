use crate::components::{
    ActiveWeapon, DropItem, Equipment, Inventory, InventoryCapacity, Name, Position,
};
use crate::log::Log;
use crate::utils::colors::*;
use specs::prelude::*;

/*
 *
 * item_drop.rs
 * ------------
 * Manages the dropping of items from the player's inventory.
 *
 */

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, Log>,
        WriteStorage<'a, ActiveWeapon>,
        WriteStorage<'a, Equipment>,
        WriteStorage<'a, InventoryCapacity>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, DropItem>,
        WriteStorage<'a, Inventory>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player,
            name,
            mut log,
            mut active_wpn,
            mut equipment,
            mut capacity,
            mut pos,
            mut drop,
            mut inventory,
        ) = data;
        let white = color("BrightWhite", 1.0);

        let mut inventory_cap = capacity.get_mut(*player).unwrap();
        for d in drop.join() {
            let drop_pos = pos.get(d.dropper).unwrap().clone();
            pos.insert(d.item, Position::new(drop_pos.x, drop_pos.y))
                .expect("Unable to insert position");

            if d.dropper == *player {
                if inventory_cap.curr > 0 {
                    inventory_cap.curr -= 1;
                }
                if equipment.get(d.item).is_some() {
                    equipment.remove(d.item);
                    if active_wpn.get(d.item).is_some() {
                        active_wpn.clear();
                    }
                }
                log.add(
                    format!("You drop the {}", name.get(d.item).unwrap().name),
                    white,
                );
            }
            inventory.remove(d.item);
        }
        drop.clear();
    }
}
