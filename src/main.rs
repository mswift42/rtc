mod common;
#[cfg_attr(test, macro_use)]
extern crate approx;
use common::ThemeColor;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    let a = ThemeColor::from_str("#ece3db");
    println!("{:?}", a.unwrap().to_hex());
    let b = ThemeColor::from_str("#fff");
    println!("{:?}", b.unwrap().to_hex());
    let c = ThemeColor::from_str("");
    println!("{:?}", c.is_err());
}
