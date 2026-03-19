use crate::{rsdk::{ ModInfo, RSDKInfo}, rsdk_json::ManagerSettings};
use eframe::egui;
use egui_extras::{TableBuilder, Column, StripBuilder, Size};

#[derive(PartialEq)]
pub struct Mods {
    selected_mod: ModInfo
}

impl Default for Mods {
    fn default() -> Self {
        Self {
            selected_mod: ModInfo::default()
        }
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
            .column(Column::remainder())
            .column(Column::auto())
            .column(Column::remainder());

        table = table.sense(egui::Sense::click());
        let mut clicked = false;

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
            game.mods.iter_mut().for_each(|mi| {
                body.row(text_height, |mut row| {
                    clicked |= row.col(|ui| {
                        ui.checkbox(&mut mi.enabled, "");
                        ui.label(&mi.name);
                    }).1.clicked();
                    clicked |= row.col(|ui| {
                        ui.label(&mi.author);
                    }).1.clicked();
                    clicked |= row.col(|ui| {
                        ui.label(&mi.version);
                    }).1.clicked();
                });

                if clicked {
                    self.selected_mod = mi.clone();
                    clicked = false;
                }
            });
        });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, mut game: &mut RSDKInfo, manager: &mut ManagerSettings) {
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

        let height = ui.available_height();

        StripBuilder::new(ui)
            .size(Size::remainder().at_most(height - 48.5))
            .size(Size::exact(5.0))
            .size(Size::exact(5.0))
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
                strip.cell(|ui| {
                    ui.label(format!("Description: {}", self.selected_mod.description));
                });
                
            }
        );

        ui.separator();

    }
}
