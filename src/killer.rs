use super::{
    components::{BaseStats, Inventory, Name, Player, Position},
    log::Log,
    spawner::spawn_remains,
};
use crate::utils::colors::*;
use legion::systems::CommandBuffer;
use legion::*;

/*
 *
 * killer.rs
 * ---------
 * Works as a "cleaner" by deleting the dead entities from the world.
 * Also inserts dead mob's remains (if there are items to be dropped).
 *
 */

pub struct Killer<'a> {
    pub ecs: &'a mut World,
    pub resources: &'a mut Resources,
}

/// Remove all the dead entities from the ECS.
pub fn remove_dead_entities(ecs: &mut World, resources: &mut Resources) {
    Killer { ecs, resources }.kill_all()
}

impl<'a> Killer<'a> {
    pub fn kill_all(&mut self) {
        let mut dead: Vec<(Entity, String, Position)> = Vec::new();
        {
            let mut log = self.resources.get_mut::<Log>().unwrap();

            let red = color("BrightRed", 1.0);
            let yellow = color("BrightYellow", 1.0);

            <(Entity, &BaseStats, &Name, &Position)>::query()
                .iter(self.ecs)
                .for_each(|(ent, stats, name, pos)| {
                    if stats.health.hp <= 0 {
                        if self
                            .ecs
                            .entry_ref(*ent)
                            .unwrap()
                            .get_component::<Player>()
                            .is_ok()
                        {
                            log.add("You died...", red);
                        } else {
                            log.add(format!("{} dies.", &name.name), yellow);
                            dead.push((*ent, name.name.to_string(), *pos));
                        }
                    }
                });

            let mut cb = CommandBuffer::new(&mut self.ecs);
            for f in dead {
                self.insert_remains(f.0, f.1, f.2);
                cb.remove(f.0);
            }
        }
    }

    #[allow(unused)]
    fn insert_remains(&mut self, ent: Entity, ent_name: String, ent_pos: Position) {
        let mut items: Vec<Entity> = Vec::new();

        items = <(Entity, &Inventory)>::query()
            .iter(self.ecs)
            .filter(|item| item.1.owner == ent)
            .map(|item| item.0)
            .cloned()
            .collect();

        if items.len() > 0 {
            spawn_remains(self.ecs, items, ent_name, ent_pos);
        }
    }
}
