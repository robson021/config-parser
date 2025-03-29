use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
enum ParserError {
    UnsupportedFileFormat,
    FileNotFound,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&"Unsupported file format", f)
    }
}

impl Error for ParserError {}

enum FileType {
    Properties,
    Yaml,
}

fn get_file_type(path: &str) -> Result<FileType, Box<dyn Error>> {
    let is_yaml = path.ends_with(".yaml") || path.ends_with(".yml");
    if is_yaml {
        Ok(FileType::Yaml)
    } else if path.ends_with(".properties") {
        Ok(FileType::Properties)
    } else {
        Err(ParserError::UnsupportedFileFormat.into())
    }
}

pub fn pare<T>(path: &str) -> Result<&T, Box<dyn Error>> {
    match get_file_type(path) {
        Ok(file_type) => match file_type {
            FileType::Properties => parse_properties(path),
            FileType::Yaml => {
                todo!()
            }
        },
        Err(e) => Err(e),
    }
}

fn parse_properties<T>(path: &str) -> Result<&T, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
