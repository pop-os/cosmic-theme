// SPDX-License-Identifier: MPL-2.0-only

use crate::{Hex, Theme, NAME, THEME_DIR};
use anyhow::{bail, Result};
use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

/// Cosmic Theme config
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// whether high contrast mode is activated
    pub is_high_contrast: bool,
    /// active
    pub is_dark: bool,
    /// Selected light theme name
    pub light: String,
    /// Selected dark theme name
    pub dark: String,
}

impl Default for Config {
    fn default() -> Self {
        // TODO load gsettings to determine if dark light or high contrast?
        Self {
            is_dark: true,
            light: Default::default(),
            dark: Default::default(),
            is_high_contrast: Default::default(),
        }
    }
}

/// name of the config file
pub const CONFIG_NAME: &str = "config";

impl Config {
    /// create a new cosmic theme config
    pub fn new(is_dark: bool, high_contrast: bool, light: String, dark: String) -> Self {
        Self {
            is_dark,
            light,
            dark,
            is_high_contrast: high_contrast,
        }
    }

    /// save the cosmic theme config
    pub fn save(&self) -> Result<()> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        if let Ok(path) = xdg_dirs.place_config_file(PathBuf::from(format!("{CONFIG_NAME}.ron"))) {
            let mut f = File::create(path)?;
            let ron = ron::ser::to_string_pretty(&self, Default::default())?;
            f.write_all(ron.as_bytes())?;
            Ok(())
        } else {
            bail!("failed to save theme config")
        }
    }

    /// init the config directory
    pub fn init() -> anyhow::Result<PathBuf> {
        let base_dirs = xdg::BaseDirectories::new()?;
        Ok(base_dirs.create_config_directory(NAME)?)
    }

    /// load the cosmic theme config
    pub fn load() -> Result<Self> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        let path = xdg_dirs.get_config_home();
        std::fs::create_dir_all(&path)?;
        let path = xdg_dirs.find_config_file(PathBuf::from(format!("{CONFIG_NAME}.ron")));
        if path.is_none() {
            let s = Self::default();
            s.save()?;
        }
        if let Some(path) = xdg_dirs.find_config_file(PathBuf::from(format!("{CONFIG_NAME}.ron"))) {
            let mut f = File::open(&path)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(ron::from_str(s.as_str())?)
        } else {
            anyhow::bail!("Failed to load config")
        }
    }

    /// get the name of the active theme
    pub fn active_name(&self) -> Option<String> {
        if self.is_dark && self.dark.is_empty() {
            Some(self.dark.clone())
        } else if !self.is_dark && !self.light.is_empty() {
            Some(self.light.clone())
        } else {
            None
        }
        // if *high_contrast {
        //     if let Some(palette) = palette.take() {
        //         // TODO enforce high contrast constraints
        //         *palette = palette.to_high_contrast();
        //         todo!()
        //     }
        // }
    }

    /// get the active theme
    pub fn get_active<C>(&self) -> anyhow::Result<Theme<C>>
    where
        C: Clone
            + fmt::Debug
            + Default
            + Into<Hex>
            + Into<Srgba>
            + From<Srgba>
            + Serialize
            + DeserializeOwned,
    {
        let active = match self.active_name() {
            Some(n) => n,
            _ => anyhow::bail!("No configured active overrides"),
        };
        let css_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let css_dirs = xdg::BaseDirectories::with_prefix(css_path)?;
        let active_theme_path = match css_dirs.find_data_file(format!("{active}.ron")) {
            Some(p) => p,
            _ => anyhow::bail!("Could not find theme"),
        };
        let active_theme_file = File::open(active_theme_path)?;
        let reader = BufReader::new(active_theme_file);

        let colors = ron::de::from_reader::<_, Theme<C>>(reader)?;
        Ok(colors)
    }

    /// set the name of the active light theme
    pub fn set_active_light(new: &str) -> Result<()> {
        let mut self_ = Self::load()?;

        self_.light = new.to_string();

        self_.save()
    }

    /// set the name of the active dark theme
    pub fn set_active_dark(new: &str) -> Result<()> {
        let mut self_ = Self::load()?;

        self_.dark = new.to_string();

        self_.save()
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
        + Serialize
        + DeserializeOwned,
{
    fn from((light, dark): (Theme<C>, Theme<C>)) -> Self {
        Self {
            light: light.name,
            dark: dark.name,
            is_dark: true,
            is_high_contrast: false,
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
        + Serialize
        + DeserializeOwned,
{
    fn from(t: Theme<C>) -> Self {
        Self {
            light: t.clone().name,
            dark: t.name,
            is_dark: true,
            is_high_contrast: true,
        }
    }
}
