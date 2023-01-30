use crate::input_stream::InputStream;
use crate::models::*;
use crate::utils;

#[derive(Eq, Hash, PartialEq)]
enum LexerState {
    BeforeData,
    Chars,
    TagOpen,
    TagName,
    EndTagOpen,
    EndTagName,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
}

pub struct Lexer<'a> {
    input_stream: InputStream<'a>,
    state: LexerState,
    tokens: Vec<LexerToken>,
    start_line: usize,
    start_col: usize,
    current_attribute: Option<TagAttribute>,
}

impl<'a> Lexer<'a> {
    pub fn new(input_stream: InputStream<'a>) -> Self {
        Self {
            input_stream,
            state: LexerState::BeforeData,
            tokens: Vec::new(),
            start_col: 0,
            start_line: 1,
            current_attribute: None,
        }
    }

    pub fn tokenize(&mut self) -> &Vec<LexerToken> {
        while !self.input_stream.eof() {
            self.read_next();
        }

        &self.tokens
    }

    fn read_next(&mut self) {
        // self.read_while(utils::is_whitespace);

        if self.input_stream.eof() {
            // end of file
            println!("End of reading");
        } else {
            match self.state {
                LexerState::BeforeData => self.read_before_data(),
                LexerState::Chars => self.read_chars(),
                LexerState::TagOpen => self.read_tag_open(),
                LexerState::TagName => self.read_tag_name(),
                LexerState::EndTagOpen => self.read_end_tag_open(),
                LexerState::EndTagName => self.read_end_tag_name(),
                LexerState::BeforeAttributeName => self.read_before_attribute_name(),
                LexerState::AttributeName => self.read_attribute_name(),
                LexerState::AfterAttributeName => self.read_after_attribute_name(),
                LexerState::BeforeAttributeValue => self.read_before_attribute_value(),
                LexerState::AttributeValueDoubleQuoted => self.read_attribute_value_double_quoted(),
                LexerState::AttributeValueSingleQuoted => self.read_attribute_value_single_quoted(),
                LexerState::AttributeValueUnquoted => self.read_attribute_value_unquoted(),
                LexerState::AfterAttributeValueQuoted => self.read_after_attribute_value_quoted(),
                LexerState::SelfClosingStartTag => self.read_self_closing_start_tag(),
            };
        }
    }

    fn transition_to(&mut self, state: LexerState) {
        self.state = state;
    }

    fn consume(&mut self) -> char {
        self.input_stream.next()
    }

    fn read_while(&mut self, predicate: fn(char) -> bool) -> String {
        let mut str: String = "".to_string();

        while !self.input_stream.eof() && predicate(self.input_stream.peek()) {
            str.push(self.input_stream.next());
        }

        str
    }

    fn push(&mut self, token: LexerToken) {
        self.tokens.push(token);
    }

    fn retrieve_loc_info(&mut self) -> Location {
        let loc = Location {
            start: LocationPos {
                line: self.start_line,
                column: self.start_col,
            },
            end: LocationPos {
                line: self.input_stream.line,
                column: self.input_stream.col,
            },
        };

        self.start_line = self.input_stream.line;
        self.start_col = self.input_stream.col;

        loc
    }

    fn begin_chars(&mut self) {
        self.push(LexerToken::Chars(Chars {
            data: "".to_owned(),
            loc: None,
        }));
    }

    fn append_to_chars(&mut self, char: char) {
        let token = self.tokens.last_mut();

        if let Some(LexerToken::Chars(chars)) = token {
            chars.append_to_chars(char);
        }
    }

    fn finish_chars(&mut self) {
        let loc = self.retrieve_loc_info();
        let token = self.tokens.last_mut();

        if let Some(LexerToken::Chars(chars)) = token {
            chars.add_loc(loc)
        }
    }

    fn begin_start_tag(&mut self) {
        self.push(LexerToken::StartTag(StartTag {
            tag_name: "".to_owned(),
            attributes: vec![],
            self_closing: false,
            loc: None,
        }))
    }

    fn begin_end_tag(&mut self) {
        self.push(LexerToken::EndTag(EndTag {
            tag_name: "".to_owned(),
            loc: None,
        }))
    }

    fn append_to_tag_name(&mut self, char: char) {
        let token = self.tokens.last_mut();

        if let Some(LexerToken::StartTag(tag)) = token {
            tag.append_to_tag_name(char);
        } else if let Some(LexerToken::EndTag(tag)) = token {
            tag.append_to_tag_name(char);
        }
    }

