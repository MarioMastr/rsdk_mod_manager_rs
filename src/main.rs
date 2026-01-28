#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

pub mod rsdk;
pub mod ui;
pub mod mods;
pub mod options;

use std::io::Write;

use eframe::egui;
use ini::Ini;

pub struct Settings {
    path: std::path::PathBuf,
    name: rsdk::Game,
}

impl Default for Settings {
    fn default() -> Self {
        Self { path: Default::default(), name: crate::rsdk::Game::Sonic1 }
    }
}

fn get_game(ctx: &egui::Context) -> rsdk::Game {
    let mut result = crate::rsdk::Game::Sonic1;

    egui::Window::new("Select Game")
        .resizable([true, false]) // resizable so we can shrink if the text edit grows
        .default_width(280.0)
        .show(ctx, |ui| {
            egui::ComboBox::from_label("Game: ")
                .selected_text(format!("{result:?}"))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut result, rsdk::Game::Sonic1, "Sonic 1");
                    ui.selectable_value(&mut result, rsdk::Game::Sonic2, "Sonic 2");
                    ui.selectable_value(&mut result, rsdk::Game::SonicCD, "Sonic CD");
                    ui.selectable_value(&mut result, rsdk::Game::SonicMania, "Sonic Mania");
            });
        });

    result
}

fn create_ini(ctx: &egui::Context) -> Result<(), Box<dyn std::error::Error>> {
    let manager_settings: &std::path::Path = std::path::Path::new("managerSettings.ini");
    if manager_settings.exists() {
        return Ok(());
    }

    let mut settings = Ini::new();




    let file = rfd::FileDialog::new()
        .add_filter("RSDK Executables", &[""])
        .set_file_name("RSDKv")
        .set_directory("/")
        .pick_file();
    
    let game = get_game(ctx);

    settings.with_section(Some("settings"))
        .set("path", String::from(file.unwrap().to_str().unwrap()))
        .set("game", String::from(format!("{game:?}")));

    settings.write_to_file("managerSettings.ini")?;


    Ok(())
}

fn read_ini() -> Result<Settings, Box<dyn std::error::Error>> {
    let mut result = Settings::default();

    let manager_settings = Ini::load_from_file("managerSettings.ini")?;
    let section = manager_settings.section(Some("settings")).unwrap();

    result.path = std::path::PathBuf::from(section.get("path").unwrap());

    Ok(result)
}

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
            create_ini(&cc.egui_ctx).unwrap();
            Ok(Box::new(ui::RMM::new(cc)))
        })
    );
    Ok(())
}