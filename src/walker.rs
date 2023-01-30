use crate::config::Config;
use crate::formatter::Formatter;
use crate::input_stream::InputStream;
use crate::lexer::Lexer;
use std::fs;

pub struct Walker {
    config: Config,
}

impl Walker {
    pub fn new(config: Config) -> Self {
        log::trace!("init new Walker with config {:#?}", config);

        Walker { config }
    }
}

impl Walker {
    pub fn run(&self) {
        for path in &self.config.files {
            self.format_file(path)
        }
    }

    fn format_file(&self, path: &String) {
        log::trace!("start format file {}", path);

        let content = fs::read_to_string(path).expect("Should have been able to read file");

        let stream = InputStream::new(content.as_str());
        let mut lexer = Lexer::new(stream);

        let tokens = lexer.tokenize();
        let formatter = Formatter::new(tokens.to_owned(), &self.config);
        let content = formatter.format();

        fs::write(path, content).expect("Unable to write file");

        log::trace!("finish format file {}", path);
    }
}
