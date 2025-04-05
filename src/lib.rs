mod error;
mod file_utils;
mod logger_config;

use crate::file_utils::{FileType, get_file_type};
use std::error::Error;

pub fn parse<T>(path: &str) -> Result<&T, Box<dyn Error>> {
    unsafe {
        logger_config::setup_logger();
    }

    let props = match get_file_type(path)? {
        FileType::Properties => parse_properties::<T>(path),
        FileType::Yaml => parse_yaml::<T>(path),
    };

    todo!() // map props to struct T
}

fn parse_properties<T>(path: &str) -> Result<&T, Box<dyn Error>> {
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
