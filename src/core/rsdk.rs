use std::{io::Write, path::PathBuf};
use ini::Ini;
use native_dialog::DialogBuilder;
use serde::{Deserialize, Serialize};
use archive::{ArchiveExtractor, ArchiveFormat};

use crate::core::json::ManagerSettings;
use crate::core::web::{self, GameBananaURIs};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum Game {
    Sonic1,
    Sonic2,
    SonicCD,
    SonicMania,
    Sonic1Forever,
    Sonic2Absolute,

    #[default]
    None,
}

#[derive(PartialEq, Default, Clone, Copy)]
pub enum NewMod {
    #[default]
    Archive,
    Folder,
    Scratch,
}

#[derive(PartialEq, Debug , Clone, Default)]
pub struct ModInfo {
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub enabled: bool,
    pub selected: bool
}

#[derive(PartialEq)]
pub struct RSDKInfo {
    pub rsdk_revision: u8,
    pub game: Game,
    pub name: String,
    pub path: PathBuf,
    pub mods: Vec<ModInfo>,
}

impl Default for RSDKInfo {
    fn default() -> Self {
        Self { rsdk_revision: 4, game: Game::None, name: String::new(), path: PathBuf::new(), mods: Vec::<ModInfo>::new(), }
    }
}

impl RSDKInfo {
    pub fn get(manager_settings: &ManagerSettings) -> Result<RSDKInfo, Box<dyn std::error::Error>> {
        let mut result = RSDKInfo::default();

        let game_settings = &manager_settings.games[manager_settings.selected_game];

        result.game = game_settings.name;

        if let Some(parent) = game_settings.path.parent() {
            result.path = parent.to_path_buf();
        }

        if let Some(rsdk_name) = game_settings.path.file_prefix() && let Some(rsdk_name_str) = rsdk_name.to_str() {
            result.name = rsdk_name_str.to_string();
            match rsdk_name_str {
                "RSDKv3" => result.rsdk_revision = 3,
                "RSDKv4" => result.rsdk_revision = 4,
                "RSDKv5" => result.rsdk_revision = 5,
                "RSDKv5U" => result.rsdk_revision = 5,

                _ => {}
            };
        }

        result.mods = result.get_mods()?;

        Ok(result)
    }

    pub fn refresh(&mut self, manager_settings: &ManagerSettings) {
        *self = RSDKInfo::get(manager_settings).expect("Unable to get information on selected game");
    }

    pub fn get_mods(&self) -> Result<Vec<ModInfo>, Box<dyn std::error::Error>> {
        let mut result = Vec::<ModInfo>::new();
        let mods_path = self.path.join("mods");
        let mods_dir = mods_path.read_dir()?;

        let modconfig_ini_path = mods_path.join("modconfig.ini");
        let modconfig_ini = Ini::load_from_file(modconfig_ini_path)?;

        let mods_text = if self.rsdk_revision == 5 {
            "Mods"
        } else {
            "mods"
        };

        if let Some(modconfig_section) = modconfig_ini.section(Some(mods_text)) {
            for entry in mods_dir {
                let mut temp = ModInfo::default();
                let entry = entry?;

                if !entry.path().is_dir() {
                    continue;
                }

                let mod_ini_path = entry.path().join("mod.ini");
                let mod_ini = Ini::load_from_file(mod_ini_path)?;

                let section = mod_ini.section(None::<String>).unwrap();

                temp.name = section.get("Name").unwrap().to_string();
                temp.author = section.get("Author").unwrap().to_string();
                temp.version = section.get("Version").unwrap().to_string();

                if let Some(description) = section.get("Description") {
                    temp.description = description.to_string();
                }

                if let Some(modconfig_key) = modconfig_section.get(&temp.name) {
                    temp.enabled = modconfig_key == (
                        if self.rsdk_revision == 5 {
                            "y"
                        } else {
                            "true"
                        }
                    );
                }

                if let Some(entry_name) = entry.file_name().to_str() && entry_name != temp.name {
                    let new_entry = mods_path.join(&temp.name);
                    std::fs::rename(entry.path(), new_entry)?;
                }

                result.push(temp);
            };
        }

        Ok(result)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let modconfig_ini_path = self.path.join("mods").join("modconfig.ini");

        let mods_text = if self.rsdk_revision == 5 {
            "Mods"
        } else {
            "mods"
        };

        let enabled_text = if self.rsdk_revision == 5 {
            "y"
        } else {
            "true"
        };

        let disabled_text = if self.rsdk_revision == 5 {
            "n"
        } else {
            "false"
        };

        let mut modconfig_ini_new = Ini::new();
        let mut section = modconfig_ini_new.with_section(Some(mods_text));

        for mi in &self.mods {
            section.set(&mi.name, if mi.enabled {
                enabled_text
            } else {
                disabled_text
            });
        }

        modconfig_ini_new.write_to_file(modconfig_ini_path)?;

        Ok(())
    }

    pub fn new_mod(
        &mut self,
        method: NewMod,
        _name: Option<String>,
        _desc: Option<String>,
        _ver: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match method {
            NewMod::Archive => {
                if let Some(archive) = DialogBuilder::file()
                    .set_location(".")
                    .add_filter("Mod Archives", ["zip", "7z"])
                    .open_single_file()
                    .show()
                    .expect("Unable to open file selector") {
                        let mut format = ArchiveFormat::Zip;
                        let extension = archive.extension().unwrap();
                        if extension == "zip" {
                            format = ArchiveFormat::Zip;
                        } else if extension == "7z" {
                            format = ArchiveFormat::SevenZ;
                        }

                        let data = std::fs::read(archive)?;
                        let extractor = ArchiveExtractor::new();
                        let files = extractor.extract(&data, format)?;

                        let mods_directory = self.path.join("mods");

                        for file in files {
                            let mut desired_file = std::fs::File::create(mods_directory.join(file.path))?;
                            desired_file.write_all(&file.data)?;
                        }
                }
            },
            NewMod::Folder => {
                if let Some(folder) = DialogBuilder::file()
                    .set_location(".")
                    .open_single_dir()
                    .show()
                    .expect("Unable to open file selector") {
                        let mods_directory = self.path.join("mods");
                        if let Some(name) = folder.file_name() {
                            let desired_mod_dir = mods_directory.join(name);
                            std::fs::rename(folder, desired_mod_dir)?;
                        }
                }
            },
            NewMod::Scratch => {},
        }

        Ok(())
    }

    pub async fn new_mod_online(
        &mut self,
        code: &str,
        url: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut mod_game: Game = Game::None;

        if code == GameBananaURIs::Sonic1.as_str() {
            mod_game = Game::Sonic1;
        } else if code == GameBananaURIs::Sonic2.as_str() {
            mod_game = Game::Sonic2;
        } else if code == GameBananaURIs::SonicCD.as_str(){
            mod_game = Game::SonicCD;
        } else if code == GameBananaURIs::SonicMania.as_str() {
            mod_game = Game::SonicMania;
        } else if code == GameBananaURIs::Sonic1Forever.as_str() {
            mod_game = Game::Sonic1Forever;
        } else if code == GameBananaURIs::Sonic2Absolute.as_str() {
            mod_game = Game::Sonic2Absolute;
        }

        if mod_game != self.game {
            return Err("Game for mod does not match selected game".into());
        }

        web::download_handler(url, "mod.zip").await?;

        Ok(())
    }
}
