use std::path::PathBuf;
use ini::Ini;
use crate::rsdk_ini::Settings;

#[derive(PartialEq, Debug)]
pub enum Game {
    Sonic1,
    Sonic2,
    SonicCD,
    SonicMania,

    None,
}

#[derive(PartialEq, Debug)]
pub struct ModInfo {
    pub name: String,
    pub author: String,
    pub version: String,
    pub enabled: bool,
}

impl Default for ModInfo {
    fn default() -> Self {
        Self { name: String::from(""), author: String::from(""), version: String::from(""), enabled: false, }
    }
}

#[derive(PartialEq)]
pub struct RSDKInfo {
    pub rsdk_revision: u8,
    pub name: Game,
    pub path: PathBuf,
    pub mods: Vec<ModInfo>,
    pub legacy: bool
}

impl Default for RSDKInfo {
    fn default() -> Self {
        Self { rsdk_revision: 4, name: Game::None , path: PathBuf::new(), mods: Vec::<ModInfo>::new(), legacy: false }
    }
}

impl RSDKInfo {
    pub fn get() -> Result<RSDKInfo, Box<dyn std::error::Error>> {
        let mut result = RSDKInfo::default();

        let settings = Settings::read_ini()?;
        result.name = settings.name;

        if let Some(parent) = settings.path.parent() {
            result.path = parent.to_path_buf();
        }

        if let Some(rsdk_name) = settings.path.file_name() {
            if let Some(rsdk_name_str) = rsdk_name.to_str() {
                match rsdk_name_str {
                    "RSDKv3" => result.rsdk_revision = 3,
                    "RSDKv4" => result.rsdk_revision = 4,
                    "RSDKv5" => result.rsdk_revision = 5,
                    "RSDKv5U" => result.rsdk_revision = 5,

                    _ => {}
                };
            }
        }

        if result.rsdk_revision == 5 && result.name != Game::SonicMania {
            result.legacy = true;
        }
    
        result.mods = result.get_mods()?;

        Ok(result)
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
                temp.enabled = modconfig_section.get(&temp.name).unwrap() == (
                    if self.rsdk_revision == 5 {
                        "y"
                    } else {
                        "true"
                    }
                );

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
}
