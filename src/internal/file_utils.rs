use crate::internal::error::ParserError;
use log::{debug, error};
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(PartialEq, Debug)]
pub(crate) enum FileType {
    Properties,
    Yaml,
}

pub(crate) fn get_file_type(path: &str) -> Result<FileType, Box<dyn Error>> {
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
        debug!("File found: {}", path);
        file_type
    } else {
        error!("File not found: {}", path);
        Err(Box::new(ParserError::FileNotFound(path.to_string())))
    }
}

pub(crate) fn read_file_to_string(path: &str) -> Result<String, Box<dyn Error>> {
    let file = read_to_string(path)?;
    Ok(file)
}

pub(crate) fn read_file_to_vec(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let lines: Vec<String> = read_to_string(path)?.lines().map(String::from).collect();
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use crate::internal::file_utils::{FileType, get_file_type};
    use std::path::PathBuf;
    use std::sync::Once;

    fn setup_test_resources() {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push("resources/test");
            println!("Test resources: {}", d.display());
        });
    }

    #[test]
    fn valid_aml_file_exists() {
        setup_test_resources();
        let file_type = get_file_type("resources/test/test_input.yml").unwrap();
        assert_eq!(file_type, FileType::Yaml)
    }

    #[test]
    fn valid_properties_file_exists() {
        setup_test_resources();
        let file_type = get_file_type("resources/test/test_input.properties").unwrap();
        assert_eq!(file_type, FileType::Properties)
    }

    #[test]
    fn valid_extension_but_file_does_not_exists() {
        let error = get_file_type("invalid/path/file.yaml").unwrap_err();
        assert_eq!(error.to_string(), "File not found: invalid/path/file.yaml");

        let error = get_file_type("invalid/path/file.yml").unwrap_err();
        assert_eq!(error.to_string(), "File not found: invalid/path/file.yml");

        let error = get_file_type("invalid/path/file.properties").unwrap_err();
        assert_eq!(
            error.to_string(),
            "File not found: invalid/path/file.properties"
        );
    }
}
