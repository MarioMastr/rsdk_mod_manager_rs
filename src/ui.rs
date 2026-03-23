use std::path::PathBuf;

use native_dialog::DialogBuilder;

use crate::rsdk_json::ManagerSettings;
use crate::mods::Mods;
use crate::options::Options;
use crate::rsdk::{RSDKInfo, Game};

use eframe::egui;
use egui_extras::{Size, StripBuilder};

#[derive(Default, PartialEq)]
enum Tabs {
    #[default]
    Mods,
    Options,
}

#[derive(Default)]
pub struct RMM {
    tabs: Tabs,
    mods: Mods,
    options: Options,
    game: RSDKInfo,
    manager: ManagerSettings,
}

impl RMM {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.#
        let mut this = Self::default();

        this.manager = ManagerSettings::read_json().expect("Unable to read/create managerSettings.json");
        this.game = RSDKInfo::get(&this.manager).expect("Unable to get information on selected game");

        this
    }
}

impl eframe::App for RMM {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.game.game == Game::None {
                let update_entry = |
                    manager: &mut ManagerSettings,
                    game: &mut RSDKInfo,
                    game_name: Game
                | {
                    let mut game_settings = manager.games[manager.selected_game].clone();
                    game_settings.name = game_name;
                    game_settings.nickname = format!("{:?}", game_name);
                    game_settings.save_entry(manager).expect("Unable to save entry");
                    game.refresh(manager);
                };

                egui::Window::new("Select Game")
                    .collapsible(false)
                    .resizable(false)
                    .show(ui.ctx(), |ui| {
                        if ui.button("Sonic 1").clicked() {
                            update_entry(&mut self.manager, &mut self.game, Game::Sonic1);
                        }
                        if ui.button("Sonic 2").clicked() {
                            update_entry(&mut self.manager, &mut self.game, Game::Sonic2);
                        }
                        if ui.button("Sonic CD").clicked() {
                            update_entry(&mut self.manager, &mut self.game, Game::SonicCD);
                        }
                        if ui.button("Sonic Mania").clicked() {
                            update_entry(&mut self.manager, &mut self.game, Game::SonicMania);
                        }
                        if ui.button("Sonic 1 Forever").clicked() {
                            update_entry(&mut self.manager, &mut self.game, Game::S1F);
                        }
                        if ui.button("Sonic 2 Absolute").clicked() {
                            update_entry(&mut self.manager, &mut self.game, Game::S2A);
                        }
                    }
                );
            }

            if !self.manager.games[self.manager.selected_game].path.exists() {
                let update_entry = |
                    manager: &mut ManagerSettings,
                    game: &mut RSDKInfo,
                    file: PathBuf
                | {
                    let mut game_settings = manager.games[manager.selected_game].clone();
                    game_settings.path = file;
                    game_settings.save_entry(manager).expect("Unable to save entry");
                    game.refresh(manager);
                };

                egui::Window::new("ERROR")
                    .collapsible(false)
                    .resizable(false)
                    .show(ui.ctx(), |ui| {
                        ui.label("Game executable not found. Please select another executable.");
                        if ui.button("OK").clicked() {
                            if let Some(file) = DialogBuilder::file()
                                .set_location(".")
                                .add_filter("RSDK Executables", [""])
                                .set_filename("RSDKv")
                                .open_single_file()
                                .show()
                                .expect("Unable to open file selector") {
                                    update_entry(&mut self.manager, &mut self.game, file);
                            }
                        }
                    }
                );
            }

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tabs, Tabs::Mods, "Mods");
                ui.selectable_value(&mut self.tabs, Tabs::Options, "Options");
            });

            ui.separator();

            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0))
                .size(Size::exact(17.5))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        match self.tabs {
                            Tabs::Mods => {
                                self.mods.ui(ui, &mut self.game, &mut self.manager);
                            }
                            Tabs::Options => {
                                self.options.ui(ui, &mut self.game, &mut self.manager);
                            }
                        }
                    });
                    strip.cell(|ui| {
                        ui.horizontal(|ui| {
                            egui::ComboBox::from_label(String::new())
                                .selected_text(&self.manager.games[self.manager.selected_game].nickname)
                                .show_ui(ui, |ui| {
                                    for n in 0..self.manager.num_games {
                                        let settings = &self.manager.games[n];
                                        if ui.selectable_value(&mut self.manager.selected_game, n, &settings.nickname).changed() {
                                            self.game.refresh(&self.manager);
                                            self.manager.save_json().expect("Unable to save managerSettings.json");
                                        }
                                    }
                                    if ui.button("New Game").clicked() {
                                        self.manager.create_entry().expect("Unable to create entry");
                                        self.game.refresh(&self.manager);
                                    }
                                }
                            );
                            if ui.button("Save & Play").clicked() {
                                self.game.save().expect("Unable to save changes");
                                std::process::Command::new("./".to_owned() + &self.game.name)
                                    .current_dir(&self.game.path)
                                    .output()
                                    .expect("Unable to launch game");
                            }
                            if ui.button("Save").clicked() {
                                self.game.save().expect("Unable to save changes");
                            }
                        });
                    });
                }
            );
        });
   }
}
