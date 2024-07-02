use crate::variables::colors::configs_variables_colors::ConfigsVariablesColor;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigsVariables {
    colors: Vec<ConfigsVariablesColor>,
}
