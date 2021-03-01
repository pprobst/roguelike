use super::{EquipSlot, Equipable};
use specs::prelude::*;

/*
 *
 * common.rs
 * ---------
 * Commonly used public functions.
 *
 */

pub fn is_weapon(ecs: &World, equip: Entity) -> bool {
    let equipable = ecs.read_storage::<Equipable>();

    if let Some(e) = equipable.get(equip) {
        if e.slot == EquipSlot::Weapon1 || e.slot == EquipSlot::Weapon2 {
            return true;
        }
    }

    false
}
