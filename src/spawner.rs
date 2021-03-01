use super::{
    common::is_weapon,
    map_gen::{Map, MapType},
    raws::*,
    utils::colors::*,
    ActiveWeapon, Attack, BaseStats, Contained, Container, Description, Equipment, Fov, Health,
    Inventory, InventoryCapacity, Mob, Name, Player, Position, Remains, Renderable,
};
use bracket_lib::prelude::{to_cp437, ColorPair, Point, RandomNumberGenerator};
use specs::prelude::*;
use std::collections::HashMap;

/*
 *
 * spawner.rs
 * ----------
 * Controls basic spawning of entities and inserts them into the ECS.
 *
 */

// Some of this stuff is based on https://github.com/tylervipond/apprentice/blob/master/src/spawner.rs

const MAX_MOBS_AREA: i32 = 6;

#[derive(Debug)]
pub struct Spawn {
    pub name: String,
    pub weight: i32,
}

#[derive(Debug)]
pub struct SpawnTable {
    pub spawns: Vec<Spawn>,
    pub total_weight: i32,
}

impl SpawnTable {
    pub fn new() -> Self {
        Self {
            spawns: Vec::new(),
            total_weight: 0,
        }
    }

    pub fn add(&mut self, name: String, weight: i32) {
        self.spawns.push(Spawn { name, weight });
        self.total_weight += weight;
    }

    pub fn roll(&self, rng: &mut RandomNumberGenerator) -> String {
        if self.total_weight == 0 {
            return "None".to_string();
        }

        let mut roll = rng.range(1, self.total_weight);
        let mut idx = 0;

        while roll > 0 {
            if roll < self.spawns[idx].weight {
                return self.spawns[idx].name.to_string();
            }
            roll -= self.spawns[idx].weight;
            idx += 1;
        }

        return "None".to_string();
    }
}

fn entity_in_container(ecs: &mut World, container: Entity) -> EntityBuilder {
    ecs.create_entity().with(Contained {
        container: container,
    })
}

fn entity_with_position(ecs: &mut World, x: i32, y: i32) -> EntityBuilder {
    ecs.create_entity().with(Position { x, y })
}

pub fn create_player(ecs: &mut World) -> Entity {
    entity_with_position(ecs, 0, 0)
        .with(Renderable {
            glyph: to_cp437('@'),
            color: ColorPair::new(color("BrightWhite", 1.0), color("Background", 1.0)),
            layer: 1,
        })
        .with(Player {})
        .with(Name {
            name: "Severian".to_string(),
        })
        .with(Description {
            descr: "It is you, wanderer.".to_string(),
        })
        .with(Fov {
            range: 20,
            visible_pos: Vec::new(),
            dirty: true,
        })
        .with(BaseStats {
            health: Health { max_hp: 15, hp: 2 },
            defense: 3,
            attack: Attack {
                base_damage: "1d3".to_string(),
                dice_n: 1,
                dice_faces: 3,
                dice_bonus: 0,
                range: 0,
            },
            god: true,
        })
        .with(InventoryCapacity { curr: 0, max: 15 })
        .build()
}

pub fn equip_player(ecs: &mut World) {
    let raws = &RAWS.lock().unwrap();
    let melee_weapon = spawn_item("Tantou", None, ecs.create_entity(), raws).unwrap();
    let missile_weapon = spawn_item("Revolver", None, ecs.create_entity(), raws).unwrap();
    let armor = spawn_item("Old Leather Armor", None, ecs.create_entity(), raws).unwrap();
    let pants = spawn_item("Bombacho", None, ecs.create_entity(), raws).unwrap();
    let ammo = spawn_item(".32 Ammo", None, ecs.create_entity(), raws).unwrap();
    let cloak = spawn_item("Sagum", None, ecs.create_entity(), raws).unwrap();
    let gloves = spawn_item("Hide Gloves", None, ecs.create_entity(), raws).unwrap();
    let boots = spawn_item("Leather Boots", None, ecs.create_entity(), raws).unwrap();
    let mut equipments = ecs.write_storage::<Equipment>();
    let player_ent = ecs.fetch::<Entity>();

    equipments
        .insert(
            melee_weapon,
            Equipment {
                user: *player_ent,
                equip: melee_weapon,
            },
        )
        .expect("FAILED to equip item.");
    equipments
        .insert(
            armor,
            Equipment {
                user: *player_ent,
                equip: armor,
            },
        )
        .expect("FAILED to equip item.");
    equipments
        .insert(
            pants,
            Equipment {
                user: *player_ent,
                equip: pants,
            },
        )
        .expect("FAILED to equip item.");
    equipments
        .insert(
            cloak,
            Equipment {
                user: *player_ent,
                equip: cloak,
            },
        )
        .expect("FAILED to equip item.");
    equipments
        .insert(
            gloves,
            Equipment {
                user: *player_ent,
                equip: gloves,
            },
        )
        .expect("FAILED to equip item.");
    equipments
        .insert(
            boots,
            Equipment {
                user: *player_ent,
                equip: boots,
            },
        )
        .expect("FAILED to equip item.");
    equipments
        .insert(
            missile_weapon,
            Equipment {
                user: *player_ent,
                equip: missile_weapon,
            },
        )
        .expect("FAILED to equip item.");

    let mut active_weapon = ecs.write_storage::<ActiveWeapon>();
    active_weapon
        .insert(melee_weapon, ActiveWeapon {})
        .expect("Insert fail");

    let mut inventory = ecs.write_storage::<Inventory>();
    inventory
        .insert(ammo, Inventory { owner: *player_ent })
        .expect("FAILED to insert item in inventory.");
}

