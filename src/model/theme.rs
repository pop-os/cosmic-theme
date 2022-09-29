// SPDX-License-Identifier: GPL-3.0-only

use crate::{Accent, Container, NAME, THEME_DIR, CosmicPalette, Widget};
use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt, path::{PathBuf, Path}, fs::File, io::Write};

/// Cosmic Theme data structure with all colors and its name
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Theme<C> {
    /// name of the theme
    pub name: String,
    /// palette of the theme
    pub palette: CosmicPalette<C>,
    /// background element colors
    pub background: Container<C>,
    /// primary element colors
    pub primary: Container<C>,
    /// secondary element colors
    pub secondary: Container<C>,
    /// accent element colors
    pub accent: Accent<C>,
    /// suggested element colors
    pub success: Widget<C>,
    /// destructive element colors
    pub destructive: Widget<C>,
    /// warning element colors
    pub warning: Widget<C>,

    // TODO derived surface colors which don't fit neatly in a category
    /// window header background color
    pub window_header_background: C,
    /// text button text color
    pub text_button_text: C,
}

// TODO better eq check
impl<C> PartialEq for Theme<C> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<C> Theme<C>
where
    C: Copy
        + Clone
        + fmt::Debug
        + Default
        + Into<Srgba>
        + From<Srgba>
        + Serialize
        + DeserializeOwned,
{
    /// create a new theme from its elements
    pub fn new(
        background: Container<C>,
        primary: Container<C>,
        secondary: Container<C>,
        accent: Accent<C>,
        destructive: Widget<C>,
        warning: Widget<C>,
        success: Widget<C>,
        window_header_background: C,
        text_button_text: C,
    ) -> Self {
        Self {
            background,
            primary,
            secondary,
            accent,
            destructive,
            warning,
            success,
            window_header_background,
            text_button_text,
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

        if let Ok(p) = ron_dirs.place_data_file(ron_name) {
            let mut f = File::create(p)?;
            f.write_all(ron::ser::to_string(self)?.as_bytes())?;
        } else {
            anyhow::bail!("Failed to write RON theme.");
        }
        Ok(())
    }

    /// init the theme directory
    pub fn init() -> anyhow::Result<PathBuf> {
        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let base_dirs = xdg::BaseDirectories::new()?;
        Ok(base_dirs.create_data_directory(ron_path)?)
    }

    /// load a theme by name
    pub fn load_from_name(name: &str) -> anyhow::Result<Self> {
        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let ron_dirs = xdg::BaseDirectories::with_prefix(ron_path)?;

        let ron_name = format!("{}.ron", name);
        if let Some(p) = ron_dirs.find_data_file(ron_name) {
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

    // pub fn light_default() -> Self {
    //     ron::de::from_bytes(include_bytes!("light_default.ron")).unwrap()
    // }

    // pub fn dark_default() -> Self {
    //     ron::de::from_bytes(include_bytes!("dark_default.ron")).unwrap()
    // }
}
