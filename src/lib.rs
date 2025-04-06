mod error;
mod file_utils;
mod logger_config;
mod properties_parser;

use crate::file_utils::{get_file_type, FileType};
use std::error::Error;

pub fn parse<T>(path: &str) -> Result<&T, Box<dyn Error>> {
    unsafe {
        logger_config::setup_logger();
    }
    match get_file_type(path)? {
        FileType::Properties => parse_properties::<T>(path),
        FileType::Yaml => parse_yaml::<T>(path),
    }
}

fn parse_properties<T>(path: &str) -> Result<&T, Box<dyn Error>> {
    let lines = file_utils::read_file_to_string(path)?;
    let map_of_props = properties_parser::properties_to_map(&lines)?;

    todo!()
}
fn parse_yaml<T>(path: &str) -> Result<&T, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
