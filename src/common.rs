use palette::rgb::{Rgb, RgbStandard};
use palette::{Lab, Component, LinSrgb, Shade, Srgb};
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

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
    pub fn is_dark_bg(&self) -> bool {
        let (l, _, _): (f32, f32, f32) = Lab::from(self.col).into_components();
        l < 50.0
    }

    pub fn lighten(&self, factor: f32) -> ThemeColor {
        let l = Lab::from(self.col);
        let lcol = l.lighten(factor);
        ThemeColor{
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
        let (r, g, b) = self.col.into_components();
        let (r, g, b) = ((r * 255.0 + 0.5) as u8,
                         (g * 255.0 + 0.5) as u8,
                         (b * 255.0 + 0.5) as u8);
        let c = Srgb::new(r,g,b);
        format!("#{:x}", c)
    }
}


impl FromStr for ThemeColor {
    type Err = Box<dyn std::error::Error>;

    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        match hex_code.len() {
            4 | 7 => {
                let (red, green, blue, factor) = if hex_code.len() == 7 {
                    (
                        u8::from_str_radix(&hex_code[1..3], 16)?,
                        u8::from_str_radix(&hex_code[3..5], 16)?,
                        u8::from_str_radix(&hex_code[5..7], 16)?,
                        1.0 / 255.0,
                    )
                } else {
                    (
                        u8::from_str_radix(&&hex_code[1..2], 16)?,
                        u8::from_str_radix(&hex_code[2..3], 16)?,
                        u8::from_str_radix(&hex_code[3..4], 16)?,
                        1.0 / 15.0,
                    )
                };

                let col = Rgb::new(
                    red as f32 * factor,
                    green as f32 * factor,
                    blue as f32 * factor,
                );

                Ok(ThemeColor { col })
            }
            _ => Err("invalid hex_code length".into()),
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
        assert_eq!(col.red, 1.0);
        assert_eq!(col.green, 1.0);
        assert_eq!(col.blue, 1.0);
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
        assert!(ThemeColor::from_str("#ff").is_err());
        assert!(ThemeColor::from_str("").is_err());
        assert!(ThemeColor::from_str("ffffff").is_err());
        assert!(ThemeColor::from_str("#0000000").is_err());
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
        assert_eq!(ThemeColor{col: c}.to_hex(), "#ffffff");
        let c = Srgb::new(0.0, 0.0, 0.0);
        assert_eq!(ThemeColor{col: c}.to_hex(), "#000000");
        let c = Srgb::new(0.22, 0.188, 0.114);
        assert_eq!(ThemeColor{col: c}.to_hex(), "#38301d");
        let tc = ThemeColor::from_str("#abc");
        assert_eq!(tc.unwrap().to_hex(), "#aabbcc");
        let col = Srgb::new(0.196, 0.66, 0.32);
        assert_eq!(ThemeColor{col}.to_hex(), "#32a852");
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
