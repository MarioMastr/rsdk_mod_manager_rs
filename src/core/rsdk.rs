use std::{error::Error, fs::File, io::Write, os::unix::fs::FileExt, path::PathBuf, process::ExitStatus, str::FromStr};
use ini::Ini;
use native_dialog::DialogBuilder;
use serde::{Deserialize, Serialize};
use archive::{ArchiveExtractor, ArchiveFormat};
use tokio::process::Command;

use crate::core::{json::ManagerSettings, web};

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
    pub id: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub enabled: bool,
    pub selected: bool
}

#[derive(PartialEq, Clone, Default)]
pub struct RSDKInfo {
    pub rsdk_revision: u8,
    pub game: Game,
    pub name: String,
    pub path: PathBuf,
    pub mods: Vec<ModInfo>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(input: &str) -> Result<Game, Self::Err> {
        match input {
            "s1mm:" => Ok(Game::Sonic1),
            "s2mm:" => Ok(Game::Sonic2),
            "scdmm:" => Ok(Game::SonicCD),
            "smmm:" => Ok(Game::SonicMania),
            "s1fmm:" => Ok(Game::Sonic1Forever),
            "s2amm:" => Ok(Game::Sonic2Absolute),
            _ => Err(()),
        }
    }
}

impl Game {
    pub fn as_str(&self) -> &'static str {
        match self {
            Game::Sonic1 => "s1mm:",
            Game::Sonic2 => "s2mm:",
            Game::SonicCD => "scdmm:",
            Game::SonicMania => "smmm:",
            Game::Sonic1Forever => "s1fmm:",
            Game::Sonic2Absolute => "s2amm:",
            Game::None => "",
        }
    }
}

impl RSDKInfo {
    pub fn get(manager_settings: &ManagerSettings) -> Result<RSDKInfo, Box<dyn Error>> {
        let mut result = RSDKInfo::default();

        let game_settings = &manager_settings.games[manager_settings.selected_game];

        result.game = game_settings.name;

        if let Some(parent) = game_settings.path.parent() {
            result.path = parent.to_path_buf();
        }

        if let Some(rsdk_name) = game_settings.path.file_prefix() && let Some(rsdk_name_str) = rsdk_name.to_str() {
            result.name = rsdk_name_str.to_string();
        }

        result.rsdk_revision = result.get_version()?;
        result.mods = result.get_mods()?;

        Ok(result)
    }

    pub fn read_size<T>(&self, file: &mut File, swap: bool, start: u64) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = vec![0; size_of::<T>()];
        file.read_exact_at(&mut buffer, start)?;

        if cfg!(target_endian = "big") || swap {
            buffer.reverse();
        }

