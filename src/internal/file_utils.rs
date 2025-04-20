use crate::internal::error::ParserError;
use log::{debug, error};
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use walkdir::WalkDir;

#[derive(PartialEq, Debug)]
pub(crate) enum FileType {
    Properties,
    Yaml,
}

impl FileType {
    pub(crate) fn get_extensions(&self) -> Vec<&str> {
        match self {
            FileType::Properties => vec!["properties"],
            FileType::Yaml => vec!["yaml", "yml"],
        }
    }
}

pub(crate) fn find_file_paths(
    root: &str,
    contained_substring: &str,
    extensions: Vec<&str>,
) -> Result<Vec<String>, Box<dyn Error>> {
    verify_if_path_exists(root)?;

    let config_files: Vec<String> = WalkDir::new(root)
        .into_iter()
        .flat_map(|entry_result| entry_result.ok())
        .flat_map(|entry| entry.metadata().ok().map(|meta| (meta, entry)))
        .filter(|(meta, __)| meta.is_file())
        .map(|(_, entry)| entry.path().to_string_lossy().to_string())
        .filter(|path| path.contains(contained_substring))
        .collect();

    if extensions.is_empty() {
        return Ok(config_files);
    }

    let cfg_files_with_extension = config_files
        .iter()
        .filter(|file| extensions.iter().any(|ext| file.ends_with(ext)))
        .map(|x| x.to_owned())
        .collect();
    Ok(cfg_files_with_extension)
}

fn verify_if_path_exists(path: &str) -> Result<(), Box<dyn Error>> {
    if Path::new(&path).exists() {
        debug!("Path exists: {}", path);
        Ok(())
    } else {
        error!("Path not found: {}", path);
        Err(Box::new(ParserError::FileNotFound(path.to_string())))
    }
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

    verify_if_path_exists(path)?;
    file_type
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
    use crate::internal::file_utils::{FileType, find_file_paths, get_file_type};

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
    fn valid_yaml_file_exists() {
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

    #[test]
    fn find_all_test_files() {
        let files = find_file_paths("resources/test", "_input", vec![]).unwrap();
        assert_eq!(files.len(), 3);

        let expected_1 = &"resources/test/test_input.yml".to_string();
        let expected_2 = &"resources/test/test2_input.yaml".to_string();
        let expected_3 = &"resources/test/test_input.properties".to_string();
        assert!(files.contains(expected_1));
        assert!(files.contains(expected_2));
        assert!(files.contains(expected_3));
    }

    #[test]
    fn find_yaml_files_only() {
        let files = find_file_paths("resources/test", "_input", vec!["yml", "yaml"]).unwrap();
        assert_eq!(files.len(), 2);

        let expected_1 = &"resources/test/test_input.yml".to_string();
        let expected_2 = &"resources/test/test2_input.yaml".to_string();
        assert!(files.contains(expected_1));
        assert!(files.contains(expected_2));
    }
}
