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
        let r: f32 = f32::from_str_radix(&hex_code[1..3], 16)?;
        let g: f32 = f32::from_str_radix(&hex_code[3..5], 16)?;
        let b: f32 = f32::from_str_radix(&hex_code[5..7], 16)?;

        let col = Srgb::new(r,g,b);

        Ok(ThemeColor{col})

    }
}
fn from_hex(scol: &str) -> (u8,u8, u8) {
    let format = "#{02x}{02x}{02x}";
    if scol.len() == 4 {
        let format = "#{1x}{1x}{1x}";
    }
    let (r,g,b) = scan_fmt!(scol, format, [hex u8], [hex u8], [hex u8]).unwrap();
    (r,g,b)
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_hex() {
        let hex1 = "#ffffff";
        assert_eq!(from_hex(hex1), (255,255,255));
        let hex1 = "#000000";
        assert_eq!(from_hex(hex1), (0,0,0));
        let hex1 = "#fafafa";
        assert_eq!(from_hex(hex1), (250,250,250));
    }
}
