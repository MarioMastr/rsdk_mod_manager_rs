pub mod mods_tab;
pub mod options_tab;

use native_dialog::DialogBuilder;

use crate::core::{json::ManagerSettings, rsdk::{RSDKInfo, Game}};
use mods_tab::Mods;
use options_tab::Options;

use eframe::egui;
use egui_extras::{Size, StripBuilder};
use egui_async::Bind;

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
    my_ip: Bind<(), String>,
}

impl RMM {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut this = Self::default();

        // only supported method of mac app is through bundle
        #[cfg(target_os = "macos")] {
            let res_folder = std::env::current_exe().unwrap().parent().unwrap().parent().unwrap().join("Resources");
            std::env::set_current_dir(res_folder).unwrap();
        }

        this.manager = ManagerSettings::read_json().expect("Unable to read/create managerSettings.json");
        this.game = RSDKInfo::get(&this.manager).expect("Unable to get information on selected game");

        cc.egui_ctx.options_mut(|a| a.theme_preference = egui::ThemePreference::System);

        this
    }
}

impl eframe::App for RMM {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.plugin_or_default::<egui_async::EguiAsyncPlugin>();
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let uri_opt = std::env::args().skip(1).find(|e| e.contains("://"));
            if uri_opt.is_some() {
                if let Some(res) = self.my_ip.read_or_request(|| async {
                    let uri = uri_opt.unwrap();
                    crate::core::web::gamebanana_uri_handler(uri.as_str()).await.map_err(|e| e.to_string())
                }) {
                    match res {
                        Ok(_)  => {
                            self.game.refresh(&self.manager);
                            ui.label("Mod download completed")
                        },
                        Err(err) => ui.colored_label(egui::Color32::RED, err),
                    };
                } else {
                    ui.spinner();
                }
            }

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
                            update_entry(&mut self.manager, &mut self.game, Game::Sonic1Forever);
                        }
                        if ui.button("Sonic 2 Absolute").clicked() {
                            update_entry(&mut self.manager, &mut self.game, Game::Sonic2Absolute);
                        }
                    }
                );
            }

            if !self.manager.games[self.manager.selected_game].path.exists() {
                egui::Window::new("ERROR")
                    .collapsible(false)
                    .resizable(false)
                    .show(ui.ctx(), |ui| {
                        ui.label("Game executable not found. Please select another executable.");
                        if ui.button("OK").clicked() && let Some(file) = DialogBuilder::file()
                            .set_location(".")
                            .add_filter("RSDK Executables", [""])
                            .set_filename("RSDKv")
                            .open_single_file()
                            .show()
                            .expect("Unable to open file selector")
                        {
                            let manager = &mut self.manager;
                            let game = &mut self.game;
                            let mut game_settings = manager.games[manager.selected_game].clone();
                            game_settings.path = file;
                            game_settings.save_entry(manager).expect("Unable to save entry");
                            game.refresh(manager);
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

#[cfg(target_os = "macos")]
                                let program = "./".to_owned() + &self.game.name.to_owned() + ".app/Contents/MacOS/" + &self.game.name.to_owned();
#[cfg(target_os = "windows")]
                                let program = "./".to_owned() + &self.game.name + ".exe";
#[cfg(target_os = "linux")]
                                let program = "./".to_owned() + &self.game.name;

                                std::process::Command::new(program)
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
