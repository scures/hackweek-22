use serde_json::{Result as ResultJSON, Value};

pub fn parse_json(json_str: &str) -> ResultJSON<Value> {
    let value: Value = serde_json::from_str(json_str)?;
    Ok(value)
}
