use super::*;
use crate::components::{Player, CollectItem, Contained, Inventory, InventoryCapacity, Name, Position};
use crate::log::Log;
use crate::utils::colors::*;

/*
 *
 * item_collect.rs
 * ---------------
 * Manages the acquiring of items on the map, inserting them in the player's backpack.
 *
 */

#[system]
#[read_component(Player)]
#[read_component(Name)]
#[write_component(InventoryCapacity)]
#[write_component(Position)]
#[write_component(CollectItem)]
#[write_component(Inventory)]
#[write_component(Contained)]
pub fn item_collect(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] log: &mut Log) {
    let white = color("BrightWhite", 1.0);
    let magenta = color("Magenta", 1.0);
    let mut player = <&Player>::query();
    let mut inventory_cap = <InventoryCapacity>::query().filter(component::<Player>());

    <(Entity, &CollectItem)>::query().iter.for_each(|ent, collects| {
        for c in collects.iter() {
            if inventory_cap.curr == inventory_cap.max && c.1 == *player {
                log.add(format!("Your inventory is full!"), magenta);
                break;
            } 
            commands.add_component(c.0, Inventory { owner: c.1 });
            if c.1 == *player {
                log.add(
                    format!("You pick up {}.", c.0.get_component::<Name>().unwrap().name), 
                            white,
                );
            }
            commands.remove_component::<CollectItem>(ent);
            commands.remove_component::<Position>(c.0);
            inventory_cap.curr += 1;
        }
    });
}
