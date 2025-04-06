use crate::MapOfProps;
use std::error::Error;

pub fn yaml_to_map(yaml_file: &str) -> Result<MapOfProps, Box<dyn Error>> {
    let parsed: MapOfProps = serde_yaml::from_str(yaml_file)?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_simple_input() -> &'static str {
        "aaa: 1.5\nbbb: 2.3\n"
    }

    fn get_complex_input() -> &'static str {
        "aaa: 123\nbbb: 456\nnested:\n  ccc: some val\n  ddd: value 3.14 test".trim()
    }

    #[test]
    fn parse_yaml_to_map() {
        let input = get_simple_input();
        let result = yaml_to_map(input).unwrap();

        let a = result.get("aaa").unwrap();
        let b = result.get("bbb").unwrap();

        assert_eq!(a, "1.5");
        assert_eq!(b, "2.3");

        // let input = get_complex_input();
        // let result = yaml_to_map(&input).unwrap();
        // println!("{:?}", input);
        // println!("{:?}", result.get("nested").unwrap());
    }

    /*    #[test]
    fn parse_simple_yaml() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Simple {
            aaa: String,
            bbb: String,
        }

        let input = get_simple_input();

        let result: Simple = yaml_to_object(input).unwrap();
        println!("{:?}", result);

        assert!(result.aaa, "1.5");
        assert!(result.bbb, "2.3");
    }*/
}
