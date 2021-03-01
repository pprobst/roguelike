use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SpawnTable {
    pub name: String,
    pub spawn_weight: i32,
    pub min_max_level: Option<(i32, i32)>, // None -> Any
    pub level_type: Option<Vec<String>>,   // None -> Any
}
