use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub input: String,

    #[clap(short, long)]
    pub output: Option<String>,
}
