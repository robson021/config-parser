mod error;
mod file_utils;
mod logger_config;
mod properties_parser;
mod yaml_parser;

use crate::file_utils::{FileType, get_file_type};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

type MapOfProps = HashMap<String, String>;
type Parsable<T> = Result<T, Box<dyn Error>>;

pub fn parse_to_map<'de, T>(path: &str) -> Result<T, Box<dyn Error>> where T: Deserialize<'de>, {
    unsafe {
        logger_config::setup_logger();
    }
    match get_file_type(path)? {
        // FileType::Properties => parse_properties_to_map(path),
        FileType::Yaml => parse_yaml(path),
        _ => {
            panic!()
        }
    }
}

fn parse_properties_to_map(path: &str) -> Result<MapOfProps, Box<dyn Error>> {
    let lines = file_utils::read_file_to_vec(path)?;
    let map_of_props = properties_parser::properties_to_map(&lines)?;
    Ok(map_of_props)
}

fn parse_yaml<'de, T>(path: &str) -> Parsable<T> where T: Deserialize<'de>, {
    let file_content = file_utils::read_file_to_string(path)?;
    let parsed = yaml_parser::yaml_to_object(&file_content)?;
    Ok(parsed)
}
