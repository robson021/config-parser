use crate::MapOfProps;
use crate::internal::error::ParserError;
use std::error::Error;

pub(crate) fn properties_to_map(lines: &[String]) -> MapOfProps {
    lines.iter().map(|x| to_key_value_pair(x)).collect()
}

#[inline]
fn to_key_value_pair(line: &str) -> Result<(String, String), Box<dyn Error>> {
    const KEY_VALUE_SEPARATOR: &str = "=";

    if !line.contains(KEY_VALUE_SEPARATOR) {
        return Err(Box::new(ParserError::InvalidPropertiesFormat(format!(
            "Missing key-value separator '{}' in line: '{}'",
            KEY_VALUE_SEPARATOR, line
        ))));
    }

    let kv = line.split(KEY_VALUE_SEPARATOR).collect::<Vec<&str>>();

    if kv.len() == 2 {
        let key = String::from(kv[0]);
        let value = String::from(kv[1]);
        return Ok((key, value));
    } else if kv.len() == 1 {
        let key = String::from(kv[0]);
        if line.starts_with(&key) {
            return Ok((String::from(kv[0]), String::from("")));
        }
    }
    Err(Box::new(ParserError::InvalidPropertiesFormat(format!(
        "Invalid line: '{}'",
        line
    ))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_maps_key_value_pair() {
        let input = "some.prop=value";
        let result = to_key_value_pair(input).unwrap();
        assert_eq!(result, ("some.prop".to_string(), "value".to_string()));

        let input = "some.prop=some value";
        let result = to_key_value_pair(input).unwrap();
        assert_eq!(result, ("some.prop".to_string(), "some value".to_string()));
    }

    #[test]
    fn it_fails_on_invalid_line() {
        let input = "invalid:value";
        let result = to_key_value_pair(input).is_err();
        assert!(result);
    }

    #[test]
    fn it_maps_key_an_empty_value() {
        let input = "some.key=";
        let result = to_key_value_pair(input).unwrap();
        assert_eq!(result, ("some.key".to_string(), "".to_string()));
    }

    #[test]
    fn in_maps_properties() {
        let lines = vec![
            "some.prop=value".to_string(),
            "another.property=complex value 123".to_string(),
        ];

        let result = properties_to_map(&lines).unwrap();

        let value_1 = result.get("some.prop").unwrap();
        let value_2 = result.get("another.property").unwrap();

        assert_eq!(value_1, "value");
        assert_eq!(value_2, "complex value 123");
    }
}
