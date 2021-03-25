use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;

pub mod ai;
pub mod consumable;
pub mod damage;
pub mod equipment;
pub mod fov;
pub mod item_collect;
pub mod item_drop;
pub mod mapping;
pub mod melee;
pub mod missile;
pub mod weapon_reload;

pub fn build_systems_scheduler() -> Schedule {
    Schedule::builder()
        // TODO
        .build()
}
