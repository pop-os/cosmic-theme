use crate::{Hex, Theme, NAME};
use anyhow::{bail, Result};
use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fmt,
    fs::{create_dir_all, File},
    io::prelude::*,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub light: String,
    pub dark: String,
    pub is_light: bool, // is light theme if true
}

pub const CONFIG_NAME: &'static str = "config.ron";

impl Config {
    pub fn new(light: String, dark: String) -> Self {
        Self {
            light,
            dark,
            is_light: false,
        }
    }

    pub fn save(&self) -> Result<()> {
        let p = Self::config_path()?;
        create_dir_all(&p)?;
        let mut f = File::create(p)?;
        let toml = toml::ser::to_string_pretty(&self)?;
        f.write_all(toml.as_bytes())?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let p = Self::config_path()?;
        let mut f = File::open(p)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(toml::from_str(s.as_str())?)
    }

    pub fn config_path() -> Result<PathBuf> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        if let Some(path) = xdg_dirs.find_config_file(PathBuf::from(CONFIG_NAME)) {
            Ok(path)
        } else {
            bail!("no theme config");
        }
    }

    pub fn active_name(&self) -> String {
        if self.is_light {
            self.light.clone()
        } else {
            self.dark.clone()
        }
    }
}

impl<C> From<(Theme<C>, Theme<C>)> for Config
where
    C: Copy
        + Clone
        + fmt::Debug
        + Default
        + Into<Hex>
        + Into<Srgba>
        + From<Srgba>
        + fmt::Display
        + Serialize
        + DeserializeOwned,
{
    fn from((light, dark): (Theme<C>, Theme<C>)) -> Self {
        Self {
            light: light.name,
            dark: dark.name,
            is_light: false,
        }
    }
}

impl<C> From<Theme<C>> for Config
where
    C: Copy
        + Clone
        + fmt::Debug
        + Default
        + Into<Hex>
        + Into<Srgba>
        + From<Srgba>
        + fmt::Display
        + Serialize
        + DeserializeOwned,
{
    fn from(t: Theme<C>) -> Self {
        Self {
            light: t.clone().name,
            dark: t.name,
            is_light: false,
        }
    }
}
