#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

pub mod rsdk;
pub mod ui;
pub mod mods;
pub mod options;
pub mod rsdk_ini;

use ui::RMM;

fn main() -> iced::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    rsdk_ini::Settings::create_ini().unwrap();
    iced::application(RMM::new, RMM::update, RMM::view)
        .theme(iced::Theme::CatppuccinMocha)
        .run()
}
