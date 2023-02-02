use crate::models::IndentStyle;
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub indent_style: IndentStyle,
    pub max_line_length: i32,
    pub files: Vec<String>,
}

impl Config {
    pub fn new(files: Vec<String>) -> Self {
        Config {
            indent_style: IndentStyle::Tab,
            max_line_length: 80,
            files,
        }
    }
}

impl Config {
    pub fn set_indent_style(mut self, indent_style: Option<String>) -> Self {
        self.indent_style = match indent_style {
            Some(indent_style) => IndentStyle::from_str(indent_style.as_str()).unwrap_or_default(),
            None => IndentStyle::default(),
        };

        self
    }

    pub fn set_max_line_length(mut self, max_line_length: i32) -> Self {
        self.max_line_length = max_line_length;

        self
    }
}
