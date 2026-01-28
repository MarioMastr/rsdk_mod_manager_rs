use crate::mods::Mods;
use crate::options::Options;
use crate::rsdk;
use rfd::FileDialog;

use eframe::egui;

// #[derive(Default)]
// pub struct MyEguiApp {}

// impl MyEguiApp {
//     pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
//         // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
//         // Restore app state using cc.storage (requires the "persistence" feature).
//         // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
//         // for e.g. egui::PaintCallback.
//         Self::default()
//     }
// }

// impl eframe::App for MyEguiApp {
//    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//        egui::CentralPanel::default().show(ctx, |ui| {
//            ui.heading("Hello World!");
//        });
//    }
// }

#[derive(Default, PartialEq)]
enum Tabs {
    #[default]
    Mods,
    Options,
}

#[derive(Default)]
pub struct RMM {
    tabs: Tabs,
    mods: Mods,
    options: Options,
}

impl RMM {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
        
    }
}

impl eframe::App for RMM {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tabs, Tabs::Mods, "Mods");
                ui.selectable_value(&mut self.tabs, Tabs::Options, "Options");
            });
            ui.separator();
            match self.tabs {
                Tabs::Mods => {
                    let settings = crate::read_ini().unwrap();
                    self.mods.ui(ui, rsdk::get_game_info(settings).unwrap());
                }
                Tabs::Options => {
                    self.options.ui(ui);
                }
            }
            ui.separator();
        });
   }
}