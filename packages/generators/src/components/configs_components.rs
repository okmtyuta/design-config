use crate::components::params::configs_components_params::ConfigsComponentsParams;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigsComponents {
    params: ConfigsComponentsParams,
}
