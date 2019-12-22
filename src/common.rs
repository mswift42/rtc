use palette::rgb::{Rgb, RgbStandard};
use palette::{Component, Lab, LinSrgb, Shade, Srgb};
use std::error::Error;
use core::num::ParseIntError;
use core::str::FromStr;

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
    pub col: Srgb<u8>,
}

impl ThemeColor {
    pub fn is_dark_bg(&self) -> bool {
        let (l, _, _): (f32, f32, f32) = Lab::from(self.col).into_components();
        l < 50.0
    }

    pub fn lighten(&self, factor: f32) -> ThemeColor {
        let l = Lab::from(self.col);
        let lcol = l.lighten(factor);
        ThemeColor {
            col: Rgb::from(lcol),
        }
    }

    pub fn darken(&self, factor: f32) -> ThemeColor {
        let l = Lab::from(self.col);
        let dcol = l.darken(factor);
        ThemeColor {
            col: Rgb::from(dcol),
        }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:x}", self.col)
    }
}





#[derive(Debug)]
pub enum FromHexError {
    ParseIntError(ParseIntError),
    HexFormatError(&'static str), // Like "invalid hex code length" or "unexpected hex code prefix"
}
impl From<ParseIntError> for FromHexError {
    fn from(err: ParseIntError) -> FromHexError {
        FromHexError::ParseIntError(err)
    }
}

impl From<&'static str> for FromHexError {
    fn from(err: &'static str) -> FromHexError {
        FromHexError::HexFormatError(err)
    }
}







