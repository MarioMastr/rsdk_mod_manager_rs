use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};

use crate::core::rsdk::Game;
use native_dialog::DialogBuilder;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ManagerSettings {
    pub selected_game: usize,
    pub num_games: usize,
    pub games: Vec<GameSettings>,
}

impl ManagerSettings {
    pub fn create_json(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_path = Path::new("managerSettings.json");
        if settings_path.exists() {
            return Ok(());
        }

        if let Some(file) = DialogBuilder::file()
            .set_location(".")
            .add_filter("RSDK Executables", [""])
            .set_filename("RSDKv")
            .open_single_file()
            .show()
            .expect("Unable to open file selector")
        {
            let settings = GameSettings {
                nickname: format!("{:?}", Game::None),
                path: file,
                name: Game::None
            };

            self.games.push(settings);

            let settings_string = serde_json::to_string_pretty(self)?;
            let mut settings_file = File::create(settings_path)?;
            settings_file.write_all(settings_string.as_bytes())?;
        }

        Ok(())
    }

    pub fn read_json() -> Result<ManagerSettings, Box<dyn std::error::Error>> {
        let mut result = ManagerSettings::default();

        let settings_path = Path::new("managerSettings.json");
        if !settings_path.exists() {
            result.create_json()?;
        }

        let settings_string = fs::read_to_string("managerSettings.json")?;

        let result: ManagerSettings = serde_json::from_str(settings_string.as_str())?;

        Ok(result)
    }

    pub fn create_entry(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(file) = DialogBuilder::file()
            .set_location(".")
            .add_filter("RSDK Executables", [""])
            .set_filename("RSDKv")
            .open_single_file()
            .show()
            .expect("Unable to open file selector")
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

        self.save_json()?;

        Ok(())
    }

    pub fn remove_entry(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.games.remove(self.selected_game);

        self.num_games -= 1;
        self.selected_game = 0;

        self.save_json()?;

        Ok(())
    }

    pub fn save_json(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_string = serde_json::to_string_pretty(self)?;
        let mut settings_file = File::create("managerSettings.json")?;
        settings_file.write_all(settings_string.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameSettings {
    pub nickname: String,
    pub path: PathBuf,
    pub name: Game,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self { nickname: String::new(), path: Default::default(), name: Game::None }
    }
}

impl GameSettings {
    pub fn save_entry(&self, manager: &mut ManagerSettings) -> Result<(), Box<dyn std::error::Error>> {
        let _ = std::mem::replace(&mut manager.games[manager.selected_game], self.clone());

        manager.save_json()?;

        Ok(())
    }
}
