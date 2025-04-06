mod error;
mod file_utils;
mod logger_config;
mod properties_parser;
mod yaml_parser;

use crate::file_utils::{get_file_type, FileType};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

trait Deserializable<T>
where
    for<'de> T: Deserialize<'de>,
{
}

type MapOfProps = HashMap<String, String>;

pub fn parse_to_map<T>(path: &str) -> Result<MapOfProps, Box<dyn Error>> {
    unsafe {
        logger_config::setup_logger();
    }
    match get_file_type(path)? {
        FileType::Properties => parse_properties_to_map(path),
        FileType::Yaml => parse_yaml(path),
    }
}

fn parse_properties_to_map(path: &str) -> Result<MapOfProps, Box<dyn Error>> {
    let lines = file_utils::read_file_to_vec(path)?;
    let map_of_props = properties_parser::properties_to_map(&lines)?;
    Ok(map_of_props)
}

fn parse_yaml(path: &str) -> Result<MapOfProps, Box<dyn Error>> {
    let file_content = file_utils::read_file_to_string(path)?;
    let parsed = yaml_parser::yaml_to_map(&file_content)?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
