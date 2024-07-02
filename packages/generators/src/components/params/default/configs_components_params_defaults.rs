use crate::components::params::default::default_reset_params::DefaultResetParams;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "schema")]
pub enum ConfigsComponentsParamsDefault {
    DefaultResetParams { value: DefaultResetParams },
}
