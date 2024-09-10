use std::collections::HashMap;

fn parse_key_value_pairs(input: &str) -> HashMap<String, String> {
    input.split(";").filter_map(|pair| {
        let mut split = pair.split("=");
        match (split.next(), split.next()) {
            (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
            _ => None,
        }
    }).collect()
}

fn main() {
    let input = "key1=value1; key2=value2; key3=value3";
    let result = parse_key_value_pairs(input);
    for (key, value) in &result {
        println!("{}: {}", key, value);
    }
}
