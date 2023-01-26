use crate::{config::Config, models::*};
use std::fmt::Write;

pub struct Formatter {
    tokens: Vec<LexerToken>,
    config: Config,
}

impl Formatter {
    pub fn new(tokens: Vec<LexerToken>, config: Config) -> Self {
        Self { tokens, config }
    }

    pub fn format(&self) -> String {
        let mut content: String = "".to_owned();

        let mut indent_count = 0;

        for token in self.tokens.iter().cloned() {
            let result = match token.clone() {
                LexerToken::StartTag(start_tag) => format_start_tag(start_tag),
                LexerToken::Chars(chars) => format_chars(chars),
                LexerToken::EndTag(end_tag) => {
                    indent_count -= 1;

                    format_end_tag(end_tag)
                }
            };

            write!(
                content,
                "{}{}\n",
                self.config.indent_style.repeat(indent_count),
                result
            )
            .unwrap();

            if let LexerToken::StartTag(_) = token {
                indent_count += 1;
            }
        }

        content
    }
}

fn format_start_tag(tag: StartTag) -> String {
    let mut result: String = "".to_owned();

    let attrs = format_tag_attributes(tag.attributes);

    write!(result, "<{}", tag.tag_name).unwrap();

    if !attrs.is_empty() {
        write!(result, " {}", attrs).unwrap();
    }

    if tag.self_closing {
        write!(result, "/").unwrap();
    }

    write!(result, ">").unwrap();

    result
}

fn format_tag_attributes(tag_attrs: Vec<TagAttribute>) -> String {
    let mut result: String = "".to_owned();

    for tag_attr in tag_attrs.iter() {
        write!(
            result,
            "{}=\"{}\"",
            tag_attr.attribute_name, tag_attr.attribute_value
        )
        .unwrap();
    }

    result
}

fn format_chars(chars: Chars) -> String {
    chars.data
}

fn format_end_tag(end_tag: EndTag) -> String {
    format!("</{}>", end_tag.tag_name)
}
