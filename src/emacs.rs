#[derive(Debug)]
pub struct EmacsTheme<'a> {
    pub background: &'a str,
    pub foreground: &'a str,
    pub keyword: &'a str,
    pub function: &'a str,
    pub comment: &'a str,
    pub string: &'a str,
    pub typeface: &'a str,
    pub builtin: &'a str,
    pub variable: &'a str,
    pub constant: &'a str,
    pub warning: &'a str,
    pub warning2: &'a str,
}
