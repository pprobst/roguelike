use super::{common_structs, Raws};
use crate::components::{
    AmmoType, Ammunition, Armor, Attack, BaseStats, Blocker, Consumable, Container, Description,
    EquipSlot, Equipable, Fov, Health, Item, MeleeWeapon, MeleeWeaponClass, MissileWeapon,
    MissileWeaponClass, Mob, Name, Position, Renderable,
};
use crate::map_gen::map::MapType;
use crate::spawner::SpawnTable;
use crate::utils::colors::color;
use bracket_lib::prelude::{parse_dice_string, to_cp437, ColorPair, RandomNumberGenerator};
use specs::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct RawMaster {
    pub raws: Raws,
    item_index: HashMap<String, usize>,
    container_index: HashMap<String, usize>,
    furniture_index: HashMap<String, usize>,
    mob_index: HashMap<String, usize>,
    spawn_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> Self {
        RawMaster {
            raws: Raws {
                items: Vec::new(),
                containers: Vec::new(),
                furnitures: Vec::new(),
                mobs: Vec::new(),
                spawn_table: Vec::new(),
            },
            item_index: HashMap::new(),
            container_index: HashMap::new(),
            furniture_index: HashMap::new(),
            mob_index: HashMap::new(),
            spawn_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;

        for (i, item) in self.raws.items.iter().enumerate() {
            self.item_index.insert(item.name.clone(), i);
        }
        for (i, container) in self.raws.containers.iter().enumerate() {
            self.container_index.insert(container.name.clone(), i);
        }
        for (i, furniture) in self.raws.furnitures.iter().enumerate() {
            self.furniture_index.insert(furniture.name.clone(), i);
        }
        for (i, mob) in self.raws.mobs.iter().enumerate() {
            self.mob_index.insert(mob.name.clone(), i);
        }
        for (i, spawn) in self.raws.spawn_table.iter().enumerate() {
            self.spawn_index.insert(spawn.name.clone(), i);
        }
    }

    pub fn get_renderable(&self, name: &str) -> &Option<common_structs::Renderable> {
        if self.item_index.contains_key(name) {
            return &self.raws.items[self.item_index[name]].renderable;
        }
        if self.mob_index.contains_key(name) {
            return &self.raws.mobs[self.mob_index[name]].renderable;
        }
        if self.container_index.contains_key(name) {
            return &self.raws.containers[self.container_index[name]].renderable;
        }
        if self.furniture_index.contains_key(name) {
            return &self.raws.furnitures[self.furniture_index[name]].renderable;
        }

        &None
    }
}

fn set_renderable(render: &common_structs::Renderable) -> Renderable {
    Renderable {
        glyph: to_cp437(render.glyph),
        color: ColorPair::new(color(&render.fg, 1.0), color(&render.bg, 1.0)),
        layer: render.layer as u8,
    }
}

pub fn get_random_possible_equips(
    name: &str,
    raws: &RawMaster,
    rng: &mut RandomNumberGenerator,
) -> Option<Vec<String>> {
    if raws.mob_index.contains_key(name) {
        let mut equips: Vec<String> = Vec::new();
        let mob = &raws.raws.mobs[raws.mob_index[name]];
        if let Some(eq) = &mob.equips {
            if let Some(wpn) = &eq.weapons {
                equips.push(rng.random_slice_entry(wpn).unwrap().to_string());
            }
            if let Some(head) = &eq.head {
                equips.push(rng.random_slice_entry(head).unwrap().to_string());
            }
            if let Some(torso) = &eq.torso {
                equips.push(rng.random_slice_entry(torso).unwrap().to_string());
            }
            if let Some(hds) = &eq.hands {
                equips.push(rng.random_slice_entry(hds).unwrap().to_string());
            }
            if let Some(lgs) = &eq.legs {
                equips.push(rng.random_slice_entry(lgs).unwrap().to_string());
            }
            if let Some(feet) = &eq.feet {
                equips.push(rng.random_slice_entry(feet).unwrap().to_string());
            }
            if let Some(bck) = &eq.back {
                equips.push(rng.random_slice_entry(bck).unwrap().to_string());
            }
            if let Some(flt) = &eq.floating {
                equips.push(rng.random_slice_entry(flt).unwrap().to_string());
            }
        }

        return Some(equips);
    }

    None
}

pub fn get_items_tier(tier: u8, raws: &RawMaster) -> Vec<String> {
    let items = &raws.raws.items;
    items
        .iter()
        .filter(|x| x.tier == tier)
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
}

pub fn get_spawn_table(level: i32, maptype: MapType, raws: &RawMaster) -> SpawnTable {
    let mut spawn_table = SpawnTable::new();

    for spawn in raws.raws.spawn_table.iter() {
        let mut insert_to_table = true;
        let mut spawn_weight = spawn.spawn_weight;
        if let Some(min_max) = spawn.min_max_level {
            if level >= min_max.0 && level <= min_max.1 {
                spawn_weight += level;
            } else {
                insert_to_table = false;
            }
        }
        if let Some(level_type) = &spawn.level_type {
            if insert_to_table {
                if !level_type.contains(&maptype.to_string()) {
                    insert_to_table = false;
                }
            }
        }
        if insert_to_table {
            spawn_table.add(spawn.name.to_string(), spawn_weight);
        }
    }

    spawn_table
}

pub fn spawn_entity(name: &str, pos: Option<Position>, entity: EntityBuilder, raws: &RawMaster) {
    if raws.mob_index.contains_key(name) {
        spawn_mob(name, pos.unwrap(), entity, raws);
    } else if raws.item_index.contains_key(name) {
        spawn_item(name, pos, entity, raws);
    } else if raws.furniture_index.contains_key(name) {
        spawn_furniture(name, pos.unwrap(), entity, raws);
    } else if raws.container_index.contains_key(name) {
        spawn_container(name, pos.unwrap(), entity, raws);
    }
}

pub fn spawn_container(
    name: &str,
    pos: Position,
    entity: EntityBuilder,
    raws: &RawMaster,
) -> Option<Entity> {
    if raws.container_index.contains_key(name) {
        let container = &raws.raws.containers[raws.container_index[name]];
        let mut ent = entity;

        ent = ent.with(Name {
            name: container.name.clone(),
        });
        ent = ent.with(Description {
            descr: container.descr.clone(),
        });
        ent = ent.with(Position { x: pos.x, y: pos.y });
        ent = ent.with(Blocker {});
        ent = ent.with(Container {
            tiers: container.tiers.clone(),
            max_items: container.max_items,
        });

        if let Some(renderable) = &container.renderable {
            ent = ent.with(set_renderable(renderable));
        }

        return Some(ent.build());
    }

    None
}

pub fn spawn_item(
    name: &str,
    position: Option<Position>,
    entity: EntityBuilder,
    raws: &RawMaster,
) -> Option<Entity> {
    if raws.item_index.contains_key(name) {
        let item = &raws.raws.items[raws.item_index[name]];
        let mut ent = entity;

        ent = ent.with(Name {
            name: item.name.clone(),
        });
        ent = ent.with(Description {
            descr: item.descr.clone(),
        });
        ent = ent.with(Item { tier: item.tier });

        if let Some(pos) = position {
            ent = ent.with(Position { x: pos.x, y: pos.y });
        }
        if let Some(renderable) = &item.renderable {
            ent = ent.with(set_renderable(renderable));
        }
        if let Some(consumable) = &item.consumable {
            for effect in consumable.effects.iter() {
                let effname = effect.0.as_str();
                match effname {
                    "heal" => {
                        ent = ent.with(Consumable { heal: *effect.1 });
                    }
                    _ => return None,
                }
            }
        }
        if let Some(equip) = &item.equipable {
            match equip.slot.as_str() {
                "weapon1" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Weapon1,
                    })
                }
                "weapon2" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Weapon2,
                    })
                }
                "head" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Head,
                    })
                }
                "torso" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Torso,
                    })
                }
                "hands" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Hands,
                    })
                }
                "legs" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Legs,
                    })
                }
                "back" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Back,
                    })
                }
                "feet" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Feet,
                    })
                }
                "floating" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Floating,
                    })
                }
                _ => return None,
            }
        }
        if let Some(melee) = &item.melee {
            if let Ok(dicetype) = parse_dice_string(&melee.damage) {
                let weapon_stats = Attack {
                    base_damage: melee.damage.to_string(),
                    dice_n: dicetype.n_dice,
                    dice_faces: dicetype.die_type,
                    dice_bonus: dicetype.bonus,
                    range: 0,
                };
                match melee.class.as_str() {
                    "dagger" => {
                        ent = ent.with(MeleeWeapon {
                            stats: weapon_stats,
                            class: MeleeWeaponClass::Dagger,
                        })
                    }
                    "axe" => {
                        ent = ent.with(MeleeWeapon {
                            stats: weapon_stats,
                            class: MeleeWeaponClass::Axe,
                        })
                    }
                    _ => return None,
                }
            }
        }
        if let Some(missile) = &item.missile {
            if let Ok(dicetype) = parse_dice_string(&missile.damage) {
                let weapon_stats = Attack {
                    base_damage: missile.damage.to_string(),
                    dice_n: dicetype.n_dice,
                    dice_faces: dicetype.die_type,
                    dice_bonus: dicetype.bonus,
                    range: 0,
                };

                match missile.class.as_str() {
                    "pistol" => {
                        ent = ent.with(MissileWeapon {
                            stats: weapon_stats,
                            class: MissileWeaponClass::Pistol,
                            ammo: Ammunition {
                                max_ammo: missile.max_ammo,
                                ammo: missile.max_ammo,
                                ammo_type: AmmoType::from_str(&missile.ammo_type).unwrap(),
                            },
                        })
                    }
                    _ => return None,
                }
            }
        }
        if let Some(ammo) = &item.ammunition {
            ent = ent.with(Ammunition {
                max_ammo: ammo.ammo,
                ammo: ammo.ammo,
                ammo_type: AmmoType::from_str(&ammo.ammo_type).unwrap(),
            })
        }
        if let Some(armor) = &item.armor {
            ent = ent.with(Armor {
                defense: armor.defense,
            })
        }

        return Some(ent.build());
    }

    None
}

