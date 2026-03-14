#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

pub mod rsdk;
pub mod ui;
pub mod mods;
pub mod options;
pub mod rsdk_ini;

use ui::RMM;
use eframe::egui;

use crate::rsdk_ini::Settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([666.0, 585.0])
            .with_min_inner_size([666.0, 585.0]),

        ..Default::default()
    };
    let _ = eframe::run_native(
        "RSDK Mod Manager",
        options,
        Box::new(|cc| {
            Settings::create_ini().expect("Unable to create managerSettings.ini");
            Ok(Box::new(RMM::new(cc)))
        })
    );
    Ok(())
}
