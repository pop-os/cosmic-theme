// SPDX-License-Identifier: GPL-3.0-only

use crate::{
    util::CssColor, Component, ComponentType, Container, ContainerType, CosmicPalette,
    DARK_PALETTE, LIGHT_PALETTE, NAME, THEME_DIR,
};
use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

/// Cosmic Theme data structure with all colors and its name
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Theme<C> {
    /// name of the theme
    pub name: String,
    /// background element colors
    pub background: Container<C>,
    /// primary element colors
    pub primary: Container<C>,
    /// secondary element colors
    pub secondary: Container<C>,
    /// accent element colors
    pub accent: Component<C>,
    /// suggested element colors
    pub success: Component<C>,
    /// destructive element colors
    pub destructive: Component<C>,
    /// warning element colors
    pub warning: Component<C>,
}

// TODO better eq check
impl<C> PartialEq for Theme<C> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<C> Theme<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    /// create a new theme from its elements
    pub fn new(
        background: Container<C>,
        primary: Container<C>,
        secondary: Container<C>,
        accent: Component<C>,
        destructive: Component<C>,
        warning: Component<C>,
        success: Component<C>,
    ) -> Self {
        Self {
            background,
            primary,
            secondary,
            accent,
            destructive,
            warning,
            success,
            ..Default::default()
        }
    }

    /// Convert the theme to a high-contrast variant
    pub fn to_high_contrast(&self) -> Self {
        todo!();
    }

    /// save the theme to the theme directory
    pub fn save(&self) -> anyhow::Result<()> {
        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let ron_dirs = xdg::BaseDirectories::with_prefix(ron_path)?;
        let ron_name = format!("{}.ron", &self.name);

        if let Ok(p) = ron_dirs.place_config_file(ron_name) {
            let mut f = File::create(p)?;
            f.write_all(ron::ser::to_string_pretty(self, Default::default())?.as_bytes())?;
        } else {
            anyhow::bail!("Failed to write RON theme.");
        }
        Ok(())
    }

    /// init the theme directory
    pub fn init() -> anyhow::Result<PathBuf> {
        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let base_dirs = xdg::BaseDirectories::new()?;
        Ok(base_dirs.create_config_directory(ron_path)?)
    }

    /// load a theme by name
    pub fn load_from_name(name: &str) -> anyhow::Result<Self> {
        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let ron_dirs = xdg::BaseDirectories::with_prefix(ron_path)?;

        let ron_name = format!("{}.ron", name);
        if let Some(p) = ron_dirs.find_config_file(ron_name) {
            let f = File::open(p)?;
            Ok(ron::de::from_reader(f)?)
        } else {
            anyhow::bail!("Failed to write RON theme.");
        }
    }

    /// load a theme by path
    pub fn load(p: &dyn AsRef<Path>) -> anyhow::Result<Self> {
        let f = File::open(p)?;
        Ok(ron::de::from_reader(f)?)
    }

    // TODO convenient getter functions for each named color variable
    // return Srgba
}

impl Theme<CssColor> {
    /// get the built in light theme
    pub fn light_default() -> Self {
        LIGHT_PALETTE.clone().into()
    }

    /// get the built in dark theme
    pub fn dark_default() -> Self {
        DARK_PALETTE.clone().into()
    }
}

impl<C> From<CosmicPalette<C>> for Theme<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    fn from(p: CosmicPalette<C>) -> Self {
        Self {
            name: p.name().to_string(),
            background: (p.clone(), ContainerType::Background).into(),
            primary: (p.clone(), ContainerType::Background).into(),
            secondary: (p.clone(), ContainerType::Background).into(),
            accent: (p.clone(), ComponentType::Accent).into(),
            success: (p.clone(), ComponentType::Success).into(),
            destructive: (p.clone(), ComponentType::Destructive).into(),
            warning: (p.clone(), ComponentType::Warning).into(),
        }
    }
}
