use palette::Srgb;
use std::str::FromStr;

pub struct ThemeMap {
    pub dark_bg: bool,
    pub fg1: Srgb,
    pub fg2: Srgb,
    pub bg1: Srgb,
    pub bg01: Srgb,
    pub bg2: Srgb,
    pub bg3: Srgb,
    pub bg4: Srgb,
    pub builtin: Srgb,
    pub keyword: Srgb,
    pub constant: Srgb,
    pub comment: Srgb,
    pub func: Srgb,
    pub typeface: Srgb,
    pub string: Srgb,
    pub warning: Srgb,
    pub warning2: Srgb,
    pub invbuiltin: Srgb,
    pub invkeyword: Srgb,
    pub invtype: Srgb,
    pub invfunc: Srgb,
    pub invstring: Srgb,
    pub invwarning: Srgb,
    pub invwarning2: Srgb,
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
             1.0 / 255.0)
        } else {
            (u8::from_str_radix(&&hex_code[1..2], 16)?,
             u8::from_str_radix(&hex_code[2..3], 16)?,
             u8::from_str_radix(&hex_code[3..4], 16)?,
             1.0 / 15.0)
        };

        let col = Srgb::new(red as f32 * factor,
                            green as f32 * factor,
                            blue as f32 * factor);

        Ok(ThemeColor { col })
    }
}


#[cfg(test)]
mod test {
    use super::*;

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
        let hex = "#gggggg";
        let tc = ThemeColor::from_str(hex);
        assert!(tc.is_err());
        let hex = "#fff";
        let tc = ThemeColor::from_str(hex);
        assert!(tc.is_ok());
    }
}
