use crate::{Hex, Theme, NAME};
use anyhow::{bail, Result};
use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt, fs::File, io::prelude::*, path::PathBuf};

/// Cosmic Theme config
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Selected light theme name
    pub light: String,
    /// Selected dark theme name
    pub dark: String,
    /// Selected dark or light theme
    pub is_light: bool, // is light theme if true
}

pub(crate) const CONFIG_NAME: &'static str = "config.ron";

impl Config {
    /// create a new cosmic theme config
    pub fn new(light: String, dark: String) -> Self {
        Self {
            light,
            dark,
            is_light: false,
        }
    }

    /// save the cosmic theme config
    pub fn save(&self) -> Result<()> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        if let Ok(path) = xdg_dirs.place_config_file(PathBuf::from(CONFIG_NAME)) {
            let mut f = File::create(path)?;
            let toml = toml::ser::to_string_pretty(&self)?;
            f.write_all(toml.as_bytes())?;
            Ok(())
        } else {
            bail!("failed to save theme config")
        }
    }

    /// load the cosmic theme config
    pub fn load() -> Result<Self> {
        let p = Self::config_path()?;
        let mut f = File::open(p)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(toml::from_str(s.as_str())?)
    }

    /// get the path of the cosmic theme config
    pub fn config_path() -> Result<PathBuf> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        if let Some(path) = xdg_dirs.find_config_file(PathBuf::from(CONFIG_NAME)) {
            Ok(path)
        } else {
            dbg!(xdg_dirs.get_config_home());
            dbg!(xdg_dirs.get_config_dirs());
            bail!("no theme config");
        }
    }

    /// get the name of the active theme
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
