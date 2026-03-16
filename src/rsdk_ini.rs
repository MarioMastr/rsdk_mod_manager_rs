use std::path::PathBuf;

use ini::Ini;

use crate::rsdk::{self, RSDKInfo, Game};
use native_dialog::DialogBuilder;

pub struct Settings {
    pub path: PathBuf,
    pub name: Game,
}

impl Default for Settings {
    fn default() -> Self {
        Self { path: Default::default(), name: Game::None }
    }
}

impl Settings {
    pub fn create_ini() -> Result<(), Box<dyn std::error::Error>> {
        let manager_settings: &std::path::Path = std::path::Path::new("managerSettings.ini");
        if manager_settings.exists() {
            return Ok(());
        }

        let mut settings = Ini::new();
        let game = crate::rsdk::Game::None;

        if let Some (file) = DialogBuilder::file()
            .set_location(".")
            .add_filter("RSDK Executables", [""])
            .set_filename("RSDKv")
            .open_single_file()
            .show()
            .expect("Unable to open file selector")
        {
            settings.with_section(Some("settings"))
                .set("path", file.to_str().unwrap())
                .set("game", format!("{game:?}"));

            settings.write_to_file("managerSettings.ini")?;
        }

        Ok(())
    }

    pub fn read_ini() -> Result<Settings, Box<dyn std::error::Error>> {
        let mut result = Settings::default();

        let manager_settings = Ini::load_from_file("managerSettings.ini")?;

        if let Some(section) = manager_settings.section(Some("settings")) {
            if let Some(path) = section.get("path") {
                result.path = PathBuf::from(path);
            }
            if let Some(game) = section.get("game") {
                if game == "None" {
                    result.name = Game::None;
                } else if game == "Sonic 1" {
                    result.name = Game::Sonic1;
                } else if game == "Sonic 2" {
                    result.name = Game::Sonic2;
                } else if game == "Sonic CD" {
                    result.name = Game::SonicCD;
                } else if game == "Sonic Mania" {
                    result.name = Game::SonicMania;
                }
            }
        }

        Ok(result)
    }

    pub fn save_ini(game: &RSDKInfo) -> Result<(), Box<dyn std::error::Error>> {
        let mut settings = Ini::new();

        let game_text = {
            if game.game == Game::Sonic1 {
                "Sonic 1"
            } else if game.game == Game::Sonic2 {
                "Sonic 2"
            } else if game.game == Game::SonicCD {
                "Sonic CD"
            } else if game.game == Game::SonicMania {
                "Sonic Mania"
            } else {
                "None"
            }
        };
        let path = game.path.join(&game.name);

        settings.with_section(Some("settings"))
            .set("path", path.to_str().unwrap())
            .set("game", format!("{game_text}"));

        settings.write_to_file("managerSettings.ini")?;

        Ok(())
    }
}
