use crate::cl_args::Args;
use crate::config::Config;
use crate::process_replays::{export_to_xlsx, get_players, load_replay, merge_players};
use dirs;
use eframe::egui;
use rfd;
use std::path::PathBuf;

#[allow(dead_code)]
pub struct GUI {
    args: Box<Args>,
    config: Box<Config>,
    replays: Vec<PathBuf>,
}

impl GUI {
    pub fn new(_cc: &eframe::CreationContext<'_>, args: Box<Args>, config: Box<Config>) -> Self {
        Self {
            args,
            config,
            replays: Vec::new(),
        }
    }
}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Add replays…").clicked() {
                    if let Some(replays) = rfd::FileDialog::new()
                        .add_filter("wotreplay", &["wotreplay"])
                        .set_directory(dirs::home_dir().unwrap())
                        .pick_files()
                    {
                        self.replays.extend(replays);
                    }
                }
                if !self.replays.is_empty() {
                    if ui.button("Go").clicked() {
                        let mut buf = Vec::new();
                        for path in &self.replays {
                            let replay = load_replay(path.to_str().unwrap());
                            match replay {
                                Ok(replay) => get_players(&replay, &mut buf),
                                Err(_) => {}
                            }
                        }
                        let merged_players = merge_players(&buf);
                        if let Some(out) = rfd::FileDialog::new()
                            .add_filter("xlsx", &["xlsx"])
                            .set_directory(dirs::home_dir().unwrap())
                            // .set_file_name(&self.args.out_path)
                            .save_file()
                        {
                            export_to_xlsx(&merged_players, &self.config, out.to_str().unwrap())
                                .unwrap();
                        }
                    }
                }
            });
            if !self.replays.is_empty() {
                let mut replay_to_remove: Option<usize> = None;
                for (i, replay) in self.replays.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.heading(replay.file_name().unwrap().to_str().unwrap());
                        if ui.button("remove").clicked() {
                            replay_to_remove = Some(i);
                        }
                    });
                }
                if let Some(i) = replay_to_remove {
                    self.replays.remove(i);
                }
            };
        });
    }
}
