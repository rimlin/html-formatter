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
        max_line_length,
    } = args;

    env_logger::init();

    log::trace!("max_line_length = {}", max_line_length);
    log::trace!("files = {:#?}", files);

    let config = Config::new(files)
        .set_indent_style(indent_style)
        .set_max_line_length(max_line_length);
    let walker = Walker::new(config);

    walker.run();
}
