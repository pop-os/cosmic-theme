use std::fmt;

use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use lazy_static::lazy_static;

lazy_static! {
    static ref LIGHT_PALETTE: CosmicPalette<Srgba> = ron::from_str(include_str!("light.ron")).unwrap();
    static ref DARK_PALETTE: CosmicPalette<Srgba> = ron::from_str(include_str!("dark.ron")).unwrap();
}

/// The palette for Cosmic Theme, from which all color properties are derived
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CosmicPalette<C> {
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
    C: Copy
        + Clone
        + fmt::Debug
        + Default
        + Into<Srgba>
        + From<Srgba>
        + Serialize
        + DeserializeOwned,
{
    // TODO
}

// TODO derive Theme from palette
