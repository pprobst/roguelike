use super::Renderable;
use serde::Deserialize;
//use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Mob {
    pub name: String,
    pub descr: String,
    pub mob_type: String,
    pub renderable: Option<Renderable>,
    pub fov_range: i32,
    pub blocker: bool,
    pub stats: Stats,
    pub equips: Option<Equipment>,
}

#[derive(Deserialize, Debug)]
pub struct Stats {
    pub hp: i32,
    pub max_hp: i32,
    pub attack: String,
    pub attack_range: i32,
    pub defense: i32,
}

#[derive(Deserialize, Debug)]
pub struct Equipment {
    pub weapons: Option<Vec<String>>,
    pub head: Option<Vec<String>>,
    pub torso: Option<Vec<String>>,
    pub hands: Option<Vec<String>>,
    pub legs: Option<Vec<String>>,
    pub feet: Option<Vec<String>>,
    pub back: Option<Vec<String>>,
    pub floating: Option<Vec<String>>,
}