    fn mark_tag_as_self_closing(&mut self) {
        let token = self.tokens.last_mut();

        if let Some(LexerToken::StartTag(tag)) = token {
            tag.mark_as_self_closing();
        }
    }

    fn finish_tag(&mut self) {
        let loc = self.retrieve_loc_info();
        let token = self.tokens.last_mut();

        if let Some(LexerToken::StartTag(tag)) = token {
            tag.add_loc(loc);
        } else if let Some(LexerToken::EndTag(tag)) = token {
            tag.add_loc(loc);
        }
    }

    fn begin_attribute(&mut self) {
        self.current_attribute = Some(TagAttribute {
            attribute_name: "".to_owned(),
            attribute_value: "".to_owned(),
        })
    }

    fn append_to_attribute_name(&mut self, char: char) {
        if let Some(attribute) = self.current_attribute.as_mut() {
            attribute.append_to_attribute_name(char);
        }
    }

    fn append_to_attribute_value(&mut self, char: char) {
        if let Some(attribute) = self.current_attribute.as_mut() {
            attribute.append_to_attribute_value(char);
        }
    }

    fn finish_attribute_value(&mut self) {
        let token = self.tokens.last_mut();

        if let Some(LexerToken::StartTag(tag)) = token {
            tag.append_to_attributes(self.current_attribute.take().unwrap());
        }
    }

    // Events

    fn read_before_data(&mut self) {
        let char = self.input_stream.peek();

        if char == '<' {
            self.transition_to(LexerState::TagOpen);
            self.consume();
        } else {
            self.transition_to(LexerState::Chars);
            self.begin_chars();
        }
    }

    fn read_chars(&mut self) {
        let char = self.input_stream.peek();

        if char == '<' {
            self.finish_chars();
            self.transition_to(LexerState::TagOpen);
            self.consume();
        } else {
            self.consume();
            self.append_to_chars(char);
        }
    }

    fn read_tag_open(&mut self) {
        let char = self.consume();

        if char == '/' {
            self.transition_to(LexerState::EndTagOpen)
        } else if utils::is_alphabet(char) {
            self.transition_to(LexerState::TagName);
            self.begin_start_tag();
            self.append_to_tag_name(char);
        }
    }

    fn read_tag_name(&mut self) {
        let char = self.consume();

        if char.is_whitespace() {
            self.transition_to(LexerState::BeforeAttributeName);
        } else if char == '/' {
            self.transition_to(LexerState::SelfClosingStartTag);
        } else if char == '>' {
            self.finish_tag();
            self.transition_to(LexerState::BeforeData);
        } else {
            self.append_to_tag_name(char);
        }
    }

    fn read_end_tag_open(&mut self) {
        let char = self.consume();

        if utils::is_alphabet(char) {
            self.transition_to(LexerState::EndTagName);
            self.begin_end_tag();
            self.append_to_tag_name(char);
        }
    }

    fn read_end_tag_name(&mut self) {
        let char = self.consume();

        if char.is_whitespace() {
            self.transition_to(LexerState::BeforeAttributeName);
        } else if char == '/' {
            self.transition_to(LexerState::SelfClosingStartTag);
        } else if char == '>' {
            self.finish_tag();
            self.transition_to(LexerState::BeforeData);
        } else {
            self.append_to_tag_name(char);
        }
    }

    fn read_before_attribute_name(&mut self) {
        let char = self.input_stream.peek();

        if char.is_whitespace() {
            self.consume();
        } else if char == '/' {
            self.transition_to(LexerState::SelfClosingStartTag);
        } else if char == '>' {
            self.finish_tag();
            self.transition_to(LexerState::BeforeData);
        } else if char == '=' {
            println!("syntax error \"attribute name can't start with equals sign\"");
            self.transition_to(LexerState::AttributeName);
            self.begin_attribute();
        } else {
            self.transition_to(LexerState::AttributeName);
            self.begin_attribute();
        }
    }

    fn read_attribute_name(&mut self) {
        let char = self.input_stream.peek();

        if char.is_whitespace() {
            self.transition_to(LexerState::AfterAttributeName);
            self.consume();
        } else if char == '/' {
            self.finish_attribute_value();
            self.consume();
            self.transition_to(LexerState::SelfClosingStartTag);
        } else if char == '=' {
            self.transition_to(LexerState::BeforeAttributeValue);
            self.consume();
        } else if char == '>' {
            self.finish_attribute_value();
            self.consume();
            self.finish_tag();
            self.transition_to(LexerState::BeforeData);
        } else if char == '"' || char == '\'' || char == '<' {
            println!("syntax error \"invalid char in attribute name\"");
            self.consume();
            self.append_to_attribute_name(char);
        } else {
            self.consume();
            self.append_to_attribute_name(char);
        }
    }

