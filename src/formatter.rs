use crate::{config::Config, models::*};
use std::fmt::Write;

pub struct Formatter<'a> {
    tokens: Vec<LexerToken>,
    config: &'a Config,
    indent_count: i32,
}

impl<'a> Formatter<'a> {
    pub fn new(tokens: Vec<LexerToken>, config: &'a Config) -> Self {
        Self {
            tokens,
            config,
            indent_count: 0,
        }
    }

    pub fn format(&mut self) -> String {
        let mut content: String = "".to_owned();

        for token in self.tokens.iter().cloned() {
            let result = match token.clone() {
                LexerToken::StartTag(start_tag) => self.format_start_tag(start_tag),
                LexerToken::Chars(chars) => self.format_chars(chars),
                LexerToken::EndTag(end_tag) => {
                    self.indent_count -= 1;

                    self.format_end_tag(end_tag)
                }
            };

            write!(
                content,
                "{}{}\n",
                self.config.indent_style.repeat(self.indent_count),
                result
            )
            .unwrap();

            if let LexerToken::StartTag(_) = token {
                self.indent_count += 1;
            }
        }

        content
    }

    fn format_start_tag(&self, tag: StartTag) -> String {
        let mut result: String = "".to_owned();

        let break_tag_attrs =
            self.get_len_tag_attributes(&tag.attributes) > self.config.max_line_length as usize;

        let attrs = self.format_tag_attributes(tag.attributes, break_tag_attrs);

        write!(result, "<{}", tag.tag_name).unwrap();

        if !attrs.is_empty() {
            if break_tag_attrs {
                write!(result, "\n{}", attrs).unwrap();
            } else {
                write!(result, " {}", attrs).unwrap();
            }
        }

        if tag.self_closing {
            write!(result, "/").unwrap();
        }

        write!(result, ">").unwrap();

        result
    }

    fn format_tag_attributes(&self, tag_attrs: Vec<TagAttribute>, break_tag_attrs: bool) -> String {
        tag_attrs
            .iter()
            .map(|tag_attr| {
                if break_tag_attrs {
                    format!(
                        "{}{}",
                        self.config.indent_style.repeat(self.indent_count + 1),
                        self.format_tag_attribute(tag_attr)
                    )
                } else {
                    self.format_tag_attribute(tag_attr)
                }
            })
            .collect::<Vec<String>>()
            .join(if break_tag_attrs { "\n" } else { " " })
    }

    fn get_len_tag_attributes(&self, tag_attrs: &Vec<TagAttribute>) -> usize {
        let mut line_length: usize = 0;

        for tag_attr in tag_attrs.iter() {
            line_length += self.format_tag_attribute(tag_attr).len();
        }

        line_length
    }

    fn format_tag_attribute(&self, tag_attr: &TagAttribute) -> String {
        format!(
            "{}=\"{}\"",
            tag_attr.attribute_name, tag_attr.attribute_value
        )
    }

    fn format_chars(&self, chars: Chars) -> String {
        chars.data
    }

    fn format_end_tag(&self, end_tag: EndTag) -> String {
        format!("</{}>", end_tag.tag_name)
    }
}
