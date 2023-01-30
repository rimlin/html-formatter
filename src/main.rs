mod args;
mod config;
mod formatter;
mod input_stream;
mod lexer;
mod models;
mod utils;
mod walker;

use args::Args;
use clap::Parser;
use config::Config;
use walker::Walker;

fn main() {
    let args = Args::parse();
    let Args {
        files,
        indent_style,
    } = args;

    env_logger::init();

    log::trace!("files = {:#?}", files);

    let config = Config::new(files).set_indent_style(indent_style);
    let walker = Walker::new(config);

    walker.run();
}