fn get_all_tiered_containers(ecs: &World) -> Vec<(Entity, Vec<u8>)> {
    let entities = ecs.entities();
    let pos = ecs.read_storage::<Position>();
    let containers = ecs.read_storage::<Container>();

    (&pos, &entities, &containers)
        .join()
        .map(|(_p, e, c)| (e, c.tiers.clone()))
        .collect()
}

fn get_all_named_mobs(ecs: &World) -> Vec<(Entity, String)> {
    let entities = ecs.entities();
    let mobs = ecs.read_storage::<Mob>();
    let names = ecs.read_storage::<Name>();

    (&mobs, &entities, &names)
        .join()
        .map(|(_c, e, n)| (e, n.name.clone()))
        .collect()
}

fn populate_containers(ecs: &mut World, raws: &RawMaster, rng: &mut RandomNumberGenerator) {
    let containers = get_all_tiered_containers(ecs);

    for c in containers {
        for tier in c.1 {
            let items = get_items_tier(tier, raws);
            if rng.range(0, 4) < 3 {
                let random_item = rng.random_slice_entry(&items).unwrap().to_string();
                spawn_item(&random_item, None, entity_in_container(ecs, c.0), raws);
            }
        }
    }
}

fn equip_mobs(ecs: &mut World, raws: &RawMaster, rng: &mut RandomNumberGenerator) {
    let mobs = get_all_named_mobs(ecs);

    for mob in mobs {
        let mut equipped = false;
        if let Some(equips) = get_random_possible_equips(&mob.1, raws, rng) {
            for equip in equips.iter() {
                if equip != "None" {
                    if let Some(e) = spawn_item(equip.as_str(), None, ecs.create_entity(), raws) {
                        let mut equipments = ecs.write_storage::<Equipment>();
                        equipments
                            .insert(
                                e,
                                Equipment {
                                    user: mob.0,
                                    equip: e,
                                },
                            )
                            .expect("FAILED to equip item.");

                        if !equipped && is_weapon(&ecs, e) {
                            let mut active_weapon = ecs.write_storage::<ActiveWeapon>();
                            active_weapon
                                .insert(e, ActiveWeapon {})
                                .expect("Insert fail");
                            equipped = true;
                        }

                        // For simplicity's sake, mobs will have a clone of the item they're
                        // equipping in their inventory, so as to make their remains' drop more
                        // generic --  this is not the case for the player. Mobs don't really
                        // have to think about inventory management, after all.
                        let mut inventory = ecs.write_storage::<Inventory>();
                        inventory
                            .insert(e, Inventory { owner: mob.0 })
                            .expect("FAILED to insert item in inventory.");
                    }
                }
            }
        }
    }
}

pub fn spawn_remains(ecs: &mut World, mut items: Vec<Entity>, ent_name: String, pos: Position) {
    // Check if there're already remains in this spot. If there are, then insert the content
    // of previous remains into the new remains.
    {
        let container = ecs.read_storage::<Container>();
        let map = ecs.fetch::<Map>();
        let idx = map.idx_pt(pos);
        let entities = ecs.entities();
        let contain = ecs.read_storage::<Contained>();

        if let Some(ents) = &map.entities[idx] {
            for ent in ents.iter() {
                if let Some(_) = container.get(*ent) {
                    for (_, e) in (&contain, &entities)
                        .join()
                        .filter(|item| item.0.container == *ent)
                    {
                        items.push(e);
                    }
                }
            }
        }
    }

    let remains = entity_with_position(ecs, pos.x, pos.y)
        .with(Renderable {
            glyph: to_cp437('▓'),
            color: ColorPair::new(color("Red", 0.6), color("Background", 1.0)),
            layer: 0,
        })
        .with(Remains {})
        .with(Container {
            tiers: vec![0],
            max_items: 15,
        })
        .with(Name {
            name: format!("Remains of {}", ent_name),
        })
        .build();

    let mut contain = ecs.write_storage::<Contained>();
    for item in items {
        contain
            .insert(item, Contained { container: remains })
            .expect("FAILED to insert item in remains.");
    }
}

