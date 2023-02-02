#[derive(Debug, clap::Parser)]
#[clap(about = "HTML formatter")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub files: Vec<String>,

    #[clap(long, short = 's')]
    #[clap(help = "Indent style: space or tab")]
    #[clap(value_parser = ["tab", "space"])]
    pub indent_style: Option<String>,

    #[clap(long, short = 'l', default_value_t = 80)]
    #[clap(help = "Max length of line")]
    pub max_line_length: i32,
}
