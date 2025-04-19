mod internal;

use crate::internal::file_utils::{FileType, get_file_type};
use crate::internal::{file_utils, logger_config, properties_parser, yaml_parser};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

type MapOfProps = HashMap<String, String>;
type Parsable<T> = Result<T, Box<dyn Error>>;

pub fn parse_to_object<T>(path: &str) -> Result<T, Box<dyn Error>>
where
    T: for<'a> Deserialize<'a>,
{
    unsafe {
        logger_config::setup_logger();
    }
    match get_file_type(path)? {
        FileType::Properties => unimplemented!(),
        FileType::Yaml => parse_yaml(path),
    }
}

pub fn parse_properties_to_map(path: &str) -> Result<MapOfProps, Box<dyn Error>> {
    let lines = file_utils::read_file_to_vec(path)?;
    let map_of_props = properties_parser::properties_to_map(&lines)?;
    Ok(map_of_props)
}

fn parse_yaml<T>(path: &str) -> Parsable<T>
where
    T: for<'a> Deserialize<'a>,
{
    let file_content = file_utils::read_file_to_string(path)?;
    let parsed = yaml_parser::yaml_to_object(&file_content)?;
    Ok(parsed)
}
