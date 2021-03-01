use super::Renderable;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub descr: String,
    pub tier: u8,
    pub renderable: Option<Renderable>,
    pub consumable: Option<Consumable>,
    pub equipable: Option<Equipable>,
    pub melee: Option<Melee>,
    pub missile: Option<Missile>,
    pub ammunition: Option<Ammunition>,
    pub armor: Option<Armor>,
}

#[derive(Deserialize, Debug)]
pub struct Consumable {
    pub effects: HashMap<String, i32>,
}

#[derive(Deserialize, Debug)]
pub struct Melee {
    pub damage: String,
    pub class: String,
}

#[derive(Deserialize, Debug)]
pub struct Missile {
    pub damage: String,
    pub range: i32,
    pub class: String,
    pub ammo_type: String,
    pub max_ammo: i32,
}

#[derive(Deserialize, Debug)]
pub struct Ammunition {
    pub ammo: i32,
    pub ammo_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Armor {
    pub defense: i32,
}

#[derive(Deserialize, Debug)]
pub struct Equipable {
    pub slot: String,
}
