use serde::Deserialize;

pub fn yaml_to_object<T>(s: String) -> Result<T, serde_yaml::Error>
where
    T: for<'a> Deserialize<'a>,
{
    let parsed = serde_yaml::from_str(&s.clone())?;
    Ok(parsed)
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use serde::Serialize;

    fn get_simple_input() -> String {
        "aaa: 1.5\nbbb: 2.3\n".to_string()
    }

    fn get_complex_input() -> &'static str {
        "aaa: 123\nbbb: 456\nnested:\n  ccc: some val\n  ddd: value 3.14 test".trim()
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Simple {
        aaa: String,
        bbb: String,
    }

    #[test]
    fn parse_yaml_to_object() {
        let input = get_simple_input();
        let result: Simple = yaml_to_object(input).unwrap();

        assert_eq!(result.aaa, "1.5");
        assert_eq!(result.bbb, "2.3");

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
