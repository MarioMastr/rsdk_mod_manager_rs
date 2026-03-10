use std::path::PathBuf;

use ini::Ini;

use crate::rsdk;

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

        if let Some(file) = rfd::FileDialog::new()
            .add_filter("RSDK Executables", &[""])
            .set_file_name("RSDKv")
            .set_directory("/")
            .pick_file()
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
