use std::path::PathBuf;

use ini::Ini;

use crate::rsdk;
use native_dialog::DialogBuilder;

pub struct Settings {
    pub path: std::path::PathBuf,
    pub name: rsdk::Game,
}

impl Default for Settings {
    fn default() -> Self {
        Self { path: Default::default(), name: crate::rsdk::Game::Sonic1 }
    }
}

impl Settings {
    pub fn get_game() -> rsdk::Game {
        let result = crate::rsdk::Game::Sonic1;

        result
    }
 
    pub fn create_ini() -> Result<(), Box<dyn std::error::Error>> {
        let manager_settings: &std::path::Path = std::path::Path::new("managerSettings.ini");
        if manager_settings.exists() {
            return Ok(());
        }

        let mut settings = Ini::new();
        let game = Settings::get_game();

        if let Some (file) = DialogBuilder::file()
            .set_location("~")
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
        }

        Ok(result)
    }
}
