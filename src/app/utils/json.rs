use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

pub fn to_json<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
}

pub fn from_json<T: Deserialize<'static>>(json_str: &str) -> T {
    serde_json::from_str(json_str).unwrap()
}

pub fn parse_json(json_str: &str) -> Value {
    serde_json::from_str(json_str).unwrap()
}
