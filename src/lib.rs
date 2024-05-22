pub mod cl_args;
pub mod config;
pub mod gui;
pub mod player;
pub mod process_replays;

use crate::cl_args::Args;
use crate::config::Config;
use crate::process_replays::{export_to_xlsx, get_players, load_replay, merge_players};
use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

pub fn cli(args: Rc<RefCell<Args>>, config: Rc<RefCell<Config>>) {
    let mut buf = Vec::new();
    for (i, path) in args.borrow().replays.iter().enumerate() {
        println!("{}/{} {}", i + 1, args.borrow().replays.len(), path);
        let replay = load_replay(&path).unwrap();
        get_players(&replay, &mut buf);
    }

    let merged_players = merge_players(&buf);

    match export_to_xlsx(&merged_players, &config.borrow(), &args.borrow().out_path) {
        Ok(()) => {}
        Err(e) => eprintln!("{e}"),
    };
}

pub fn gui(args: Rc<RefCell<Args>>, config: Rc<RefCell<Config>>) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.borrow().gui.width, config.borrow().gui.height]),
        ..Default::default()
    };
    eframe::run_native(
        "rparser",
        options,
        Box::new(|cc| Box::new(gui::GUI::new(cc, args, config))),
    )
    .unwrap();
}
