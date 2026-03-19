use crate::{rsdk::{Game, RSDKInfo}, rsdk_json::{GameSettings, ManagerSettings}};
use eframe::egui;
use egui_extras::{TableBuilder, Column, StripBuilder, Size};

#[derive(PartialEq)]
pub struct Mods {}

impl Default for Mods {
    fn default() -> Self {
        Self {}
    }
}

impl Mods {
    pub fn table_ui(&mut self, ui: &mut egui::Ui, game: &mut RSDKInfo) {
        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);
        let mut table = TableBuilder::new(ui)
            .resizable(true)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder());

        table = table.sense(egui::Sense::click());

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
            header.col(|ui| {
                ui.strong("Description");
            });
        }).body(|mut body| {
            game.mods.iter_mut().for_each(|mi| {
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
                    row.col(|ui| {
                        ui.label(&mi.description);
                    });
                });
            });
        });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, mut game: &mut RSDKInfo, manager: &mut ManagerSettings) {
        if game.game == Game::None {
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
                        update_entry(manager, game, Game::Sonic1);
                    }
                    if ui.button("Sonic 2").clicked() {
                        update_entry(manager, game, Game::Sonic2);
                    }
                    if ui.button("Sonic CD").clicked() {
                        update_entry(manager, game, Game::SonicCD);
                    }
                    if ui.button("Sonic Mania").clicked() {
                        update_entry(manager, game, Game::SonicMania);
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

        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0))
            .size(Size::exact(15.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.table_ui(ui, &mut game);
                    });
                });
                strip.cell(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Refresh").clicked() {
                            game.refresh(manager);
                        }

                        ui.add_space(ui.available_width() - 34.0);

                        if ui.button("New").clicked() {}
                    });
                });
            }
        );

        ui.separator();

    }
}
