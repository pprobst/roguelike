use super::{EquipSlot, Equipable};
pub use legion::*;

/*
 *
 * common.rs
 * ---------
 * Commonly used public functions.
 *
 */

pub fn is_weapon(ecs: &World, equip: Entity) -> bool {
    if let Ok(e) = ecs.entry_ref(equip).unwrap().get_component::<Equipable>() {
        if e.slot == EquipSlot::Weapon1 || e.slot == EquipSlot::Weapon2 {
            return true;
        }
    }
    false
}
