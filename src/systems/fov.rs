use super::*;
use crate::components::{Fov, Player, Position};
use crate::map_gen::Map;
use bracket_lib::prelude::*;

/*
 *
 * fov.rs
 * ------
 * Manages the entities field-of-view (FOV, "what we/they see").
 *
 */

// See: https://github.com/thebracket/bracket-lib/blob/master/rltk/examples/ex04-fov.rs

#[system]
#[read_component(Position)]
#[read_component(Player)]
#[write_component(Fov)]
pub fn fov(ecs: &mut SubWorld, #[resource] map: &Map) {
    <(Entity, &Position, &mut Fov)>::query()
        .iter_mut(ecs)
        .filter(|(ent, _, fov)| fov.dirty)
        .for_each(|(ent, pos, mut fov)| {
            fov.dirty = false;
            fov.visible_pos.clear();
            fov.visible_pos = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
            fov.visible_pos.retain(|p| map.in_map_bounds(*p));
            if ecs
                .entry_ref(*ent)
                .unwrap()
                .get_component::<Player>()
                .is_ok()
            {
                // Reset visible tiles in map.tiles.
                for tile in map.tiles.iter_mut() {
                    tile.visible = false
                }
                // For each visible position (point) in visible_pos, mark the same position
                // as visible and revealed in the map tiles.
                for pos in fov.visible_pos.iter() {
                    let idx = map.idx(pos.x, pos.y);
                    map.tiles[idx].visible = true;
                    map.tiles[idx].revealed = true;
                }
            }
        });
}
