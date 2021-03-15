use super::*;
use crate::components::{Player, BaseStats, Consumable, ConsumeItem, Inventory, InventoryCapacity, Name};
use crate::log::Log;
use crate::utils::colors::*;

/*
 *
 * consumable.rs
 * -------------
 * Manages the consuming (food, potions, etc.) of items from the player's inventory.
 *
 */

#[system]
#[read_component(Name)]
#[read_component(Player)]
#[read_component(Consumable)]
#[write_component(InventoryCapacity)]
#[write_component(ConsumeItem)]
#[write_component(BaseStats)]
#[write_component(Inventory)]
pub fn consumable(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] log: &mut Log) {
    let white = color("BrightWhite", 1.0);
    let mut inventory_cap = <InventoryCapacity>::query().filter(component::<Player>());

    <(Entity, &ConsumeItem)>::query().iter(ecs).for_each(|ent, consume| {
        let mut has_consumed = false;
        
        if let Some(item) = consume.item.get_component::<Consumable>() {
            let mut target_stats = consume.target.get_component::<BaseStats>().unwrap();
            target_stats.health.hp = i32::min(
                target_stats.health.max_hp,
                target_stats.health.hp + item.heal,
            );

            if ecs.entry_ref(*consume.target).unwrap().get_component::<Player>().is_ok() {
                log.add(
                    format!(
                        "You consume the {}, healing {} hp.",
                        consume.item.get_component::<Name>().unwrap().name,
                        item.heal
                    ),
                    white,
                );
            }
            has_consumed = true;
        }

        if has_consumed {
            commands.remove(consume.item);
            inventory_cap.curr -= 1;
        }

        commands.remove_component::<ConsumeItem>(ent);
    });
}
