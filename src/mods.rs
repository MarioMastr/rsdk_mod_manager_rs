use crate::{rsdk::{Game, RSDKInfo}, rsdk_ini::Settings};
use eframe::egui::{self, TextStyle};
use egui_extras::{TableBuilder, Column};

#[derive(PartialEq)]
pub struct ModTable {
    clickable: bool,
    resizable: bool,
    striped: bool,
    game: RSDKInfo
}

impl Default for ModTable {
    fn default() -> Self {
        Self {
            clickable: true,
            resizable: true,
            striped: true,
            game: RSDKInfo::get().expect("Cannot get information on selected game"),
        }
    }
}

impl ModTable {
    fn ui(&mut self, ui: &mut egui::Ui) {
        if self.game.game == Game::None {
            egui::Window::new("Select Game")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    if ui.button("Sonic 1").clicked() {
                        self.game.game = Game::Sonic1;
                    }
                    if ui.button("Sonic 2").clicked() {
                        self.game.game = Game::Sonic2;
                    }
                    if ui.button("Sonic CD").clicked() {
                        self.game.game = Game::SonicCD;
                    }
                    if ui.button("Sonic Mania").clicked() {
                        self.game.game = Game::SonicMania;
                    }
                }
            );

            Settings::save_ini(&self.game).expect("Unable to save managerSettings.ini");
        }
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Enable All").clicked() {
                    self.game.mods.iter_mut().for_each(|mi| {
                        mi.enabled = true;
                    });
                }
                if ui.button("Disable All").clicked() {
                    self.game.mods.iter_mut().for_each(|mi| {
                        mi.enabled = false;
                    });
                }
                if ui.button("Save").clicked() {
                    self.game.save().expect("Unable to save changes");
                }
                if ui.button("Play").clicked() {
                    self.game.save().expect("Unable to save changes");
                    std::process::Command::new("./".to_owned() + &self.game.name)
                        .current_dir(&self.game.path)
                        .output().expect("Unable to launch game");
                }
            });
        });

        ui.separator();

        // Leave room for the source code link after the table demo:
        let body_text_size = TextStyle::Body.resolve(ui.style()).size;
        use egui_extras::{Size, StripBuilder};
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0)) // for the table
            .size(Size::exact(body_text_size)) // for the source code link
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.table_ui(ui);
                    });
                });
            });

    }

    fn table_ui(&mut self, ui: &mut egui::Ui) {
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
            for mi in &mut self.game.mods {
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
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.table.ui(ui);
    }
}
