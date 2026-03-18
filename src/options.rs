use eframe::egui;
use crate::{rsdk::RSDKInfo, rsdk_json::ManagerSettings};

#[derive(PartialEq)]
pub struct Options {
    save_path: String,
    show_delete_box: bool
}

impl Default for Options {
    fn default() -> Self {
        Self {
            save_path: String::new(),
            show_delete_box: false
        }
    }
}

impl Options {
    pub fn ui(&mut self, ui: &mut egui::Ui, game: &mut RSDKInfo, manager: &mut ManagerSettings) {
        if self.show_delete_box {
            if manager.num_games == 1 {
                egui::Window::new("ERROR")
                    .collapsible(false)
                    .resizable(false)
                    .show(ui.ctx(), |ui| {
                        ui.label(format!("At least one game must be added."));
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
        
        if ui.button("Remove Current Game").clicked() {
            self.show_delete_box = true;
        }
    }
}
