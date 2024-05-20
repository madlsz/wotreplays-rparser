pub mod cl_args;
pub mod config;
pub mod gui;
pub mod player;
pub mod process_replays;

use crate::cl_args::Args;
use crate::config::Config;
use crate::process_replays::{export_to_xlsx, get_players, load_replay, merge_players};
use eframe::egui;

pub fn cli(args: Box<Args>, config: Box<Config>) {
    let mut buf = Vec::new();
    for (i, path) in args.replays.iter().enumerate() {
        println!("{}/{} {}", i + 1, args.replays.len(), path);
        let replay = load_replay(&path).unwrap();
        get_players(&replay, &mut buf);
    }

    let merged_players = merge_players(&buf);

    match export_to_xlsx(&merged_players, &config, &args.out_path) {
        Ok(()) => {}
        Err(e) => eprintln!("{e}"),
    };
}

pub fn gui(args: Box<Args>, config: Box<Config>) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([620.0, 460.0]),
        ..Default::default()
    };
    eframe::run_native(
        "wot-pref",
        options,
        Box::new(|cc| Box::new(gui::GUI::new(cc, args, config))),
    )
    .unwrap();
}
