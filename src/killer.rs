use super::{log::Log, spawner::spawn_remains, BaseStats, Inventory, Name, Player, Position};
use crate::utils::colors::*;
use specs::prelude::*;

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
}

/// Remove all the dead entities from the ECS.
pub fn remove_dead_entities(ecs: &mut World) {
    Killer { ecs }.kill_all()
}

impl<'a> Killer<'a> {
    pub fn kill_all(&mut self) {
        let mut dead: Vec<(Entity, String, Position)> = Vec::new();
        {
            let entities = self.ecs.entities();
            let stats = self.ecs.read_storage::<BaseStats>();
            let names = self.ecs.read_storage::<Name>();
            let positions = self.ecs.read_storage::<Position>();
            let player = self.ecs.read_storage::<Player>();
            let mut log = self.ecs.fetch_mut::<Log>();

            let red = color("BrightRed", 1.0);
            let yellow = color("BrightYellow", 1.0);

            for (ent, stats, name, pos) in (&entities, &stats, &names, &positions).join() {
                if stats.health.hp <= 0 {
                    let p: Option<&Player> = player.get(ent);
                    if let Some(_p) = p {
                        log.add("You died...", red);
                    } else {
                        log.add(format!("{} dies.", &name.name), yellow);
                        dead.push((ent, name.name.to_string(), *pos));
                    }
                }
            }
        }
        for f in dead {
            self.insert_remains(f.0, f.1, f.2);
            self.ecs
                .delete_entity(f.0)
                .expect("Unable to remove the dead");
        }
    }

    #[allow(unused)]
    fn insert_remains(&mut self, ent: Entity, ent_name: String, ent_pos: Position) {
        let mut items: Vec<Entity> = Vec::new();
        {
            let inventory = self.ecs.read_storage::<Inventory>();
            let entities = self.ecs.entities();

            items = (&inventory, &entities)
                .join()
                .filter(|item| item.0.owner == ent)
                .map(|item| item.1)
                .collect::<Vec<_>>();
        }

        if items.len() > 0 {
            spawn_remains(self.ecs, items, ent_name, ent_pos);
        }
    }
}