    fn read_after_attribute_name(&mut self) {
        let char = self.input_stream.peek();

        if char.is_whitespace() {
            self.consume();
        } else if char == '/' {
            self.finish_attribute_value();
            self.consume();
            self.transition_to(LexerState::SelfClosingStartTag);
        } else if char == '=' {
            self.transition_to(LexerState::BeforeAttributeValue);
            self.consume();
        } else if char == '>' {
            self.finish_attribute_value();
            self.consume();
            self.finish_tag();
            self.transition_to(LexerState::BeforeData);
        } else {
            /*
             * When start a new attribute.
             * Example: <div data-attr1 data-attr2>
             */
            self.finish_attribute_value();
            self.transition_to(LexerState::AttributeName);
            self.begin_attribute();
            self.consume();
            self.append_to_attribute_name(char);
        }
    }

    fn read_before_attribute_value(&mut self) {
        let char = self.input_stream.peek();

        if char.is_whitespace() {
            self.consume();
        } else if char == '"' {
            self.transition_to(LexerState::AttributeValueDoubleQuoted);
            self.consume();
        } else if char == '\'' {
            self.transition_to(LexerState::AttributeValueSingleQuoted);
            self.consume();
        } else if char == '>' {
            self.finish_attribute_value();
            self.consume();
            self.finish_tag();
            self.transition_to(LexerState::BeforeData);
        } else {
            self.transition_to(LexerState::AttributeValueUnquoted);
            self.consume();
            self.append_to_attribute_value(char);
        }
    }

    fn read_attribute_value_double_quoted(&mut self) {
        let char = self.consume();

        if char == '"' {
            self.finish_attribute_value();
            self.transition_to(LexerState::AfterAttributeValueQuoted);
        } else {
            self.append_to_attribute_value(char);
        }
    }

    fn read_attribute_value_single_quoted(&mut self) {
        let char = self.consume();

        if char == '\'' {
            self.finish_attribute_value();
            self.transition_to(LexerState::AfterAttributeValueQuoted);
        } else {
            self.append_to_attribute_value(char);
        }
    }

    fn read_attribute_value_unquoted(&mut self) {
        let char = self.input_stream.peek();

        if char.is_whitespace() {
            self.finish_attribute_value();
            self.consume();
            self.transition_to(LexerState::BeforeAttributeName);
        } else if char == '/' {
            let next_char = self.consume();

            if next_char == '>' {
                self.finish_attribute_value();
                self.transition_to(LexerState::SelfClosingStartTag);
            } else {
                // In example: <a href=https://www.w3schools.com>
                self.append_to_attribute_value(char);
            }
        } else if char == '>' {
            self.finish_attribute_value();
            self.consume();
            self.finish_tag();
            self.transition_to(LexerState::BeforeData);
        } else {
            self.consume();
            self.append_to_attribute_value(char);
        }
    }

    fn read_after_attribute_value_quoted(&mut self) {
        let char = self.input_stream.peek();

        if char.is_whitespace() {
            self.consume();
            self.transition_to(LexerState::BeforeAttributeName);
        } else if char == '/' {
            self.consume();
            self.transition_to(LexerState::SelfClosingStartTag);
        } else if char == '>' {
            self.consume();
            self.finish_tag();
            self.transition_to(LexerState::BeforeData);
        } else {
            self.transition_to(LexerState::BeforeAttributeName);
        }
    }

    fn read_self_closing_start_tag(&mut self) {
        let char = self.input_stream.peek();

        if char == '>' {
            self.consume();
            self.mark_tag_as_self_closing();
            self.finish_tag();
            self.transition_to(LexerState::BeforeData);
        } else {
            self.transition_to(LexerState::BeforeAttributeName);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::input_stream::InputStream;

    #[test]
    fn attribute_without_quotes() {
        let content =
            "<html><body><a href=https://www.w3schools.com>This is a link</a></body></html>";

        let stream = InputStream::new(content);
        let mut lexer = Lexer::new(stream);
        let tokens = lexer.tokenize();

        insta::assert_debug_snapshot!(tokens);
    }
}
