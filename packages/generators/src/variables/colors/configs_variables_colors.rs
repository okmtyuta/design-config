use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "schema")]
pub enum ConfigsVariablesColor {
    UniversalColor { value: UniversalColor },
}

#[derive(Deserialize, Debug, Clone)]
pub struct UniversalColor {
    name: String,
    value: String,
}
