use std::error::Error;
use std::collections::HashMap;
use crate::common::ThemeColor;
use core::str::FromStr;
use serde::Deserialize;
use serde_json::Result;
use json::parse;

type ParseResult<'a> = serde_json::Result<HashMap<&'a str, String>>;


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

pub fn from_json(cm: &str) -> ParseResult {
    let parsed: JsonTheme = serde_json::from_str(cm)?;
    let mut map: HashMap<&str, String> = HashMap::new();
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
    Ok(map)
}

pub fn failed_colors(hm: HashMap<&str, String>) -> Vec<String> {
    hm.keys().filter(|i| parse_color(*i).is_none())
        .map(|i| (*i).to_string())
        .collect()
}

pub fn succeeded_colors(hm: HashMap<&str, String>) -> Vec<String> {
    hm.keys().filter(|i| parse_color(*i).is_some())
        .map(|i| (*i).to_string())
        .collect()
}

fn parse_color(hex: &str) -> Option<ThemeColor> {
    match ThemeColor::from_str(hex) {
        Err(_) => None,
        Ok(col) => Some(col)
    }
}