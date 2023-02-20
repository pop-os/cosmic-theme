// SPDX-License-Identifier: GPL-3.0-only

use crate::{
    util::CssColor, Component, ComponentType, Container, ContainerType, CosmicPalette,
    DARK_PALETTE, LIGHT_PALETTE, NAME, THEME_DIR,
};
use anyhow::Context;
use directories::{BaseDirsExt, ProjectDirsExt};
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
    /// default element
    pub basic: Component<C>,
    /// accent element colors
    pub accent: Component<C>,
    /// suggested element colors
    pub success: Component<C>,
    /// destructive element colors
    pub destructive: Component<C>,
    /// warning element colors
    pub warning: Component<C>,
    /// generic text color
    pub on: C,
    /// generic divider color
    pub divider: C,
    /// on disabled
    pub on_disabled: C,
    /// on accent
    pub on_accent: C,
    /// on success
    pub on_success: C,
    /// on destructive
    pub on_destructive: C,
    /// on warning
    pub on_warning: C,
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
        let ron_dirs = directories::ProjectDirs::from_path(ron_path)
            .context("Failed to get project directories.")?;
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
        let base_dirs = directories::BaseDirs::new().context("Failed to get base directories.")?;
        Ok(base_dirs.create_config_directory(ron_path)?)
    }

    /// load a theme by name
    pub fn load_from_name(name: &str) -> anyhow::Result<Self> {
        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let ron_dirs = directories::ProjectDirs::from_path(ron_path)
            .context("Failed to get project directories.")?;

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

    /// get @primary_container_color
    pub fn primary_container_color(&self) -> Srgba {
        self.primary.base.clone().into()
    }
    /// get @secondary_container_color
    pub fn secondary_container_color(&self) -> Srgba {
        self.secondary.base.clone().into()
    }

    /// get @divider_color
    pub fn divider_color(&self) -> Srgba {
        self.divider.clone().into()
    }

    // Text
    /// get @on_color for all regular widgets and containers
    pub fn on_color(&self) -> Srgba {
        self.on.clone().into()
    }

    /// get @on_disabled_color for all widgets that can be disabled
    pub fn on_disabled_color(&self) -> Srgba {
        self.on_disabled.clone().into()
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
        self.on_accent.clone().into()
    }
    /// get @on_success_color
    pub fn on_success_color(&self) -> Srgba {
        self.on_success.clone().into()
    }
    /// get @oon_warning_color
    pub fn on_warning_color(&self) -> Srgba {
        self.on_warning.clone().into()
    }
    /// get @on_destructive_color
    pub fn on_destructive_color(&self) -> Srgba {
        self.on_destructive.clone().into()
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

    /// get the built in high contrast dark theme
    pub fn high_contrast_dark_default() -> Self {
        CosmicPalette::HighContrastDark(DARK_PALETTE.as_ref().clone()).into()
    }

    /// get the built in high contrast light theme
    pub fn high_contrast_light_default() -> Self {
        CosmicPalette::HighContrastLight(LIGHT_PALETTE.as_ref().clone()).into()
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
            on: self.on.into(),
            divider: self.divider.into(),
            on_disabled: self.on_disabled.into(),
            on_accent: self.on_accent.into(),
            on_success: self.on_success.into(),
            on_destructive: self.on_destructive.into(),
            on_warning: self.on_warning.into(),
            basic: self.basic.into_srgba(),
        }
    }
}

impl<C> From<CosmicPalette<C>> for Theme<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    fn from(p: CosmicPalette<C>) -> Self {
        let neutral_10 = match &p {
            CosmicPalette::Dark(p) => p.neutral_10.clone(),
            CosmicPalette::Light(p) => p.neutral_10.clone(),
            CosmicPalette::HighContrastLight(p) => p.neutral_10.clone(),
            CosmicPalette::HighContrastDark(p) => p.neutral_10.clone(),
        };
        let neutral_1 = match &p {
            CosmicPalette::Dark(p) => p.neutral_1.clone(),
            CosmicPalette::Light(p) => p.neutral_1.clone(),
            CosmicPalette::HighContrastLight(p) => p.neutral_1.clone(),
            CosmicPalette::HighContrastDark(p) => p.neutral_1.clone(),
        };
        let (on, divider) = if p.is_high_contrast() {
            let mut divider: Srgba = neutral_10.clone().into();
            divider.alpha = 0.5;
            (neutral_10.clone(), divider.into())
        } else {
            let mut divider: Srgba = neutral_10.clone().into();
            divider.alpha = 0.2;
            (neutral_10.clone(), divider.into())
        };
        // TODO Ashley does this change for the high contrast variants?
        let mut on_disabled: Srgba = neutral_10.clone().into();
        on_disabled.alpha = 0.5;
        Self {
            name: p.name().to_string(),
            background: (p.clone(), ContainerType::Background).into(),
            primary: (p.clone(), ContainerType::Primary).into(),
            secondary: (p.clone(), ContainerType::Secondary).into(),
            accent: (p.clone(), ComponentType::Accent).into(),
            success: (p.clone(), ComponentType::Success).into(),
            destructive: (p.clone(), ComponentType::Destructive).into(),
            warning: (p.clone(), ComponentType::Warning).into(),
            basic: (p.clone(), ComponentType::Basic).into(),
            on,
            divider,
            on_disabled: on_disabled.into(),
            on_accent: neutral_1.clone(),
            on_success: neutral_1.clone(),
            on_destructive: neutral_1.clone(),
            on_warning: neutral_1,
        }
    }
}
