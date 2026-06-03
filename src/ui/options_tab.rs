use eframe::egui::{self, UiKind};
use egui_extras::{StripBuilder, Size};
use crate::core::{rsdk::RSDKInfo, json::ManagerSettings};

#[derive(PartialEq, Default)]
pub struct Options {
    show_delete_box: bool
}

impl Options {
    pub fn ui(&mut self, ui: &mut egui::Ui, game: &mut RSDKInfo, manager: &mut ManagerSettings) {
        if manager.num_games == 1 {
            egui::Window::new("ERROR")
                .collapsible(false)
                .resizable(false)
                .open(&mut self.show_delete_box)
                .show(ui.ctx(), |ui| {
                    ui.label("At least one game must be added.");
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("OK").clicked() {
                            ui.close_kind(UiKind::Window);
                        }
                    });
                }
            );
        } else {
            egui::Window::new("Remove Game")
                .collapsible(false)
                .resizable(false)
                .open(&mut self.show_delete_box)
                .show(ui.ctx(), |ui| {
                    ui.label(format!("Are you sure you want to remove {:?}?", manager.games[manager.selected_game].nickname));
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            manager.remove_entry().expect("Unable to remove entry");
                            game.refresh(manager);
                            ui.close_kind(UiKind::Window);
                        }
                        if ui.button("No").clicked() {
                            ui.close_kind(UiKind::Window);
                        }
                    });
                }
            );
        }

        let height = ui.available_height();
        
        StripBuilder::new(ui)
            .size(Size::initial(50.0))
            .size(Size::remainder().at_most(height - 8.75))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Nickname: ");
                        if ui.text_edit_singleline(&mut manager.games[manager.selected_game].nickname).lost_focus() {
                            manager.save_json().expect("Unable to save managerSettings.json");
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Remove Current Game").clicked() {
                            self.show_delete_box = true;
                        }
                        #[cfg(target_os = "windows")] {
                            if ui.button("Install URL Handler").clicked() {

                            }
                        }
                    });
                });
                strip.cell(|ui| {
                    ui.label(egui::RichText::new("Updates").underline().strong());
                    if ui.button("Check now").clicked() {

                    }
                });
            }
        );

        ui.separator();
    }
}
