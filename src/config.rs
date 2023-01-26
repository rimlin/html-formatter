use crate::models::IndentStyle;
use std::str::FromStr;

pub struct Config {
    pub indent_style: IndentStyle,
}

impl Config {
    pub fn new(indent_style: &str) -> Self {
        Config {
            indent_style: IndentStyle::from_str(indent_style).unwrap_or_default(),
        }
    }
}
