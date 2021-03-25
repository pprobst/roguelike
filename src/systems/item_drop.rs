use super::*;
use crate::components::{
    ActiveWeapon, DropItem, Equipment, Inventory, InventoryCapacity, Name, Player, Position,
};
use crate::log::Log;
use crate::utils::colors::*;

/*
 *
 * item_drop.rs
 * ------------
 * Manages the dropping of items from the player's inventory.
 *
 */

#[system]
#[read_component(Name)]
#[write_component(ActiveWeapon)]
#[write_component(Equipment)]
#[write_component(InventoryCapacity)]
#[write_component(Position)]
#[write_component(DropItem)]
#[write_component(Inventory)]
pub fn item_collect(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] log: &mut Log) {
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

    <(Entity, &DropItem)>::query()
        .iter(ecs)
        .for_each(|(ent, drop)| {
            let drop_pos = ecs
                .entry_ref(drop.dropper)
                .unwrap()
                .get_component::<Position>()
                .unwrap();
            commands.add_component(drop.item, Position::new(drop_pos.x, drop_pos.y));

            if drop.dropper == player {
                if inventory_cap.curr > 0 {
                    inventory_cap.curr -= 1;
                }
                let drop_item = ecs.entry_ref(drop.item);
                if let Ok(drop_item) = drop_item {
                    if let Ok(_) = drop_item.get_component::<Equipment>() {
                        commands.remove_component::<Equipment>(drop.item);
                        if let Ok(_) = drop_item.get_component::<ActiveWeapon>() {
                            commands.remove_component::<ActiveWeapon>(drop.item);
                        }
                    }
                    log.add(
                        format!(
                            "You drop the {}",
                            drop_item.get_component::<Name>().unwrap().name
                        ),
                        white,
                    );
                }
            }
            commands.remove_component::<Inventory>(drop.item);
            commands.remove_component::<DropItem>(*ent);
        });
}
