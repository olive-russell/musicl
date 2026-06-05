// use anyhow::{Context, Result};
// use dirs_next::config_dir;
// use serde::{Deserialize, Serialize};
// use std::{fs, path::PathBuf};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Config {
//     pub db: Option<PathBuf>,
// }

// impl Config {
//     /// Load config from ~/.config/musicl/config.toml
//     pub fn load() -> Result<Self> {
//         let mut path = config_dir().context("could not find config dir")?;
//         path.push("musicl/config.toml");

//         if path.exists() {
//             let data = fs::read_to_string(&path)?;
//             Ok(toml::from_str(&data)?)
//         } else {
//             Ok(Config { db: None })
//         }
//     }

//     /// Save config to ~/.config/musicl/config.toml
//     pub fn save(&self) -> Result<()> {
//         let mut path = config_dir().unwrap();
//         path.push("musicl");
//         fs::create_dir_all(&path)?;  // ensure folder exists
//         path.push("config.toml");

//         let data = toml::to_string_pretty(self)?;
//         fs::write(&path, data)?;
//         Ok(())
//     }
// }
