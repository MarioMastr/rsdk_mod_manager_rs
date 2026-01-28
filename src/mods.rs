// use eframe::egui::{self, Id};

// use crate::rsdk;

// #[derive(PartialEq)]
// pub struct ModTable {
//     clickable: bool,
// }

// impl Default for ModTable {
//     fn default() -> Self {
//         Self {
//             clickable: true,
//         }
//     }
// }

// impl ModTable {
//     fn ui(&mut self, ui: &mut egui::Ui) {
//         egui::SidePanel::left(Id::new("mods_list")).show(ui.ctx(), |ui| {
//             self.table_ui(ui, rsdk::read_ini(String::from("/home/mariomastr/Applications/RSDK/Sonic 1/")).unwrap());
//         });
//     }

//     fn table_ui(&mut self, ui: &mut egui::Ui, mods: Vec<rsdk::ModInfo>) {
//         use egui_extras::{Column, TableBuilder};

//         let text_height = egui::TextStyle::Body
//             .resolve(ui.style())
//             .size
//             .max(ui.spacing().interact_size.y);
//         let mut table = TableBuilder::new(ui)
//             .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
//             .column(Column::auto())
//             .column(Column::auto())
//             .column(Column::auto())
//             .min_scrolled_height(0.0);

//         if self.clickable {
//             table = table.sense(egui::Sense::click());
//         }

//         table.header(20.0, |mut header| {
//             header.col(|ui| {
//                 ui.strong("Name");
//             });
//             header.col(|ui| {
//                 ui.strong("Author");
//             });
//             header.col(|ui| {
//                 ui.strong("Version");
//             });
//         }).body(|mut body| {
//             for mi in mods {
//                 body.row(text_height, |mut row| {
//                     row.col(|ui| {
//                         ui.label(mi.name);
//                     });
//                     row.col(|ui| {
//                         ui.label(mi.author);
//                     });
//                     row.col(|ui| {
//                         ui.label(mi.version);
//                     });
//                 });
//             }
//         });
//     }
// }

// #[derive(PartialEq)]
// pub struct Mods {
//     table: ModTable,
// }

// impl Default for Mods {
//     fn default() -> Self {
//         Self {
//             table: ModTable::default()
//         }
//     }
// }

// impl Mods {
//     pub fn ui(&mut self, ui: &mut egui::Ui) {
//         ui.separator();
//         egui::CentralPanel::default().show(ui.ctx(), |ui| {
//             self.table.ui(ui);
//         });
//     }
// }

use crate::{Settings, rsdk};
use eframe::egui;

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
    fn ui(&mut self, ui: &mut egui::Ui, game: rsdk::GameInfo) {
        self.table_ui(ui, game.mods);
    }

    fn table_ui(&mut self, ui: &mut egui::Ui, mods: Vec<rsdk::ModInfo>) {
        use egui_extras::{Column, TableBuilder};

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
            for mut mi in mods {
                body.row(text_height, |mut row| {
                    row.col(|ui| {
                        ui.checkbox(&mut mi.enabled, "");
                        ui.label(mi.name);
                    });
                    row.col(|ui| {
                        ui.label(mi.author);
                    });
                    row.col(|ui| {
                        ui.label(mi.version);
                    });
                });
            }
        });
    }
}

#[derive(PartialEq)]
pub struct Mods {
    table: ModTable
}

impl Default for Mods {
    fn default() -> Self {
        Self {
            table: ModTable::default()
        }
    }
}

impl Mods {
    pub fn ui(&mut self, ui: &mut egui::Ui, game: rsdk::GameInfo) {
        self.table.ui(ui, game);
    }
}