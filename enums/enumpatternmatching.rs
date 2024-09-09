use std::collections::HashMap;
use std::fmt;

// Define the JsonValue enum
#[derive(Clone, Debug)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

// Implement Display trait for JsonValue to print values
impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(s) => write!(f, "{}", s),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                for (i, value) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(obj) => {
                write!(f, "{{ ")?;
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                }
                write!(f, " }}")
            }
        }
    }
}

fn main() {
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), JsonValue::String("John".to_string()));
    obj.insert("age".to_string(), JsonValue::Number(30.0));
    obj.insert("is_student".to_string(), JsonValue::Bool(false));

    let json = JsonValue::Object(obj);

    println!("{}", json);
}
