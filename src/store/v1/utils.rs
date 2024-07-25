use serde_json::{Map, Value};

pub fn get_json_string_value(obj: Map<String, Value>, key: String) -> Option<String> {
    obj.get(&key).and_then(|v| {
        match v {
            Value::String(s) => {
                Some(s.clone())
            }
            _ => None
        }
    })
}