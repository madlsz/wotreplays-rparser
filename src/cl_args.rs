use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// List of replays to parse
    pub replays: Vec<String>,

    /// Disable GUI
    #[arg(short, long, default_value_t = true)]
    pub gui: bool,

    /// Out path
    #[arg(short, default_value = "./a.xlsx")]
    pub out_path: String,
}
