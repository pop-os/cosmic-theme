use std::{
    fmt,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use lazy_static::lazy_static;
use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{util::CssColor, NAME, PALETTE_DIR};

lazy_static! {
    /// built in light palette
    pub static ref LIGHT_PALETTE: CosmicPalette<CssColor> =
        ron::from_str(include_str!("light.ron")).unwrap();
    /// built in dark palette
    pub static ref DARK_PALETTE: CosmicPalette<CssColor> =
        ron::from_str(include_str!("dark.ron")).unwrap();
}

/// Palette type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CosmicPalette<C> {
    /// Dark mode
    Dark(CosmicPaletteInner<C>),
    /// Light mode
    Light(CosmicPaletteInner<C>),
    /// High contrast light mode
    HighContrastLight(CosmicPaletteInner<C>),
    /// High contrast dark mode
    HighContrastDark(CosmicPaletteInner<C>),
}

impl<C> Default for CosmicPalette<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    fn default() -> Self {
        CosmicPalette::Dark(Default::default())
    }
}

/// The palette for Cosmic Theme, from which all color properties are derived
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CosmicPaletteInner<C> {
    /// name of the palette
    pub name: String,

    /// basic palette
    /// blue: colors used for various points of emphasis in the UI
    pub blue: C,
    /// red: colors used for various points of emphasis in the UI
    pub red: C,
    /// green: colors used for various points of emphasis in the UI
    pub green: C,
    /// yellow: colors used for various points of emphasis in the UI
    pub yellow: C,

    /// surface grays
    /// colors used for three levels of surfaces in the UI
    pub gray_1: C,
    /// colors used for three levels of surfaces in the UI
    pub gray_2: C,
    /// colors used for three levels of surfaces in the UI
    pub gray_3: C,

    /// System Neutrals
    /// A wider spread of dark colors for more general use.
    pub neutral_1: C,
    /// A wider spread of dark colors for more general use.
    pub neutral_2: C,
    /// A wider spread of dark colors for more general use.
    pub neutral_3: C,
    /// A wider spread of dark colors for more general use.
    pub neutral_4: C,
    /// A wider spread of dark colors for more general use.
    pub neutral_5: C,
    /// A wider spread of dark colors for more general use.
    pub neutral_6: C,
    /// A wider spread of dark colors for more general use.
    pub neutral_7: C,
    /// A wider spread of dark colors for more general use.
    pub neutral_8: C,
    /// A wider spread of dark colors for more general use.
    pub neutral_9: C,
    /// A wider spread of dark colors for more general use.
    pub neutral_10: C,

    /// Extended Color Palette
    /// Colors used for themes, app icons, illustrations, and other brand purposes.
    pub ext_warm_grey: C,
    /// Colors used for themes, app icons, illustrations, and other brand purposes.
    pub ext_orange: C,
    /// Colors used for themes, app icons, illustrations, and other brand purposes.
    pub ext_yellow: C,
    /// Colors used for themes, app icons, illustrations, and other brand purposes.
    pub ext_blue: C,
    /// Colors used for themes, app icons, illustrations, and other brand purposes.
    pub ext_purple: C,
    /// Colors used for themes, app icons, illustrations, and other brand purposes.
    pub ext_pink: C,
    /// Colors used for themes, app icons, illustrations, and other brand purposes.
    pub ext_indigo: C,

    /// Potential Accent Color Combos
    pub accent_warm_grey: C,
    /// Potential Accent Color Combos
    pub accent_orange: C,
    /// Potential Accent Color Combos
    pub accent_yellow: C,
    /// Potential Accent Color Combos
    pub accent_purple: C,
    /// Potential Accent Color Combos
    pub accent_pink: C,
    /// Potential Accent Color Combos
    pub accent_indigo: C,
}

impl<C> CosmicPalette<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    /// name of the palette
    pub fn name(&self) -> &str {
        match &self {
            CosmicPalette::Dark(p) => &p.name,
            CosmicPalette::Light(p) => &p.name,
            CosmicPalette::HighContrastLight(p) => &p.name,
            CosmicPalette::HighContrastDark(p) => &p.name,
        }
    }
    /// save the theme to the theme directory
    pub fn save(&self) -> anyhow::Result<()> {
        let ron_path: PathBuf = [NAME, PALETTE_DIR].iter().collect();
        let ron_dirs = xdg::BaseDirectories::with_prefix(ron_path)?;
        let ron_name = format!("{}.ron", self.name());

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
        let ron_path: PathBuf = [NAME, PALETTE_DIR].iter().collect();
        let base_dirs = xdg::BaseDirectories::new()?;
        Ok(base_dirs.create_config_directory(ron_path)?)
    }

    /// load a theme by name
    pub fn load_from_name(name: &str) -> anyhow::Result<Self> {
        let ron_path: PathBuf = [NAME, PALETTE_DIR].iter().collect();
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
}
