use crate::cl_args::Args;
use crate::config::Config;
use crate::process_replays::{export_to_xlsx, get_players, load_replay, merge_players};
use dirs;
use eframe::egui;
use rfd;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[allow(dead_code)]
pub struct GUI {
    args: Rc<RefCell<Args>>,
    config: Rc<RefCell<Config>>,
    replays: Vec<PathBuf>,
    show_settings: bool,
}

impl GUI {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        args: Rc<RefCell<Args>>,
        config: Rc<RefCell<Config>>,
    ) -> Self {
        Self {
            args,
            config,
            replays: Vec::new(),
            show_settings: false,
        }
    }
}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // settings window, opens when self.show_settings
                egui::Window::new("Settings")
                    .open(&mut self.show_settings)
                    .show(ctx, |ui| {
                        // ui.set_width(100.0);
                        // ui.set_height(100.0);
                        if ui.button("Delete config").clicked() {
                            match Config::delete_from_disk() {
                                Ok(_) => println!("Config deleted"),
                                Err(e) => eprintln!("{e}"),
                            }
                        }
                    });
                // set self.show_settings to true in order to show the settings window
                if ui.button("Settings").clicked() {
                    self.show_settings = true;
                }
                // show file dialog to select replays, load the replays to self.replays
                if ui.button("Add replays…").clicked() {
                    let mut config_mut = self.config.borrow_mut();
                    if let Some(replays) = rfd::FileDialog::new()
                        .add_filter("wotreplay", &["wotreplay"])
                        .set_directory(match &config_mut.select_replays_last_path {
                            Some(path) => PathBuf::from(path),
                            None => dirs::home_dir().unwrap(),
                        })
                        .pick_files()
                    {
                        self.replays.extend(replays);
                        let parent = self
                            .replays
                            .last()
                            .unwrap()
                            .parent()
                            .unwrap()
                            .to_str()
                            .unwrap();

                        match &config_mut.select_replays_last_path {
                            Some(path) => {
                                if path != parent {
                                    config_mut.select_replays_last_path = Some(parent.to_string());
                                    config_mut.is_edited = true;
                                }
                            }
                            None => {
                                config_mut.select_replays_last_path = Some(parent.to_string());
                                config_mut.is_edited = true
                            }
                        }
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
                            export_to_xlsx(
                                &merged_players,
                                &self.config.borrow(),
                                out.to_str().unwrap(),
                            )
                            .unwrap();
                        }
                    }
                }
            });
            if !self.replays.is_empty() {
                let mut replay_to_remove: Option<usize> = None;
                ui.label(format!("{} replays", self.replays.len()));
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, replay) in self.replays.iter().enumerate() {
                        ui.horizontal(|ui| {
                            if ui
                                .label(replay.file_name().unwrap().to_str().unwrap())
                                .on_hover_text("right-click to remove")
                                .secondary_clicked()
                            {
                                replay_to_remove = Some(i);
                            }
                        });
                    }
                });
                if let Some(i) = replay_to_remove {
                    self.replays.remove(i);
                }
            };
        });
    }
}
