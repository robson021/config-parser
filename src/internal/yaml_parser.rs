use serde::Deserialize;

pub(crate) fn yaml_to_object<T>(s: &str) -> Result<T, serde_yaml::Error>
where
    T: for<'a> Deserialize<'a>,
{
    let parsed = serde_yaml::from_str(s)?;
    Ok(parsed)
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::collections::HashMap;

    fn get_simple_input() -> String {
        "aaa: 1.5\nbbb: 2.3\n".to_string()
    }

    fn get_complex_input() -> String {
        "aaa: 123\nbbb: 456\nnested:\n  ccc: some val\n  ddd: value 3.14 test"
            .trim()
            .to_string()
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Simple {
        aaa: String,
        bbb: String,
    }

    #[derive(Debug, Deserialize)]
    struct Complex {
        aaa: String,
        bbb: String,
        nested: HashMap<String, String>,
    }

    #[test]
    fn parse_simple_yaml_to_object() {
        let input = get_simple_input();
        let result: Simple = yaml_to_object(&input).unwrap();

        assert_eq!(result.aaa, "1.5");
        assert_eq!(result.bbb, "2.3");
    }

    #[test]
    fn parse_complex_yaml_to_object() {
        let input = get_complex_input();
        let result: Complex = yaml_to_object(&input).unwrap();

        assert_eq!(result.aaa, "123");
        assert_eq!(result.bbb, "456");
        assert_eq!(result.nested.get("ccc").unwrap(), "some val");
        assert_eq!(result.nested.get("ddd").unwrap(), "value 3.14 test");
    }
}