pub fn spawn_furniture(
    name: &str,
    pos: Position,
    entity: EntityBuilder,
    raws: &RawMaster,
) -> Option<Entity> {
    if raws.furniture_index.contains_key(name) {
        let furniture = &raws.raws.furnitures[raws.furniture_index[name]];
        let mut ent = entity;
        ent = ent.with(Name {
            name: furniture.name.clone(),
        });
        ent = ent.with(Description {
            descr: furniture.descr.clone(),
        });
        ent = ent.with(Position { x: pos.x, y: pos.y });

        if let Some(_blocker) = &furniture.blocker {
            ent = ent.with(Blocker {});
        }

        if let Some(renderable) = &furniture.renderable {
            ent = ent.with(set_renderable(renderable));
        }

        Some(ent.build());
    }
    None
}

pub fn spawn_mob(
    name: &str,
    pos: Position,
    entity: EntityBuilder,
    raws: &RawMaster,
) -> Option<Entity> {
    if raws.mob_index.contains_key(name) {
        let mob = &raws.raws.mobs[raws.mob_index[name]];
        let mut ent = entity;

        ent = ent.with(Mob {
            mob_type: mob.mob_type.parse().unwrap(),
        });
        ent = ent.with(Name {
            name: mob.name.clone(),
        });
        ent = ent.with(Description {
            descr: mob.descr.clone(),
        });
        ent = ent.with(Position { x: pos.x, y: pos.y });
        ent = ent.with(Fov {
            range: mob.fov_range,
            dirty: true,
            visible_pos: Vec::new(),
        });
        if mob.blocker {
            ent = ent.with(Blocker {});
        }

        let mut attack_stats = Attack {
            base_damage: "1d3".to_string(),
            dice_n: 1,
            dice_faces: 3,
            dice_bonus: 1,
            range: 0,
        };

        if let Ok(dicetype) = parse_dice_string(&mob.stats.attack) {
            attack_stats.base_damage = mob.stats.attack.to_string();
            attack_stats.dice_n = dicetype.n_dice;
            attack_stats.dice_faces = dicetype.die_type;
            attack_stats.dice_bonus = dicetype.bonus;
            attack_stats.range = mob.stats.attack_range;
        }

        ent = ent.with(BaseStats {
            health: Health {
                max_hp: mob.stats.max_hp,
                hp: mob.stats.hp,
            },
            defense: mob.stats.defense,
            attack: attack_stats,
            god: false,
        });

        if let Some(renderable) = &mob.renderable {
            ent = ent.with(set_renderable(renderable));
        }

        Some(ent.build());
    }

    None
}
