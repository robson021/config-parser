use crate::error::ParserError;
use log::debug;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub enum FileType {
    Properties,
    Yaml,
}

pub fn get_file_type(path: &str) -> Result<FileType, Box<dyn Error>> {
    let is_yaml = path.ends_with(".yaml") || path.ends_with(".yml");
    let file_type = if is_yaml {
        Ok(FileType::Yaml)
    } else if path.ends_with(".properties") {
        Ok(FileType::Properties)
    } else {
        debug!("Unsupported file type");
        Err(ParserError::UnsupportedFileFormat.into())
    };

    if Path::new(&path).exists() {
        file_type
    } else {
        debug!("File not found");
        Err(Box::new(ParserError::FileNotFound))
    }
}

pub fn read_properties_to_map(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    debug!("Reading properties from: {}", path);

    let lines: Vec<String> = read_to_string(path)?.lines().map(String::from).collect();

    let props: Result<HashMap<String, String>, _> =
        lines.iter().map(|x| to_key_value_pair(x)).collect();
    props
}

#[inline]
fn to_key_value_pair(line: &str) -> Result<(String, String), Box<dyn Error>> {
    let kv = line.split("=").collect::<Vec<&str>>();
    if kv.len() != 2 {
        return Err(Box::new(ParserError::InvalidPropertiesFormat));
    }
    let key = String::from(kv[0]);
    let value = String::from(kv[1]);
    Ok((key, value))
}
