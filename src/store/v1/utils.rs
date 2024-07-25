use serde_json::{Map, Value};

pub fn get_json_string_value(obj: Map<String, Value>) {
    obj.get()
}