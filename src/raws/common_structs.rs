use serde::Deserialize;
//use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Renderable {
    pub glyph: char,
    pub fg: String,
    pub bg: String,
    pub layer: i32,
}
