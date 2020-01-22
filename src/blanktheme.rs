use std::error::Error;
use std::collections::HashMap;
use crate::common::ThemeColor;
use serde::Deserialize;
use serde_json::Result;

type ParseError = Box<dyn Error>;


#[derive(Deserialize)]
pub struct JsonTheme {
    pub background: String,
    pub foreground: String,
    pub keyword: String,
    pub function: String,
    pub comment: String,
    pub string: String,
    pub typeface: String,
    pub builtin: String,
    pub constant: String,
    pub warning: String,
    pub warning2: String,
}

pub fn from_json(cm: &str) -> Result<HashMap<&str, ThemeColor>, ParseError> {
    let parsed = json::parse(cm)?;
    let map: HashMap<&str,ThemeColor> = HashMap::new();


}