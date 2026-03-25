use eframe::egui;
use egui_extras::{StripBuilder, Size};
use crate::core::{rsdk::RSDKInfo, json::ManagerSettings};

#[derive(PartialEq, Default)]
pub struct Options {
    save_path: String,
    show_delete_box: bool
}

impl Options {
    pub fn ui(&mut self, ui: &mut egui::Ui, game: &mut RSDKInfo, manager: &mut ManagerSettings) {
        if self.show_delete_box {
            if manager.num_games == 1 {
                egui::Window::new("ERROR")
                    .collapsible(false)
                    .resizable(false)
                    .show(ui.ctx(), |ui| {
                        ui.label("At least one game must be added.");
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if ui.button("OK").clicked() {
                                self.show_delete_box = false;
                            }
                        });
                    }
                );
            } else {
                egui::Window::new("Remove Game")
                    .collapsible(false)
                    .resizable(false)
                    .show(ui.ctx(), |ui| {
                        ui.label(format!("Are you sure you want to remove {:?}?", manager.games[manager.selected_game].nickname));
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if ui.button("Yes").clicked() {
                                manager.remove_entry().expect("Unable to remove entry");
                                game.refresh(manager);
                                self.show_delete_box = false;
                            }
                            if ui.button("No").clicked() {
                                self.show_delete_box = false;
                            }
                        });
                    }
                );
            }
        }

        let height = ui.available_height();
        
        StripBuilder::new(ui)
            .size(Size::remainder().at_most(height - 8.75))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    if ui.button("Remove Current Game").clicked() {
                        self.show_delete_box = true;
                    }
                    ui.horizontal(|ui| {
                        ui.label("Nickname: ");
                        if ui.text_edit_singleline(&mut manager.games[manager.selected_game].nickname).lost_focus() {
                            manager.save_json().expect("Unable to save managerSettings.json");
                        }
                    });
                });
            }
        );

        ui.separator();
    }
}
