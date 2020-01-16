mod common;

extern crate approx;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use common::ThemeColor;
use std::str::FromStr;
use pest::Parser;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[grammar = "blanktheme.pest"]
pub struct BlankParser;

fn main() {
    println!("Hello, world!");
    let a = ThemeColor::from_str("#ece3db");
    println!("{:?}", a.unwrap().to_hex());
    let b = ThemeColor::from_str("#fff");
    println!("{:?}", b.unwrap().to_hex());
    let c = ThemeColor::from_str("");
    println!("{}", c.err().unwrap());
    let f = fs::read_to_string("src/themefiles/blank.theme")
        .expect("could not read file.");
    let file = BlankParser::parse(Rule::file, &f)
        .expect("unsuccesful parse")
        .next().unwrap();
}
