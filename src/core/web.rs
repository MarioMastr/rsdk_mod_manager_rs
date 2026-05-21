use crate::core::rsdk::RSDKInfo;
use crate::core::json::ManagerSettings;

use std::fs::File;
use std::io::Write;

use futures_util::StreamExt;

pub enum GameBananaURIs {
    Sonic1,
    Sonic2,
    SonicCD,
    SonicMania,

    Sonic1Forever,
    Sonic2Absolute
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
        }
    }
}

pub async fn download_handler(url: &str, name: &str) -> Result<(), Box<dyn std::error::Error>>  {
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

pub async fn gamebanana_uri_handler(uri: &str) -> Result<(), Box<dyn std::error::Error>> {
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
