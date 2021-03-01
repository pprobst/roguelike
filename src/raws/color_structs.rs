use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Colorscheme {
    pub name: String,
    pub colors: HashMap<String, String>,
}
