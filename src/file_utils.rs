use crate::error::ParserError;
use log::debug;
use std::error::Error;
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
