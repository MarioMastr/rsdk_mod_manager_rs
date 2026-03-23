use crate::{rsdk::{ ModInfo, RSDKInfo}, rsdk_json::ManagerSettings};
use eframe::egui;
use egui_extras::{TableBuilder, Column, StripBuilder, Size};

#[derive(PartialEq)]
pub struct Mods {
    selected_mod_index: Option<usize>
}

impl Default for Mods {
    fn default() -> Self {
        Self {
            selected_mod_index: None
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
            .auto_shrink([false; 2])
            .column(Column::remainder())
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
        }).body(|mut body| {
            for i in 0..game.mods.len() {
                let mi = &mut game.mods[i];
                body.row(text_height, |mut row| {
                    if let Some(index) = self.selected_mod_index {
                        row.set_selected(index == i);
                    }

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

                    self.toggle_row_selection(mi, &row.response(), i);
                });
            }
        });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, mut game: &mut RSDKInfo, manager: &mut ManagerSettings) {
        let len = game.mods.len();
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
                if let Some(index) = self.selected_mod_index {
                    if len != 1 {
                        if index != 0 {
                            if ui.button("Move Up").clicked() {
                                game.mods.swap(index, index - 1);
                            }
                            if ui.button("Move to Top").clicked() {
                                game.mods.swap(index, 0);
                            }
                        } else if index != len {
                            if ui.button("Move Down").clicked() {
                                game.mods.swap(index, index + 1);
                            }
                            if ui.button("Move to Bottom").clicked() {
                                game.mods.swap(index, len);
                            }
                        }
                    }
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
                if let Some(index) = self.selected_mod_index {
                    strip.cell(|ui| {
                        ui.label(format!("Description: {}", game.mods[index].description));
                    });
                }
                
            }
        );

        ui.separator();
    }

    fn toggle_row_selection(&mut self, mi: &mut ModInfo, row_response: &egui::Response, i: usize) {
        if row_response.clicked() {
            mi.selected = !mi.selected;
            if mi.selected {
                self.selected_mod_index = Some(i);
            } else {
                self.selected_mod_index = None;
            }
        }
    }
}
