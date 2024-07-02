use crate::components::configs_components::ConfigsComponents;
use crate::variables::configs_variables::ConfigsVariables;
use serde::Deserialize;
use serde_json::from_reader;
use std::{fs::File, io::BufReader, path::PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct Configs {
    components: ConfigsComponents,
    variables: ConfigsVariables,
}

impl Configs {
    pub fn from_file(path: PathBuf) -> Configs {
        let file = File::open(path).unwrap();

        let reader: BufReader<File> = BufReader::new(file);

        let configs = from_reader::<BufReader<File>, Configs>(reader).unwrap();

        return configs;
    }
}
