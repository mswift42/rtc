use palette::{Srgb, Component};
use std::str::FromStr;
use palette::rgb::{Rgb, RgbStandard};
use std::num::ParseIntError;
use std::error::Error;

pub struct ThemeMap {
    pub dark_bg: bool,
    pub fg1: ThemeColor,
    pub fg2: ThemeColor,
    pub bg1: ThemeColor,
    pub bg01: ThemeColor,
    pub bg2: ThemeColor,
    pub bg3: ThemeColor,
    pub bg4: ThemeColor,
    pub builtin: ThemeColor,
    pub keyword: ThemeColor,
    pub constant: ThemeColor,
    pub comment: ThemeColor,
    pub func: ThemeColor,
    pub typeface: ThemeColor,
    pub string: ThemeColor,
    pub warning: ThemeColor,
    pub warning2: ThemeColor,
    pub invbuiltin: ThemeColor,
    pub invkeyword: ThemeColor,
    pub invtype: ThemeColor,
    pub invfunc: ThemeColor,
    pub invstring: ThemeColor,
    pub invwarning: ThemeColor,
    pub invwarning2: ThemeColor,
}

pub struct ThemeColor {
    pub col: Srgb,
}

impl ThemeColor {
//    pub fn from_hex(scol: String) -> ThemeColor {
//    }
}

impl FromStr for ThemeColor {
    type Err = std::num::ParseIntError;

    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        let (red, green, blue, factor) = if hex_code.len() == 7 {
            (u8::from_str_radix(&hex_code[1..3], 16)?,
             u8::from_str_radix(&hex_code[3..5], 16)?,
             u8::from_str_radix(&hex_code[5..7], 16)?,
             1.0 / 255.0
            )
        } else {
            (u8::from_str_radix(&&hex_code[1..2], 16)?,
             u8::from_str_radix(&hex_code[2..3], 16)?,
             u8::from_str_radix(&hex_code[3..4], 16)?,
             1.0 / 15.0
            )
        };

        let col = Srgb::new(
            red as f32 * factor,
            green as f32 * factor,
            blue as f32 * factor,
        );

        Ok(ThemeColor { col })
    }
}

//impl<S, T> FromStr for Rgb<S,T>
//where
//    S: RgbStandard,
//    T: Component,
//{
//   type Err = std::num::ParseIntError;
//
//    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
//        let (red, green, blue, factor) = if hex_code.len() == 7 {
//            (u8::from_str_radix(&hex_code[1..3], 16)?,
//             u8::from_str_radix(&hex_code[3..5], 16)?,
//             u8::from_str_radix(&hex_code[5..7], 16)?,
//             1.0 / 255.0)
//        } else {
//            (u8::from_str_radix(&&hex_code[1..2], 16)?,
//             u8::from_str_radix(&hex_code[2..3], 16)?,
//             u8::from_str_radix(&hex_code[3..4], 16)?,
//             1.0 / 15.0)
//        };
//
//    }
//}
//

#[cfg(test)]
mod test {
    use super::*;

    extern crate approx;

    #[test]
    fn from_str() {
        let hex = "#ffffff";
        let tc = ThemeColor::from_str("#ffffff");
        assert!(tc.is_ok());
        let col = tc.unwrap().col;
        assert_eq!(col.red, 1.0);
        assert_eq!(col.green, 1.0);
        assert_eq!(col.blue, 1.0);
        let hex = "#000000";
        let tc = ThemeColor::from_str("#000000");
        let col = tc.unwrap().col;
        assert_eq!(col.red, 0.0);
        assert_eq!(col.green, 0.0);
        assert_eq!(col.blue, 0.0);
        let hex = "#fff";
        let tc = ThemeColor::from_str(hex);
        assert!(tc.is_ok());
        let col = tc.unwrap().col;
        assert_eq!(col.red, 1.0);
        assert_eq!(col.green, 1.0);
        assert_eq!(col.blue, 1.0);
        let hex = "#abc";
        let tc = ThemeColor::from_str(hex);
        let col = tc.unwrap().col;
        let (r, g, b) = (col.red, col.green, col.blue);
        assert_eq!((r * 255.0).round() as u8, 170);
        assert_eq!((g * 255.0).round() as u8, 187);
        assert_eq!((b * 255.0).round() as u8, 204);
        let hex = "#123";
        let col = ThemeColor::from_str(hex).unwrap().col;
        let (r, g, b) = (col.red, col.green, col.blue);
        assert_eq!((r * 255.0).round() as u8, 17);
        assert_eq!((g * 255.0).round() as u8, 34);
        assert_eq!((b * 255.0).round() as u8, 51);
        let hex = "hello";
        assert!(ThemeColor::from_str(hex).is_err());
    }
}
