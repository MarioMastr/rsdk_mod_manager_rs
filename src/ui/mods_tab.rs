use crate::{core::{json::ManagerSettings, rsdk::{ModInfo, NewMod, RSDKInfo}}, ui::RMMError};
use eframe::egui::{self, UiKind};
use egui_extras::{TableBuilder, Column, StripBuilder, Size};

#[derive(PartialEq, Clone)]
pub struct Mods {
    selected_mod_index: Option<usize>,
    show_new_window: bool,
    show_new_scratch_window: bool,
    show_remove_window: bool,
    new_mod_option: NewMod,
    new_mod_name: String,
    new_mod_author: String,
    new_mod_version: String,
    new_mod_description: String,
    mod_window_title: String,
}

impl Default for Mods {
    fn default() -> Self {
        Self {
            selected_mod_index: None,
            show_new_window: false,
            show_new_scratch_window: false,
            show_remove_window: false,
            new_mod_option: NewMod::Archive,
            new_mod_name: String::new(),
            new_mod_author: String::new(),
            new_mod_version: String::new(),
            new_mod_description: String::new(),
            mod_window_title: "New Mod".to_string(),
        }
    }
}

impl RMMError for Mods {}

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

    pub fn ui(&mut self, ui: &mut egui::Ui, game: &mut RSDKInfo, manager: &mut ManagerSettings) {
        egui::Window::new(&self.mod_window_title)
            .id(egui::Id::new("new_mod_window"))
            .collapsible(false)
            .resizable(false)
            .open(&mut self.show_new_window)
            .show(ui.ctx(), |ui| {
                if self.show_new_scratch_window {
                    self.mod_window_title = "New Mod (Scratch)".to_string();
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.text_edit_singleline(&mut self.new_mod_name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Author: ");
                        ui.text_edit_singleline(&mut self.new_mod_author);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Version: ");
                        ui.text_edit_singleline(&mut self.new_mod_version);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Description (optional): ");
                        ui.text_edit_singleline(&mut self.new_mod_description);
                    });

                    ui.add_space(10.0);

                    if ui.button("OK").clicked() {
                        if let Err(res) = game.new_mod(self.new_mod_option, Some(self.new_mod_name.clone()), Some(self.new_mod_author.clone()), Some(self.new_mod_version.clone()), Some(self.new_mod_description.clone())) {
                            Mods::error_window(ui, "Unable to add new mod", res.as_ref(), true);
                        }
                        if let Err(res) = game.refresh(manager) {
                            Mods::error_window(ui, "Unable to refresh game", res.as_ref(), true);
                        }
                        ui.close_kind(UiKind::Window);
                    }
                } else {
                    ui.label("Select option:");
                    ui.add_space(10.0);
                    ui.radio_value(&mut self.new_mod_option, NewMod::Archive, "From Archive");
                    ui.radio_value(&mut self.new_mod_option, NewMod::Folder, "From Folder");
                    ui.radio_value(&mut self.new_mod_option, NewMod::Scratch, "From Scratch (for developers)");
                    ui.add_space(10.0);
                    if ui.button("OK").clicked() {
                        if self.new_mod_option == NewMod::Scratch {
                            self.show_new_scratch_window = true;
                        } else {
                            if let Err(res) = game.new_mod(self.new_mod_option, None, None, None, None) {
                                Mods::error_window(ui, "Unable to add new mod", res.as_ref(), true);
                            }
                            if let Err(res) = game.refresh(manager) {
                                Mods::error_window(ui, "Unable to refresh game", res.as_ref(), true);
                            }
                            ui.close_kind(UiKind::Window);
                        }
                    }
                }
            }
        );

        egui::Window::new("Remove")
            .collapsible(false)
            .resizable(false)
            .open(&mut self.show_remove_window)
            .show(ui, |ui| {
                if let Some(index) = self.selected_mod_index {
                    ui.label(format!("Are you sure you want to remove {}?", game.mods[index].name));
                    ui.add_space(10.0);
                    if ui.button("YES").clicked() {
                        if let Err(res) = game.remove_mod(index) {
                            Mods::error_window(ui, "Unable to remove mod", res.as_ref(), true);
                        }
                        if let Err(res) = game.refresh(manager) {
                            Mods::error_window(ui, "Unable to refresh game", res.as_ref(), true);
                        }
                        ui.close_kind(UiKind::Window);
                        self.selected_mod_index = None;
                    }
                }
            }
        );

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
                                self.selected_mod_index = Some(index - 1);
                            }
                            if ui.button("Move to Top").clicked() {
                                game.mods.swap(index, 0);
                                self.selected_mod_index = Some(0);
                            }
                        }
                        if index != (len - 1) {
                            if ui.button("Move Down").clicked() {
                                game.mods.swap(index, index + 1);
                                self.selected_mod_index = Some(index + 1);
                            }
                            if ui.button("Move to Bottom").clicked() {
                                game.mods.swap(index, len);
                                self.selected_mod_index = Some(len - 1);
                            }
                        }
                    }
                    if ui.button("Remove").clicked() {
                        self.show_remove_window = true;
                    }
                }
            });
        });

        ui.separator();

        let height = ui.available_height();

        StripBuilder::new(ui)
            .size(Size::remainder().at_least(height - 48.5))
            .size(Size::exact(5.0))
            .size(Size::remainder().at_most(5.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.table_ui(ui, game);
                    });
                });
                strip.cell(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Refresh").clicked() && let Err(res) = game.refresh(manager) {
                            Mods::error_window(ui, "Unable to refresh game", res.as_ref(), true);
                        }

                        ui.add_space(ui.available_width() - 34.0);

                        if ui.button("New").clicked() {
                            self.show_new_window = true;
                        }
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
