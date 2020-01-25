use std::error::Error;
use std::collections::HashMap;
use crate::common::ThemeColor;
use core::str::FromStr;
use serde::Deserialize;
use serde_json::Result;

type ParseError = Box<dyn Error>;


#[derive(Deserialize, Debug)]
pub struct JsonTheme {
    pub background: String,
    pub foreground: String,
    pub keyword: String,
    pub function: String,
    pub comment: String,
    pub string: String,
    #[serde(rename = "type")]
    pub typeface: String,
    pub builtin: String,
    pub constant: String,
    pub warning: String,
    pub warning2: String,
}

pub fn from_json(cm: &str) -> HashMap<&str, String> {
    let parsed: JsonTheme = serde_json::from_str(cm).unwrap();
    let mut map: HashMap<&str,String> = HashMap::new();
    map.insert("background", parsed.background);
    map.insert("foreground", parsed.foreground);
    map.insert("keyword", parsed.keyword);
    map.insert("function", parsed.function);
    map.insert("comment", parsed.comment);
    map.insert("string", parsed.string);
    map.insert("typeface", parsed.typeface);
    map.insert("builtin", parsed.builtin);
    map.insert("constant", parsed.constant);
    map.insert("warning", parsed.warning);
    map.insert("warning2", parsed.warning2);
    map
}