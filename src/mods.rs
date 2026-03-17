use crate::{rsdk::{Game, RSDKInfo}, rsdk_json::ManagerSettings};
use eframe::egui;
use egui_extras::{TableBuilder, Column};

#[derive(PartialEq)]
pub struct ModTable {
    clickable: bool,
    resizable: bool,
    striped: bool,
}

impl Default for ModTable {
    fn default() -> Self {
        Self {
            clickable: true,
            resizable: true,
            striped: true,
        }
    }
}

impl ModTable {
    fn ui(&mut self, ui: &mut egui::Ui, game: &mut RSDKInfo) {
        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);
        let mut table = TableBuilder::new(ui)
            .resizable(self.resizable)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder());
            // min_scrolled_height(0.0);

        if self.clickable {
            table = table.sense(egui::Sense::click());
        }

        table.header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Name");
            });
            header.col(|ui| {
                ui.strong("Author");
            });
            header.col(|ui| {
                ui.strong("Version");
            });
        }).body(|mut body| {
            for mi in &mut game.mods {
                body.row(text_height, |mut row| {
                    row.col(|ui| {
                        ui.checkbox(&mut mi.enabled, "");
                        ui.label(&mi.name);
                    });
                    row.col(|ui| {
                        ui.label(&mi.author);
                    });
                    row.col(|ui| {
                        ui.label(&mi.version);
                    });
                });
            }
        });
    }
}

#[derive(PartialEq)]
pub struct Mods {
    table: ModTable,
    striped: bool,
    overline: bool,
    resizable: bool,
    clickable: bool,
}

impl Default for Mods {
    fn default() -> Self {
        Self {
            table: ModTable::default(),
            striped: true,
            overline: true,
            resizable: true,
            clickable: true,
        }
    }
}

impl Mods {
    pub fn ui(&mut self, ui: &mut egui::Ui, mut game: &mut RSDKInfo, manager: &mut ManagerSettings) {
        if game.game == Game::None {
            egui::Window::new("Select Game")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    if ui.button("Sonic 1").clicked() {
                        game.game = Game::Sonic1;
                        let mut game_settings = manager.games[manager.selected_game].clone();
                        game_settings.name = game.game;
                        game_settings.nickname = format!("{:?}", game.game);
                        game_settings.save_entry(manager).expect("Unable to save entry");
                        *game = RSDKInfo::get(manager.clone()).expect("Unable to get information on selected game");
                    }
                    if ui.button("Sonic 2").clicked() {
                        game.game = Game::Sonic2;
                        let mut game_settings = manager.games[manager.selected_game].clone();
                        game_settings.name = game.game;
                        game_settings.nickname = format!("{:?}", game.game);
                        game_settings.save_entry(manager).expect("Unable to save entry");
                        *game = RSDKInfo::get(manager.clone()).expect("Unable to get information on selected game");
                    }
                    if ui.button("Sonic CD").clicked() {
                        game.game = Game::SonicCD;
                        let mut game_settings = manager.games[manager.selected_game].clone();
                        game_settings.name = game.game;
                        game_settings.nickname = format!("{:?}", game.game);
                        game_settings.save_entry(manager).expect("Unable to save entry");
                        *game = RSDKInfo::get(manager.clone()).expect("Unable to get information on selected game");
                    }
                    if ui.button("Sonic Mania").clicked() {
                        game.game = Game::SonicMania;
                        let mut game_settings = manager.games[manager.selected_game].clone();
                        game_settings.name = game.game;
                        game_settings.nickname = format!("{:?}", game.game);
                        game_settings.save_entry(manager).expect("Unable to save entry");
                        *game = RSDKInfo::get(manager.clone()).expect("Unable to get information on selected game");
                    }
                }
            );
        }

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Enable All").clicked() {
                    game.mods.iter_mut().for_each(|mi| {
                        mi.enabled = true;
                    });
                }
                if ui.button("Disable All").clicked() {
                    game.mods.iter_mut().for_each(|mi| {
                        mi.enabled = false;
                    });
                }
            });
        });

        ui.separator();

        egui::ScrollArea::horizontal().show(ui, |ui| {
            self.table.ui(ui, &mut game);
        });
    }
}
