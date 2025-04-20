mod internal;

use crate::internal::file_utils::{FileType, get_file_type};
use crate::internal::{file_utils, logger_config, properties_parser, yaml_parser};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

type MapOfProps = Result<HashMap<String, String>, Box<dyn Error>>;
type Parsable<T> = Result<T, Box<dyn Error>>;

pub fn load_all_yaml_configs<T: for<'a> serde::Deserialize<'a>>(
    path: &str,
    config_suffix: &str,
) -> Result<Vec<Parsable<T>>, Box<dyn Error>> {
    let extensions = FileType::Yaml.get_extensions();
    let configs = file_utils::find_file_paths(path, config_suffix, extensions)?;

    let configs: Vec<Parsable<T>> = configs
        .into_iter()
        .map(|config| parse_to_object(&config))
        .collect();

    Ok(configs)
}

pub fn parse_to_object<T>(path: &str) -> Parsable<T>
where
    T: for<'a> Deserialize<'a>,
{
    logger_config::setup_logger();
    match get_file_type(path)? {
        FileType::Properties => unimplemented!(),
        FileType::Yaml => parse_yaml(path),
    }
}

pub fn parse_properties_to_map(path: &str) -> MapOfProps {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestYaml {
        aaa: HashMap<String, HashMap<String, String>>,
        ddd: String,
        list: Vec<String>,
    }

    #[test]
    fn parse_yaml() {
        let result: TestYaml = parse_to_object("resources/test/test_input.yml").unwrap();
        println!("{:?}", result);
    }

    #[derive(Debug, Deserialize)]
    struct YamlToMap {
        aaa: String,
        bbb: HashMap<String, String>,
    }

    #[test]
    fn load_yaml_configs() {
        let configs: Vec<Parsable<YamlToMap>> =
            load_all_yaml_configs("resources/test/multiple", "_cfg").unwrap();

        println!("{:?}", configs);
        assert_eq!(configs.len(), 2);
        configs.iter().for_each(|config| assert!(config.is_ok()))
    }
}
