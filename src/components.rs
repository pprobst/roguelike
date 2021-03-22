use legion::{Entity, systems::CommandBuffer, world::*};
use crate::utils::directions::Direction;
use bracket_lib::prelude::{to_cp437, ColorPair, Point, RGB};
use std::ops::{Add, AddAssign, Sub};
use strum_macros::EnumString;
//use std::collections::HashSet;

/*
 *
 * components.rs
 * -------------
 * Contains all the possible ECS components.
 *
 */

pub type Position = Point;

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, other: Direction) {
        *self = Self {
            x: self.x + other.delta_x as i32,
            y: self.y + other.delta_y as i32,
        };
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, other: Direction) -> Self {
        Self {
            x: self.x + other.delta_x as i32,
            y: self.y + other.delta_y as i32,
        }
    }
}

impl Sub<Direction> for Point {
    type Output = Self;

    fn sub(self, other: Direction) -> Self {
        Self {
            x: self.x - other.delta_x as i32,
            y: self.y - other.delta_y as i32,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Renderable {
    pub glyph: u16,
    pub color: ColorPair,
    pub layer: u8,
}

impl Renderable {
    pub fn new(glyph: char, fg: RGB, bg: RGB) -> Self {
        Self {
            glyph: to_cp437(glyph),
            color: ColorPair::new(fg, bg),
            layer: 0,
        }
    }
}

pub struct Player {}

#[derive(EnumString, Debug)]
pub enum MobType {
    Gen,        // A "true" human. May be genetically/cybernetically modified or not.
    Savage,     // Various savages that will probably try to eat you. Can be human, but not Gen.
    Wildlife,   // Kitties, doggos, etc.
    Cacogen,    // Otherwordly species. Have a physical form.
    Threadling, // Beings that navigate between the various threads of existence. Have a physical form or not.
}

/*
impl std::str::FromStr for MobType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Gen" => Ok(MobType::Gen),
            "Savage" => Ok(MobType::Savage),
            "Wildlife" => Ok(MobType::Wildlife),
            "Cacogen" => Ok(MobType::Cacogen),
            "Threadling" => Ok(MobType::Threadling),
            _ => Err(format!("'{}' is not a valid value for MobType.", s)),
        }
    }
}
*/

// Enemies & NPCs.
pub struct Mob {
    pub mob_type: MobType,
}

#[derive(Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Debug)]
pub struct Description {
    pub descr: String,
}

pub struct InventoryCapacity {
    pub max: u8,
    pub curr: u8,
}

#[derive(PartialEq)]
// An entity's field of view (fov).
pub struct Fov {
    pub range: i32,
    pub visible_pos: Vec<Position>,
    pub dirty: bool,
}

// Entities with this component will "block" movement over them.
// After all, you can't walk over enemies (unless you're flying!).
pub struct Blocker {}

#[derive(Debug)]
pub struct Attack {
    pub base_damage: String,
    pub dice_n: i32,
    pub dice_faces: i32,
    pub dice_bonus: i32,
    pub range: i32,
}

#[derive(Debug)]
pub struct Health {
    pub max_hp: i32,
    pub hp: i32,
}

#[derive(Debug)]
pub struct BaseStats {
    pub health: Health,
    pub defense: i32,
    pub attack: Attack,
    pub god: bool, // Doesn't die
}

#[derive(Clone)]
pub struct SufferDamage {
    pub amount: Vec<(i32, bool)>,
}

impl SufferDamage {
    pub fn add_damage(
        commands: &CommandBuffer,
        victim: Entity,
        amount: i32,
        from_player: bool,
    ) {
        commands.exec_mut(move |world, _| {
            if let Some(entry) = world.entry(victim) {
                let mut dmg = if let Ok(suffering) = entry.get_component::<SufferDamage>() {
                    (*suffering).clone()
                } else {
                    SufferDamage { amount: Vec::new() }
                };

                dmg.amount.push((amount, from_player));
                entry.add_component(dmg)
            }
        });
    }
}

pub struct MeleeAttack {
    pub target: Entity,
}

pub struct MissileAttack {
    pub target: Entity,
}

#[derive(Debug)]
pub enum MeleeWeaponClass {
    Dagger,
    Sword,
    Axe,
}

pub struct MeleeWeapon {
    pub stats: Attack,
    pub class: MeleeWeaponClass,
}

#[derive(Debug, EnumString, PartialEq)]
pub enum AmmoType {
    Arrow,
    _32,
    _9mm,
}

pub struct Ammunition {
    pub max_ammo: i32,
    pub ammo: i32,
    pub ammo_type: AmmoType,
}

#[derive(Debug)]
pub enum MissileWeaponClass {
    Pistol, // includes Revolvers
    Rifle,
    Heavy,
    Grenade,
}

pub struct MissileWeapon {
    pub stats: Attack,
    pub class: MissileWeaponClass,
    pub ammo: Ammunition,
}

pub struct ActiveWeapon {}

pub struct TryReload {
    pub weapon: Entity,
}

pub struct Target {
    pub covered: bool,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EquipSlot {
    Weapon1,
    Weapon2,
    Head,
    Torso,
    Hands,
    Legs,
    Feet,
    Back,
    Floating,
}

#[derive(Debug, PartialEq)]
pub struct Equipable {
    pub slot: EquipSlot,
}

#[derive(Debug)]
pub struct Equipment {
    pub user: Entity,
    pub equip: Entity,
}

#[derive(Debug)]
pub struct TryEquip {
    pub equipment: Equipment,
}

#[derive(Debug)]
pub struct TryUnequip {
    pub equipment: Equipment,
}

#[derive(Debug)]
pub struct Armor {
    pub defense: i32,
}

#[derive(Debug)]
pub struct Item {
    pub tier: u8,
}

pub struct Consumable {
    pub heal: i32,
}

#[derive(Debug, Clone)]
pub struct CollectItem {
    pub collects: Vec<(Entity, Entity)>,
}

impl CollectItem {
    pub fn add_collect(
        commands: &CommandBuffer,
        item: Entity,
        collector: Entity,
    ) {
        commands.exec_mut(move |world, _| {
            if let Some(entry) = world.entry(collector) {
                if let Ok(collecting) = entry.get_component::<CollectItem>() {
                    collecting.collects.push((item, collector));
                } else {
                    let itm = CollectItem {
                        collects: vec![(item, collector)],
                    };
                    entry.add_component(itm);
                }
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct DropItem {
    pub dropper: Entity,
    pub item: Entity,
}

#[derive(Debug, Clone)]
pub struct ConsumeItem {
    pub target: Entity,
    pub item: Entity,
}

#[derive(Debug)]
pub struct Inventory {
    pub owner: Entity,
}

#[derive(Debug)]
pub struct SelectedItem {
    pub item: Entity,
}

#[derive(Debug)]
pub struct SelectedPosition {
    pub pos: Position,
}

#[derive(Debug)]
pub struct Remains {} // The remains of a dead mob.

#[derive(Debug)]
pub struct Container {
    pub tiers: Vec<u8>,
    pub max_items: u8,
}

pub struct Contained {
    // Similar to Inventory, but specifically for containers.
    pub container: Entity,
}
