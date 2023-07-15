use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonValue {
    String(String),
    Number(i64),

    Object(HashMap<String, JsonValue>),
}
