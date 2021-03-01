use super::Renderable;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Furniture {
    pub name: String,
    pub descr: String,
    pub blocker: Option<bool>,
    pub renderable: Option<Renderable>,
}
