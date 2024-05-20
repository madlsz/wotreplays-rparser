use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// list of replays to parse
    pub files: Vec<String>,

    /// enable GUI
    #[arg(short, long, default_value_t = false)]
    pub gui: bool,
}
