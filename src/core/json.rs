use std::{fs, path::PathBuf, error::Error};
use serde::{Deserialize, Serialize};

use crate::core::rsdk::Game;
use native_dialog::DialogBuilder;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GameSettings {
    pub nickname: String,
    pub path: PathBuf,
    pub name: Game,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManagerSettings {
    pub selected_game: usize,
    pub num_games: usize,
    pub games: Vec<GameSettings>,
}

impl Default for ManagerSettings {
    fn default() -> Self {
        Self { selected_game: 0, num_games: 1, games: Vec::new() }
    }
}

impl ManagerSettings {
    pub fn create_json(&mut self) -> Result<(), Box<dyn Error>> {
        let os_config = dirs::config_local_dir().ok_or("Unable to get config directory")?;
        let app_config = os_config.join("rmm");
        if !app_config.exists() {
            fs::create_dir_all(&app_config)?;
        }

        let settings_path = app_config.join("managerSettings.json");

        if settings_path.exists() {
            return Ok(());
        }

#[cfg(target_os = "macos")]
        let extension = ".app";
#[cfg(target_os = "windows")]
        let extension = ".exe";
#[cfg(target_os = "linux")]
        let extension = "";

        if let Some(file) = DialogBuilder::file()
            .add_filter("RSDK Executables", [extension])
            .set_filename("RSDKv")
            .open_single_file()
            .show()?
        {
            let settings = GameSettings {
                nickname: format!("{:?}", Game::None),
                path: file,
                name: Game::None
            };

            self.games.push(settings);

            let settings_string = serde_json::to_string_pretty(self)?;
            fs::write(settings_path, settings_string.as_bytes())?;
        }

        Ok(())
    }

    pub fn read_json() -> Result<ManagerSettings, Box<dyn Error>> {
        let mut result = ManagerSettings::default();

        let os_config = dirs::config_local_dir().ok_or("Unable to get config directory")?;
        let app_config = os_config.join("rmm");
        if !app_config.exists() {
            fs::create_dir_all(&app_config)?;
        }

        let settings_path = app_config.join("managerSettings.json");
        if !settings_path.exists() {
            result.create_json()?;
        }

        let settings_string = fs::read_to_string(settings_path)?;

        Ok(serde_json::from_str(settings_string.as_str())?)
    }

    pub fn create_entry(&mut self) -> Result<(), Box<dyn Error>> {
#[cfg(target_os = "macos")]
        let extension = ".app";
#[cfg(target_os = "windows")]
        let extension = ".exe";
#[cfg(target_os = "linux")]
        let extension = "";

        if let Some(file) = DialogBuilder::file()
            .add_filter("RSDK Executables", [extension])
            .set_filename("RSDKv")
            .open_single_file()
            .show()?
        {
            let settings = GameSettings {
                nickname: format!("{:?}", Game::None),
                path: file,
                name: Game::None
            };

            self.games.push(settings);
            self.num_games = self.games.len();
            self.selected_game = self.num_games - 1;
        }

        self.save_json()
    }

    pub fn remove_entry(&mut self) -> Result<(), Box<dyn Error>> {
        self.games.remove(self.selected_game);

        self.num_games -= 1;
        self.selected_game = 0;

        self.save_json()
    }

    pub fn save_json(&self) -> Result<(), Box<dyn Error>> {
        let settings_string = serde_json::to_string_pretty(self)?;
        let config = dirs::config_local_dir().ok_or("Unable to get config directory")?;
        let settings_path = config.join("rmm").join("managerSettings.json");
        Ok(fs::write(settings_path, settings_string.as_bytes())?)
    }
}