impl FromStr for ThemeColor {
    type Err = FromHexError;

    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        let hex = if hex_code.starts_with('#') {
            &hex_code[1..]
        } else {
            hex_code
        };
        match hex.len() {
            3 => {
                let red = u8::from_str_radix(&hex[..1], 16)?;
                let green = u8::from_str_radix(&hex[1..2], 16)?;
                let blue = u8::from_str_radix(&hex[2..3], 16)?;
               let col = Rgb::new(red  * 17,
               green * 17,
               blue * 17,
               ) ;
                Ok(ThemeColor{col})

            },
            6 => {
                let red = u8::from_str_radix(&hex[..2])?;
                let green = u8::from_str_radix(&hex[2..4])?;
                let blue = u8::from_str_radix(&hex[4..6])?;
                let col = Rgb::new(red, green, blue);
                Ok(ThemeColor{col})
            }
            _ => Err("invalid hex code length".into())
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    extern crate approx;

    #[test]
    fn from_str() {
        let tc = ThemeColor::from_str("#ffffff");
        assert!(tc.is_ok());
        let col = tc.unwrap().col;
        assert_relative_eq!(col, Rgb::new(255,255,255));
        let tc = ThemeColor::from_str("#000000");
        let col = tc.unwrap().col;
        assert_relative_eq!(col, Rgb::new(0,0,0));
        let hex = "#fff";
        let tc = ThemeColor::from_str(hex);
        assert!(tc.is_ok());
        assert_relative_eq!(tc.unwrap().col, Rgb::new(255,255,255));
        let hex = "#abc";
        let tc = ThemeColor::from_str(hex);
        let col = tc.unwrap().col;
        assert_eq!(col, Rgb::new(170, 187,204));
        let hex = "#123";
        let col = ThemeColor::from_str(hex).unwrap().col;
        assert_eq!(col, Rgb::new(17,34,51));
        let hex = "hello";
        assert!(ThemeColor::from_str(hex).is_err());
        assert!(ThemeColor::from_str("#ff").is_err());
        assert!(ThemeColor::from_str("").is_err());
        assert!(ThemeColor::from_str("ffffff").is_ok());
        assert!(ThemeColor::from_str("#0000000").is_err());
        let hex = "#123456";
        let tc = ThemeColor::from_str(hex).unwrap();
        assert_eq!(tc.col, Rgb::new(28,52,86));
        let tc = ThemeColor::from_str("#fff");
        assert_eq!(tc.unwrap().to_hex(), "#ffffff");
        let tc = ThemeColor::from_str("#ece3db");
        assert_relative_eq!(tc.unwrap().col, Rgb::new(236,227,219));
        let tc = ThemeColor::from_str("#3366cc");
        assert_relative_eq!(tc.unwrap().col, Rgb::new(51, 102,204));
        let tc = ThemeColor::from_str("fffffg");
        assert!(tc.is_err());
        let tc = ThemeColor::from_str("abc");
        assert_eq!(tc.unwrap().col, Rgb::new(170, 187, 204));
    }

    #[test]
    fn is_dark_bg() {
        let tc = ThemeColor::from_str("#ffffff");
        assert_eq!(tc.unwrap().is_dark_bg(), false);
        let tc = ThemeColor::from_str("#000000");
        assert_eq!(tc.unwrap().is_dark_bg(), true);
        let tc = ThemeColor::from_str("#002b36");
        assert_eq!(tc.unwrap().is_dark_bg(), true);
        let tc = ThemeColor::from_str("#506e75");
        assert_eq!(tc.unwrap().is_dark_bg(), true);
        let tc = ThemeColor::from_str("#fdf6e3");
        assert_eq!(tc.unwrap().is_dark_bg(), false);
        let tc = ThemeColor::from_str("#d2d2d2");
        assert_eq!(tc.unwrap().is_dark_bg(), false);
        let tc = ThemeColor::from_str("#932ad7");
        assert_eq!(tc.unwrap().is_dark_bg(), true);
        let tc = ThemeColor::from_str("#87e37e");
        assert_eq!(tc.unwrap().is_dark_bg(), false);
        let tc = ThemeColor::from_str("#39c52b");
        assert_eq!(tc.unwrap().is_dark_bg(), false);
    }

    #[test]
    fn to_hex() {
        let tc = ThemeColor::from_str("#000000");
        assert_eq!(tc.unwrap().to_hex(), "#000000");
        let tc = ThemeColor::from_str("#ffffff");
        assert_eq!(tc.unwrap().to_hex(), "#ffffff");
        let tc = ThemeColor::from_str("#abc123");
        assert_eq!(tc.unwrap().to_hex(), "#abc123");
        let c = Srgb::new(1.0, 1.0, 1.0);
        assert_eq!(ThemeColor { col: c }.to_hex(), "#ffffff");
        let c = Srgb::new(0.0, 0.0, 0.0);
        assert_eq!(ThemeColor { col: c }.to_hex(), "#000000");
        let c = Srgb::new(0.22, 0.188, 0.114);
        assert_eq!(ThemeColor { col: c }.to_hex(), "#38301d");
        let tc = ThemeColor::from_str("#abc");
        assert_eq!(tc.unwrap().to_hex(), "#aabbcc");
        let col = Srgb::new(0.196, 0.66, 0.32);
        assert_eq!(ThemeColor { col }.to_hex(), "#32a852");
    }

    #[test]
    fn darken() {
        let tc = ThemeColor::from_str("#ffffff");
        assert_eq!(tc.unwrap().darken(0.1).to_hex(), "#e2e2e2");
        let tc = ThemeColor::from_str("#f3f3f3");
        assert_eq!(tc.unwrap().darken(0.1).to_hex(), "#d7d7d7");
        let tc = ThemeColor::from_str("#87e37e");
        assert_eq!(tc.unwrap().darken(0.1).to_hex(), "#6bc764");
        let tc = ThemeColor::from_str("#fdf6e3");
        assert_eq!(tc.unwrap().darken(0.1).to_hex(), "#e0dac7");
    }

    #[test]
    fn lighten() {
        let tc = ThemeColor::from_str("#000000");
        let lighten = tc.unwrap().lighten(0.1);
        assert_eq!(lighten.to_hex(), "#1b1b1b");
        let tc = ThemeColor::from_str("#932ad7");
        let lighten = tc.unwrap().lighten(0.1);
        assert_eq!(lighten.to_hex(), "#b049f4");
    }
}
