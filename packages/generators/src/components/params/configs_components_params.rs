use crate::components::params::default::configs_components_params_defaults::ConfigsComponentsParamsDefault;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigsComponentsParams {
    defaults: Vec<ConfigsComponentsParamsDefault>,
}
