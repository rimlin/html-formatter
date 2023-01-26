use std::str::FromStr;

#[derive(Clone)]
pub struct LocationPos {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone)]
pub struct Location {
    pub start: LocationPos,
    pub end: LocationPos,
}

pub trait Token {
    fn add_loc(&mut self, loc: Location);
}

#[derive(Clone)]
pub struct TagAttribute {
    pub attribute_name: String,
    pub attribute_value: String,
}

impl TagAttribute {
    pub fn append_to_attribute_name(&mut self, char: char) {
        let mut buffer = [0; 4];
        self.attribute_name.push_str(char.encode_utf8(&mut buffer));
    }

    pub fn append_to_attribute_value(&mut self, char: char) {
        let mut buffer = [0; 4];
        self.attribute_value.push_str(char.encode_utf8(&mut buffer));
    }
}

#[derive(Clone)]
pub struct StartTag {
    pub tag_name: String,
    pub attributes: Vec<TagAttribute>,
    pub self_closing: bool,
    pub loc: Option<Location>,
}

impl StartTag {
    pub fn append_to_tag_name(&mut self, char: char) {
        let mut buffer = [0; 4];
        self.tag_name.push_str(char.encode_utf8(&mut buffer));
    }

    pub fn append_to_attributes(&mut self, tag_attribute: TagAttribute) {
        self.attributes.push(tag_attribute);
    }

    pub fn mark_as_self_closing(&mut self) {
        self.self_closing = true;
    }
}

impl Token for StartTag {
    fn add_loc(&mut self, loc: Location) {
        self.loc = Some(loc);
    }
}

#[derive(Clone)]
pub struct EndTag {
    pub tag_name: String,
    pub loc: Option<Location>,
}

impl EndTag {
    pub fn append_to_tag_name(&mut self, char: char) {
        let mut buffer = [0; 4];
        self.tag_name.push_str(char.encode_utf8(&mut buffer));
    }
}

impl Token for EndTag {
    fn add_loc(&mut self, loc: Location) {
        self.loc = Some(loc);
    }
}

#[derive(Clone)]
pub struct Chars {
    pub data: String,
    pub loc: Option<Location>,
}

impl Chars {
    pub fn append_to_chars(&mut self, char: char) {
        let mut buffer = [0; 4];
        self.data.push_str(char.encode_utf8(&mut buffer));
    }
}

impl Token for Chars {
    fn add_loc(&mut self, loc: Location) {
        self.loc = Some(loc);
    }
}

#[derive(Clone)]
pub enum LexerToken {
    StartTag(StartTag),
    EndTag(EndTag),
    Chars(Chars),
}

#[derive(Default)]
pub enum IndentStyle {
    #[default]
    Tab,
    Space,
}

impl IndentStyle {
    pub fn repeat(&self, size: usize) -> String {
        match self {
            Self::Tab => "	".repeat(size),
            Self::Space => " ".repeat(size),
        }
    }
}

impl FromStr for IndentStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tab" => Ok(Self::Tab),
            "space" => Ok(Self::Space),
            _ => Err("Not valid indent style"),
        }
    }
}
