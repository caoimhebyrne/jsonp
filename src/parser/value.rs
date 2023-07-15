use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonValue {
    String(String),
    Number(i64),
    Boolean(bool),
    Null,

    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
