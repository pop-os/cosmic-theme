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
    /// get @accent_color
    pub fn accent_color(&self) -> Srgba {
        self.accent.base.clone().into()
    }
    /// get @success_color
    pub fn success_color(&self) -> Srgba {
        self.success.base.clone().into()
    }
    /// get @destructive_color
    pub fn destructive_color(&self) -> Srgba {
        self.destructive.base.clone().into()
    }
    /// get @warning_color
    pub fn warning_color(&self) -> Srgba {
        self.warning.base.clone().into()
    }

    // Containers
    /// get @bg_color
    pub fn bg_color(&self) -> Srgba {
        self.background.base.clone().into()
    }
    /// get @bg_component_color
    pub fn bg_component_color(&self) -> Srgba {
        self.background.component.base.clone().into()
    }
    /// get @primary_container_color
    pub fn primary_container_color(&self) -> Srgba {
        self.primary.base.clone().into()
    }
    /// get @primary_component_color
    pub fn primary_component_color(&self) -> Srgba {
        self.primary.component.base.clone().into()
    }
    /// get @secondary_container_color
    pub fn secondary_container_color(&self) -> Srgba {
        self.secondary.base.clone().into()
    }
    /// get @secondary_component_color
    pub fn secondary_component_color(&self) -> Srgba {
        self.secondary.component.base.clone().into()
    }

    // Text
    /// get @on_bg_color
    pub fn on_bg_color(&self) -> Srgba {
        self.background.on.clone().into()
    }
    /// get @on_bg_component_color
    pub fn on_bg_component_color(&self) -> Srgba {
        self.background.component.on.clone().into()
    }
    /// get @on_primary_color
    pub fn on_primary_container_color(&self) -> Srgba {
        self.primary.on.clone().into()
    }
    /// get @on_primary_component_color
    pub fn on_primary_component_color(&self) -> Srgba {
        self.primary.component.on.clone().into()
    }
    /// get @on_secondary_color
    pub fn on_secondary_container_color(&self) -> Srgba {
        self.secondary.on.clone().into()
    }
    /// get @on_secondary_component_color
    pub fn on_secondary_component_color(&self) -> Srgba {
        self.secondary.component.on.clone().into()
    }
    /// get @accent_text_color
    pub fn accent_text_color(&self) -> Srgba {
        self.accent.base.clone().into()
    }
    /// get @success_text_color
    pub fn success_text_color(&self) -> Srgba {
        self.success.base.clone().into()
    }
    /// get @warning_text_color
    pub fn warning_text_color(&self) -> Srgba {
        self.warning.base.clone().into()
    }
    /// get @destructive_text_color
    pub fn destructive_text_color(&self) -> Srgba {
        self.destructive.base.clone().into()
    }
    /// get @on_accent_color
    pub fn on_accent_color(&self) -> Srgba {
        self.accent.on.clone().into()
    }
    /// get @on_success_color
    pub fn on_success_color(&self) -> Srgba {
        self.success.on.clone().into()
    }
    /// get @oon_warning_color
    pub fn oon_warning_color(&self) -> Srgba {
        self.warning.on.clone().into()
    }
    /// get @on_destructive_color
    pub fn on_destructive_color(&self) -> Srgba {
        self.destructive.on.clone().into()
    }

    // Borders and Dividers
    /// get @bg_divider
    pub fn bg_divider(&self) -> Srgba {
        self.background.divider.clone().into()
    }
    /// get @bg_component_divider
    pub fn bg_component_divider(&self) -> Srgba {
        self.background.component.divider.clone().into()
    }
    /// get @primary_container_divider
    pub fn primary_container_divider(&self) -> Srgba {
        self.primary.divider.clone().into()
    }
    /// get @primary_component_divider
    pub fn primary_component_divider(&self) -> Srgba {
        self.primary.component.divider.clone().into()
    }
    /// get @secondary_container_divider
    pub fn secondary_container_divider(&self) -> Srgba {
        self.secondary.divider.clone().into()
    }
    /// get @secondary_component_divider
    pub fn secondary_component_divider(&self) -> Srgba {
        self.secondary.component.divider.clone().into()
    }

    /// get @window_header_bg
    pub fn window_header_bg(&self) -> Srgba {
        self.background.base.clone().into()
    }
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

    /// convert to srgba
    pub fn into_srgba(self) -> Theme<Srgba> {
        Theme {
            name: self.name,
            background: self.background.into_srgba(),
            primary: self.primary.into_srgba(),
            secondary: self.secondary.into_srgba(),
            accent: self.accent.into_srgba(),
            success: self.success.into_srgba(),
            destructive: self.destructive.into_srgba(),
            warning: self.warning.into_srgba(),
        }
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
