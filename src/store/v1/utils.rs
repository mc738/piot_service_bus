use serde_json::{Map, Value};

pub fn get_json_object_value(obj: &Map<String, Value>, key: String) -> Option<&Map<String, Value>> {
    obj.get(&key).and_then(|v| {
        match v {
            Value::Object(o) => {
                Some(o)
            }
            _ => None
        }
    })
}


pub fn get_json_object_as_string_value(obj: &Map<String, Value>, key: String) -> Option<String> {
    obj.get(&key).and_then(|v| {
        match v {
            Value::Object(o) => {
                Some(v.to_string())
            }
            _ => None
        }
    })
}

pub fn get_json_array_value(obj: &Map<String, Value>, key: String) -> Option<&Vec<Value>> {
    obj.get(&key).and_then(|v| {
        match v {
            Value::Array(a) => {
                Some(a)
            }
            _ => None
        }
    })
}

pub fn get_json_string_value(obj: &Map<String, Value>, key: String) -> Option<&String> {
    obj.get(&key).and_then(|v| {
        match v {
            Value::String(s) => {
                Some(s)
            }
            _ => None
        }
    })
}

pub fn get_json_i64_value(obj: &Map<String, Value>, key: String) -> Option<i64> {
    obj.get(&key).and_then(|v| {
        match v {
            Value::Number(n) => {
                n.as_i64()
            }
            _ => None
        }
    })
}

pub fn get_json_f64_value(obj: &Map<String, Value>, key: String) -> Option<f64> {
    obj.get(&key).and_then(|v| {
        match v {
            Value::Number(n) => {
                n.as_f64()
            }
            _ => None
        }
    })
}

pub fn get_json_bool_value(obj: &Map<String, Value>, key: String) -> Option<&bool> {
    obj.get(&key).and_then(|v| {
        match v {
            Value::Bool(b) => {
                Some(b)
            }
            _ => None
        }
    })
}
