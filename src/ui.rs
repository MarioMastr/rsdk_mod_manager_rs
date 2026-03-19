use std::ops::Index;

use crate::rsdk_json::ManagerSettings;
use crate::mods::Mods;
use crate::options::Options;
use crate::rsdk::RSDKInfo;

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
                                        let settings = self.manager.games.index(n);
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
