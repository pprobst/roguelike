use super::*;
use crate::components::{Blocker, Position, Remains};
use crate::map_gen::Map;

/*
 *
 * mapping.rs
 * ----------
 * Responsible for managing the entities on the map tiles.
 *
 */

#[system]
#[read_component(Position)]
#[read_component(Blocker)]
#[read_component(Remains)]
pub fn mapping(ecs: &SubWorld, #[resource] map: &mut Map) {
    map.refresh_entities();

    <(Entity, &Position, &Blocker)>::query()
        .iter(ecs)
        .for_each(|(ent, pos, _)| {
            map.add_blocker(pos.x, pos.y);
            let i = map.idx(pos.x, pos.y);
            map.add_entity(ent.clone(), i);
        });

    <(Entity, &Position, &Remains)>::query()
        .iter(ecs)
        .for_each(|(ent, pos, _)| {
            let i = map.idx(pos.x, pos.y);
            map.add_entity(ent.clone(), i);
        });
}
