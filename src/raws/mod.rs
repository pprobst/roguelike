use bracket_lib::prelude::{embedded_resource, link_resource, EMBED};
use serde::Deserialize;
use std::sync::Mutex;

mod rawcolors;
pub use rawcolors::*;
mod rawmaster;
pub use rawmaster::*;
mod common_structs;
pub use common_structs::*;
mod color_structs;
pub use color_structs::*;
mod item_structs;
use item_structs::*;
mod mob_structs;
pub use mob_structs::*;
mod container_structs;
pub use container_structs::*;
mod furniture_structs;
pub use furniture_structs::*;
mod spawn_structs;
pub use spawn_structs::*;

embedded_resource!(RAW_COLORS, "../../resources/raws/colors.ron");
embedded_resource!(RAW, "../../resources/raws/raws.ron");

lazy_static! {
    pub static ref COLORS: Mutex<RawColors> = Mutex::new(RawColors::empty());
    pub static ref RAWS: Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}

#[derive(Deserialize, Debug)]
pub struct Raws {
    pub items: Vec<Item>,
    pub mobs: Vec<Mob>,
    pub containers: Vec<Container>,
    pub furnitures: Vec<Furniture>,
    pub spawn_table: Vec<SpawnTable>,
}

#[derive(Deserialize, Debug)]
pub struct Colors {
    pub colorschemes: Vec<Colorscheme>,
}

pub fn load_raws() {
    link_resource!(RAW_COLORS, "resources/colors.ron");
    link_resource!(RAW, "resources/raws.ron");

    let raw_string_colors = get_raw_string("resources/colors.ron".to_string());
    let raw_string_etc = get_raw_string("resources/raws.ron".to_string());

    /*
    let full_string = [
        raw_string_items[..raw_string_items.len() - 3].to_string(),
        raw_string_colors[1..].to_string(),
    ]
    .concat();
    */

    let decoder_colors: Colors =
        ron::de::from_str(&raw_string_colors).expect("Unable to parse RON.");
    COLORS.lock().unwrap().load(decoder_colors);

    let decoder: Raws = ron::de::from_str(&raw_string_etc).expect("Unable to parse RON.");
    RAWS.lock().unwrap().load(decoder);
}

fn get_raw_string(path: String) -> &'static str {
    let raw_data = EMBED.lock().get_resource(path).unwrap();
    let raw_string =
        std::str::from_utf8(&raw_data).expect("Unable to convert to a valid UTF-8 string.");
    raw_string
}
