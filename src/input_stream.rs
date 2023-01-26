pub struct InputStream<'a> {
    pos: usize,
    pub line: usize,
    pub col: usize,
    pub input: &'a str,
}

impl<'a> InputStream<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            pos: 0,
            line: 1,
            col: 0,
            input,
        }
    }

    pub fn next(&mut self) -> char {
        let char = self.input.chars().nth(self.pos).unwrap();

        self.pos += 1;
        if char == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }

        char
    }

    pub fn peek(&self) -> char {
        self.input.chars().nth(self.pos).unwrap()
    }

    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn croak(&self, msg: &str) {
        panic!("{} ({}:{})", msg, self.line, self.col);
    }
}
