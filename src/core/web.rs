use crate::core::{rsdk::{RSDKInfo, Game}, json::ManagerSettings};
use std::{fs::File, io::Write, error::Error};

use futures_util::StreamExt;

#[cfg(target_os = "windows")]
use registry::{Hive, Security};

pub enum GameBananaURIs {
    Sonic1,
    Sonic2,
    SonicCD,
    SonicMania,

    Sonic1Forever,
    Sonic2Absolute,

    None
}

impl GameBananaURIs {
    pub fn as_str(&self) -> &'static str {
        match self {
            GameBananaURIs::Sonic1 => "s1mm:",
            GameBananaURIs::Sonic2 => "s2mm:",
            GameBananaURIs::SonicCD => "scdmm:",
            GameBananaURIs::SonicMania => "smmm:",
            GameBananaURIs::Sonic1Forever => "s1fmm:",
            GameBananaURIs::Sonic2Absolute => "s2amm:",
            GameBananaURIs::None => "",
        }
    }
}

pub async fn download_handler(url: &str, name: &str) -> Result<(), Box<dyn Error>>  {
    let res = reqwest::get(url).await.or(Err(format!("Failed to GET from '{}'", &url)))?;

    let temp_path = std::env::current_dir()?.join("temp");
    if !temp_path.exists() {
        std::fs::create_dir(&temp_path)?;
    }

    let path = temp_path.join(name);

    // download chunks
    let mut file = File::create(&path).or(Err(format!("Failed to create file '{}'", path.display())))?;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file"))?;
        file.write_all(&chunk)
            .or(Err("Error while writing to file"))?;
    }

    Ok(())
}

pub async fn gamebanana_uri_handler(uri: &str) -> Result<(), Box<dyn Error>> {
    if let Some(index) = uri.find("https") {
        let uri_split = uri.split_at(index);
        let uri_parts: Vec<&str> = uri_split.1.split(",").collect();

        let download_url = uri_parts[0];
        let code = uri_split.0;

        let manager = ManagerSettings::read_json()?;
        let mut game = RSDKInfo::get(&manager)?;

        game.new_mod_online(code, download_url).await?;
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn windows_install_uri(game: GameBananaURIs) {
    let uri_str = game.as_str();
    let regkey = Hive::CurrentUser.open(format!("HKEY_CLASSES_ROOTf\\{uri_str}"), Security::Read)?;
}

pub fn get_uri(game: Game) -> GameBananaURIs {
    match game {
        Game::Sonic1 => GameBananaURIs::Sonic1,
        Game::Sonic2 => GameBananaURIs::Sonic2,
        Game::SonicCD => GameBananaURIs::SonicCD,
        Game::SonicMania=> GameBananaURIs::SonicMania,
        Game::Sonic1Forever=> GameBananaURIs::Sonic1Forever,
        Game::Sonic2Absolute=> GameBananaURIs::Sonic2Absolute,
        Game::None => GameBananaURIs::None,
    }
}
