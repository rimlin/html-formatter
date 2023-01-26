mod config;
mod formatter;
mod input_stream;
mod lexer;
mod models;
mod utils;

use config::Config;
use formatter::Formatter;
use input_stream::InputStream;
use lexer::Lexer;
use std::fs;

fn main() {
    let example = String::from(
        "<html>hello<div class=\"wrapper\"><span>world</span><time>09:41</time></div></html>",
    );

    let config = Config::new("space");
    let stream = InputStream::new(&example);
    let mut lexer = Lexer::new(stream);

    let tokens = lexer.tokenize();
    let formatter = Formatter::new(tokens.to_owned(), config);
    let content = formatter.format();

    fs::write("./result.html", content).expect("Unable to write file");
}