        Ok(buffer)
    }

    pub fn get_version(&self) -> Result<u8, Box<dyn Error>> {
        let data_rsdk_path = self.path.join("Data.rsdk");
        let data_folder_path = self.path.join("Data");

        if data_rsdk_path.exists() {
            let rsdk_data = std::fs::read(data_rsdk_path)?;

            if rsdk_data[5] == 66 { // B in ASCII; RSDKvB = RSDKv4
                return Ok(4);
            } else {
                return Ok(rsdk_data[5]);
            }
        } else if data_folder_path.exists() {
            let game_config_path = data_folder_path.join("Game").join("GameConfig.bin");
            let mut game_config_file = File::open(game_config_path)?;

            let mut sig = self.read_size::<i32>(&mut game_config_file, false, 0)?;
            if std::str::from_utf8(sig.as_slice())? == "CFG\0" {
                return Ok(5);
            } else {
                sig = self.read_size::<i32>(&mut game_config_file, false, 9)?;
                if std::str::from_utf8(sig.as_slice())? == "Data" {
                    return Ok(3);
                } else {
                    return Ok(4);
                }
            }
        }

        Ok(4)
    }

    pub fn refresh(&mut self, manager_settings: &ManagerSettings) -> Result<(), Box<dyn Error>> {
        *self = RSDKInfo::get(manager_settings)?;

        Ok(())
    }

    pub fn get_mods(&self) -> Result<Vec<ModInfo>, Box<dyn Error>> {
        let mut result = Vec::<ModInfo>::new();
        let mods_path = self.path.join("mods");

        if !mods_path.exists() {
            return Ok(result);
        }

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
                if !mod_ini_path.exists() {
                    continue;
                }

                let mod_ini = Ini::load_from_file(mod_ini_path)?;

                let section = mod_ini.section(None::<String>).unwrap();

                temp.name = entry.file_name().into_string().unwrap();
                temp.id = section.get("Name").unwrap().to_string(); 
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

                result.push(temp);
            };
        }

        Ok(result)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let mods_path = self.path.join("mods");
        if !mods_path.exists() {
            return Ok(());
        }
        let modconfig_ini_path = mods_path.join("modconfig.ini");

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

        modconfig_ini_new.write_to_file(modconfig_ini_path).map_err(|e| e.into())
    }

    pub fn new_mod(
        &mut self,
        method: NewMod,
        name: Option<String>,
        author: Option<String>,
        ver: Option<String>,
        desc: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        match method {
            NewMod::Archive => {
                if let Some(archive) = DialogBuilder::file()
                    .add_filter("Mod Archives", ["zip", "7z"])
                    .open_single_file()
                    .show()?
                {
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

                    // we pass through twice, handling directories and then non-directories to prevent errors
                    for file in &files {
                        if file.is_directory {
                            let dest = mods_directory.join(&file.path);
                            if !dest.exists() {
                                std::fs::create_dir(dest)?;
                            } else {
                                return Err("Mod already exists".into());
                            }
                        }
                    }

                    for file in &files {
                        if !file.is_directory {
                            let mut desired_file = std::fs::File::create(mods_directory.join(&file.path))?;
                            desired_file.write_all(&file.data)?;
                        }
                    }
                }
            },
            NewMod::Folder => {
                if let Some(folder) = DialogBuilder::file()
                    .open_single_dir()
                    .show()?
                {
                    let mods_directory = self.path.join("mods");
                    if let Some(name) = folder.file_name() {
                        let desired_mod_dir = mods_directory.join(name);
                        std::fs::rename(folder, desired_mod_dir)?;
                    }
                }
            },
            NewMod::Scratch => {
                if let (Some(name), Some(author), Some(ver)) = (name, author, ver) {
                    let mods_directory = self.path.join("mods");
                    let desired_mod_dir = mods_directory.join(&name);
                    std::fs::create_dir(&desired_mod_dir)?;

                    let mod_ini_path = desired_mod_dir.join("mod.ini");
                    let mut mod_ini = Ini::new();
                    let mut section = mod_ini.with_section(None::<String>);
                    section.set("Name", &name);
                    section.set("Author", &author);
                    section.set("Version", &ver);

                    if let Some(description) = desc && !description.is_empty() {
                        section.set("Description", &description);
                    }

                    mod_ini.write_to_file(mod_ini_path)?;
                } else {
                    return Err("Missing required fields for new mod".into());
                }
            },
        }

        Ok(())
    }

    pub async fn new_mod_online(
        &mut self,
        code: &str,
        url: &str
    ) -> Result<(), Box<dyn Error>> {
        let mod_game = Game::from_str(code).map_err(|_| "Invalid game code for mod")?;

        if mod_game != self.game {
            return Err("Game for mod does not match selected game".into());
        }

        let os_cache = dirs::cache_dir().ok_or("Unable to get cache directory")?;
        let app_cache = os_cache.join("rmm");

        let temp_path = app_cache.join("temp");
        let path = temp_path.join("mod.zip");

        web::download_handler(url, "mod.zip").await?;

        let data = std::fs::read(&path)?;
        let extractor = ArchiveExtractor::new();
        let mut files_res = extractor.extract(&data, ArchiveFormat::Zip);

        if files_res.is_err() {
            files_res = extractor.extract(&data, ArchiveFormat::SevenZ);
        }

        let files = files_res?;

        let mods_directory = self.path.join("mods");

        // we pass through twice, handling directories and then non-directories to prevent errors
        for file in &files {
            if file.is_directory {
                let dest = mods_directory.join(&file.path);
                if !dest.exists() {
                    std::fs::create_dir(dest)?;
                } else {
                    return Err("Mod already exists".into());
                }
            }
        }

        for file in files {
            if !file.is_directory {
                let mut desired_file = std::fs::File::create(mods_directory.join(file.path))?;
                desired_file.write_all(&file.data)?;
            }
        }

        std::fs::remove_dir_all(temp_path).map_err(|e| e.into())
    }

    pub fn remove_mod(&mut self, selected_mod: usize) -> Result<(), Box<dyn Error>> {
        let mod_info = &self.mods[selected_mod];
        let dir_to_del = self.path.join("mods").join(&mod_info.name);

        std::fs::remove_dir_all(dir_to_del).map_err(|e| e.into())
    }

    pub async fn launch(&self) -> Result<ExitStatus, String> {
        #[cfg(target_os = "macos")]
            let extension = ".app";
        #[cfg(target_os = "windows")]
            let extension = ".exe";
        #[cfg(target_os = "linux")]
            let extension = "";

        let game_path = self.name.to_owned() + extension;

        #[cfg(target_os = "macos")]
            let mut child = Command::new("open").arg("-a").arg(game_path).current_dir(&self.path).spawn().map_err(|e| e.to_string())?;
        #[cfg(target_os = "windows")]
            let mut child = Command::new("start").arg(game_path).current_dir(&self.path).spawn().map_err(|e| e.to_string())?;
        #[cfg(target_os = "linux")]
            let mut child = Command::new("./".to_owned() + game_path.as_str()).current_dir(&self.path).spawn().map_err(|e| e.to_string())?;

        child.wait().await.map_err(|e| e.to_string())
    }
}
