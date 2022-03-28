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
    light: String,
    dark: String,
}

const CONFIG_NAME: &'static str = "config.ron";

impl Config {
    pub fn new(light: String, dark: String) -> Self {
        Self { light, dark }
    }

    pub fn save(&self) -> Result<()> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        let relative_path = PathBuf::from(CONFIG_NAME);
        if let Ok(p) = xdg_dirs.place_config_file(relative_path) {
            create_dir_all(&p)?;
            let mut f = File::create(p)?;
            let toml = toml::ser::to_string_pretty(&self)?;
            f.write_all(toml.as_bytes())?;
            Ok(())
        } else {
            bail!("No home directory.")
        }
    }

    pub fn load<P: AsRef<Path>>() -> Result<Self> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        let relative_path = PathBuf::from(CONFIG_NAME);
        if let Some(p) = xdg_dirs.find_config_file(relative_path) {
            let mut f = File::open(p)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(toml::from_str(s.as_str())?)
        } else {
            bail!("No home directory.")
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
        }
    }
}
