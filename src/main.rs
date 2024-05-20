use clap::Parser;

use wotreplays_rparser::cl_args::Args;
use wotreplays_rparser::config::Config;
use wotreplays_rparser::{cli, gui};

fn main() {
    // load config, create it if it doesnt exist
    let config = Box::new(match Config::load() {
        Some(config) => config,
        None => {
            let config = Config::new();
            config.save().unwrap();
            config
        }
    });

    let args = Box::new(Args::parse());
    match args.gui {
        true => gui(args, config),
        false => cli(args, config),
    };
}
