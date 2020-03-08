mod common;
mod blanktheme;
mod template;
mod emacs;

extern crate json;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate lazy_static;

use common::ThemeColor;
use std::str::FromStr;
use pest::Parser;
use std::collections::HashMap;
use std::fs;
use crate::blanktheme::from_json;


fn main() {
    let f = fs::read_to_string("src/themefiles/metalheart-theme.el").unwrap();
//    let fj = from_json(&f);
//    println!("{:?}", fj);
}
