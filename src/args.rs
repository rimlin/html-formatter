#[derive(Debug, clap::Parser)]
#[clap(about = "HTML formatter")]
pub struct Args {
	#[clap(long, short = 's')]
	#[clap(help = "Indent style: space or tab")]
	#[clap(value_parser = ["tab", "space"])]
    pub indent_style: String,
}
