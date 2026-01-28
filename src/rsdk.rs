use std::{ffi::OsString, path::{self, PathBuf}, str::FromStr};

use ini::Ini;

#[derive(PartialEq, Debug)]
pub enum Game {
    Sonic1,
    Sonic2,
    SonicCD,
    SonicMania,

    None,
}

pub struct GameInfo {
    pub rsdk_revision: u8,
    pub name: Game,
    pub path: PathBuf,
    pub mods: Vec<ModInfo>,
}

impl Default for GameInfo {
    fn default() -> Self {
        Self { rsdk_revision: 4, name: Game::None , path: PathBuf::new(), mods: Vec::<ModInfo>::new()}
    }
}

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

pub fn get_game_info(settings: crate::Settings) -> Result<GameInfo, Box<dyn std::error::Error>> {
    let mut result = GameInfo::default();

    result.name = settings.name;
    result.path = settings.path.parent().unwrap().to_path_buf();

    if result.name == Game::SonicCD {
        result.rsdk_revision = 3;
    } else if result.name == Game::Sonic1 || result.name == Game::Sonic2  {
        result.rsdk_revision = 4;
    } else if result.name == Game::SonicMania {
        result.rsdk_revision = 5;
    }
    
    result.mods = get_mods(&result)?;

    Ok(result)
}

pub fn get_mods(game: &GameInfo) -> Result<Vec<ModInfo>, Box<dyn std::error::Error>> {
    let mut result = Vec::<ModInfo>::new();
    let mods_path = game.path.join("mods");
    let mods_dir = mods_path.read_dir()?;

    let modconfig_ini_path = mods_path.join("modconfig.ini");
    let modconfig_ini = Ini::load_from_file(modconfig_ini_path)?;

    let modconfig_section = modconfig_ini.section(Some("mods")).unwrap();

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
            if game.rsdk_revision == 5 {
                "y"
            } else {
                "true"
            }
        );

        result.push(temp);
    };

    Ok(result)
}