pub fn get_spawn_table_for_level(level: usize, maptype: MapType, raws: &RawMaster) -> SpawnTable {
    get_spawn_table(level as i32, maptype, raws)
}

pub fn build_spawn_list(
    spawn_list: &mut Vec<(usize, String)>,
    spawn_table: &SpawnTable,
    loc: &[usize],
    is_room: bool,
    level: i32,
    rng: &mut RandomNumberGenerator,
) {
    let mut loc_size = loc.len() as i32;
    if loc_size == 0 {
        return;
    }
    //println!("loc size: {}", loc_size);
    let mut spawns: HashMap<usize, String> = HashMap::new();
    let mut spawn_locs: Vec<usize> = Vec::from(loc);
    let num_mobs = if !is_room {
        i32::min(loc_size, rng.range(level, MAX_MOBS_AREA))
    } else {
        i32::min(loc_size, rng.range(1, 6))
    };

    {
        for _i in 0..num_mobs {
            loc_size = spawn_locs.len() as i32;
            let idx = if loc_size == 1 {
                0usize
            } else {
                (rng.roll_dice(1, loc_size) - 1) as usize
            };
            let map_idx = spawn_locs[idx];
            spawns.insert(map_idx, spawn_table.roll(rng));
            spawn_locs.remove(idx);
        }
    }

    for spawn in spawns {
        spawn_list.push((spawn.0, spawn.1));
    }
}

pub fn spawn_from_list(
    ecs: &mut World,
    spawn_list: Vec<(usize, String)>,
    map: &Map,
    raws: &RawMaster,
    rng: &mut RandomNumberGenerator,
) {
    // Seek and spawn each entity in spawn_list.
    for spawn in spawn_list {
        let pos = map.idx_pos(spawn.0);
        if pos != map.spawn_point {
            let name = &spawn.1;
            spawn_entity(name, Some(pos), ecs.create_entity(), raws)
        }
    }

    // Insert items in chests.
    populate_containers(ecs, raws, rng);
    // Equip mobs with equipment.
    equip_mobs(ecs, raws, rng);
}

pub fn spawn_player(ecs: &mut World, map: &Map) {
    let player = ecs.fetch::<Entity>();
    let mut pos = ecs.write_storage::<Position>();
    let mut ppos = pos.get_mut(*player).unwrap();
    let map_pos = map.spawn_point;
    ppos.x = map_pos.x;
    ppos.y = map_pos.y;

    let mut player_pos = ecs.write_resource::<Point>();
    player_pos.x = ppos.x;
    player_pos.y = ppos.y;

    let mut fov = ecs.write_storage::<Fov>();
    let mut pfov = fov.get_mut(*player).unwrap();
    pfov.dirty = true;
}

/*
pub fn spawn_map(ecs: &mut World, map: &Map) {
    let idx = map.idx(8, 16);
    let pt = map.idx_pos(idx);
    ecs.insert(Point::new(pt.x, pt.y));
    let player = player(ecs, pt.x, pt.y);
    ecs.insert(player);
    let raws = &RAWS.lock().unwrap();

    spawn_entity(
        "Bonfire",
        Some(Position::new(pt.x + 2, pt.y)),
        ecs.create_entity(),
        raws,
    );

    spawn_entity(
        "Med-Kit",
        Some(Position::new(pt.x + 2, pt.y + 1)),
        ecs.create_entity(),
        raws,
    );

    spawn_entity(
        "Tantou",
        Some(Position::new(pt.x + 1, pt.y + 1)),
        ecs.create_entity(),
        raws,
    );

    spawn_entity(
        "Old Leather Armor",
        Some(Position::new(pt.x + 1, pt.y + 2)),
        ecs.create_entity(),
        raws,
    );

    spawn_entity(
        "Chest",
        Some(Position::new(pt.x + 3, pt.y + 1)),
        ecs.create_entity(),
        raws,
    );

    let mut rng = RandomNumberGenerator::new();

    populate_containers(ecs, raws, &mut rng);

    for _i in 0..15 {
        let x = rng.roll_dice(1, map.width - 2);
        let y = rng.roll_dice(1, map.height - 2);
        let idx = map.idx(x, y);
        if !map.tiles[idx].block {
            spawn_entity(
                "Man-Ape",
                Some(Position::new(x, y)),
                ecs.create_entity(),
                raws,
            );
        }
    }

    equip_mobs(ecs, raws, &mut rng);
}
*/
