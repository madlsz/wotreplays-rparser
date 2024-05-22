#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use clap::Parser;
use std::cell::RefCell;
use std::rc::Rc;

use wotreplays_rparser::cl_args::Args;
use wotreplays_rparser::config::Config;
use wotreplays_rparser::{cli, gui};

fn main() {
    // load config, create it if it doesnt exist
    let config = Rc::new(RefCell::new(match Config::load() {
        Some(config) => config,
        None => {
            let config = Config::new();
            config.save().unwrap();
            config
        }
    }));

    let args = Rc::new(RefCell::new(Args::parse()));
    match args.borrow().gui {
        true => gui(Rc::clone(&args), Rc::clone(&config)),
        false => cli(Rc::clone(&args), Rc::clone(&config)),
    };

    if config.borrow().is_edited {
        config.borrow().save().unwrap();
    }
}
