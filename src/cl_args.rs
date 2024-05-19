use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// list of replays to parse
    #[arg(required = true)]
    files: Vec<String>,
}
