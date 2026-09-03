use eframe::egui::{self, UiKind};
use egui_extras::{StripBuilder, Size};
use crate::{core::{json::ManagerSettings, rsdk::RSDKInfo}, ui::RMMError};

#[cfg(target_os = "windows")]
use crate::core::web;

#[derive(PartialEq, Default)]
pub struct Options {
    show_delete_box: bool
}

impl RMMError for Options {}

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
                            if let Err(res) = manager.remove_entry() {
                                Options::error_window(ui, "Unable to remove entry", res.as_ref(), true);
                            }
                            if let Err(res) = game.refresh(manager) {
                                Options::error_window(ui, "Unable to refresh game", res.as_ref(), true);
                            }
                            ui.close_kind(UiKind::Window);
                        }
                        if ui.button("No").clicked() {
                            ui.close_kind(UiKind::Window);
                        }
                    });
                }
            );
        }

        // let height = ui.available_height();
        
        StripBuilder::new(ui)
            .size(Size::initial(50.0))
            // .size(Size::remainder().at_most(height - 72.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Nickname: ");
                        if ui.text_edit_singleline(&mut manager.games[manager.selected_game].nickname).lost_focus() && let Err(res) = manager.save_json() {
                            Options::error_window(ui, "Unable to save managerSettings.json", res.as_ref(), true);
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Remove Current Game").clicked() {
                            self.show_delete_box = true;
                        }
                        // #[cfg(target_os = "windows")] {
                        //     if ui.button("Install URL Handler").clicked() {
                        //         let uri = web::get_uri(game.game);
                        //         if let Err(res) = web::windows_install_uri(uri) {
                        //             Options::error_window(ui, "Unable to add uri", res.as_ref(), true);
                        //         }
                        //     }
                        // }
                    });
                });
                // strip.cell(|ui| {
                //     ui.label(egui::RichText::new("Updates").underline().strong());
                //     if ui.button("Check now").clicked() {
                //     }
                // });
            }
        );

        ui.separator();
    }
}
