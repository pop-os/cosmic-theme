use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

use crate::CosmicPalette;

/// Theme Container colors of a theme, can be a theme background container, primary container, or secondary container
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Container<C> {
    /// type of the container, background, primary, or secondary
    pub prefix: ContainerType,
    /// the color of the container
    pub container: C,
    /// the color of components in the container
    pub container_component: Widget<C>,
    /// the color of dividers in the container
    pub container_divider: C,
    /// the color of text in the container
    pub container_fg: C,
    /// the color of text with opacity 80 in the container
    pub container_fg_opacity_80: C,
}

impl<C> From<(CosmicPalette<C>, ContainerType)> for Container<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    fn from((p, t): (CosmicPalette<C>, ContainerType)) -> Self {
        match (p, t) {
            (CosmicPalette::Dark(p), ContainerType::Background) => todo!(),
            (CosmicPalette::Dark(p), ContainerType::Primary) => todo!(),
            (CosmicPalette::Dark(p), ContainerType::Secondary) => todo!(),
            (CosmicPalette::Light(p), ContainerType::Background) => todo!(),
            (CosmicPalette::Light(p), ContainerType::Primary) => todo!(),
            (CosmicPalette::Light(p), ContainerType::Secondary) => todo!(),
            (CosmicPalette::HighContrastLight(_), ContainerType::Background) |
            (CosmicPalette::HighContrastLight(_), ContainerType::Primary) |
            (CosmicPalette::HighContrastLight(_), ContainerType::Secondary) |
            (CosmicPalette::HighContrastDark(_), ContainerType::Background) |
            (CosmicPalette::HighContrastDark(_), ContainerType::Primary) |
            (CosmicPalette::HighContrastDark(_), ContainerType::Secondary) => todo!(),
        }
    }
}


/// The type of the container
#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ContainerType {
    /// Background type
    Background,
    /// Primary type
    Primary,
    /// Secondary type
    Secondary,
}

impl Default for ContainerType {
    fn default() -> Self {
        Self::Background
    }
}

impl fmt::Display for ContainerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ContainerType::Background => write!(f, "Background"),
            ContainerType::Primary => write!(f, "Primary Container"),
            ContainerType::Secondary => write!(f, "Secondary Container"),
        }
    }
}

/// The accent colors of a theme
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Accent<C> {
    /// Base accent color
    pub accent: C,
    /// Accent text color
    pub accent_fg: C,
    /// Accent nav handle text color
    pub accent_nav_handle_fg: C,
    /// Accent Widget colors
    pub suggested: Widget<C>,
}

impl<C> From<CosmicPalette<C>> for Accent<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    fn from(p: CosmicPalette<C>) -> Self {
        match p {
            CosmicPalette::Dark(_) => todo!(),
            CosmicPalette::Light(_) => todo!(),
            CosmicPalette::HighContrastLight(_) |
            CosmicPalette::HighContrastDark(_) => todo!(),
        }
    }
}

/// The colors for a widget of the Cosmic theme
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Widget<C> {
    /// The base color of the widget
    pub base: C,
    /// The color of the widget when it is hovered
    pub hover: C,
    /// the color of the widget when it is pressed
    pub pressed: C,
    /// the color of the widget when it is selected
    pub selected: C,
    /// the color of dividers for this widget
    pub divider: C,
    /// the color of text for this widget
    pub text: C,
    /// the color of text with opacity 80 for this widget
    pub text_opacity_80: C,
    /// the color of the widget when it is disabled
    pub disabled: C,
    /// the color of text in the widget when it is disabled
    pub disabled_fg: C,
}

/// Derived theme element from a palette and constraints
#[derive(Debug)]
pub struct Derivation<E> {
    /// Derived  theme element
    pub derived: E,
    /// Derivation errors (Failed constraints)
    pub errors: Vec<anyhow::Error>,
}

pub(crate) enum ComponentType {
    Background,
    Primary,
    Secondary,
    Destructive,
    Warning,
    Success,
}

impl<C> From<(CosmicPalette<C>, ComponentType)> for Widget<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    fn from((p, t): (CosmicPalette<C>, ComponentType)) -> Self {
        match (p, t) {
            (CosmicPalette::Dark(p), ComponentType::Background) => todo!(),
            (CosmicPalette::Dark(p), ComponentType::Primary) => todo!(),
            (CosmicPalette::Dark(p), ComponentType::Secondary) => todo!(),
            (CosmicPalette::Dark(p), ComponentType::Destructive) => todo!(),
            (CosmicPalette::Dark(p), ComponentType::Warning) => todo!(),
            (CosmicPalette::Dark(p), ComponentType::Success) => todo!(),
            (CosmicPalette::Light(p), ComponentType::Background) => todo!(),
            (CosmicPalette::Light(p), ComponentType::Primary) => todo!(),
            (CosmicPalette::Light(p), ComponentType::Secondary) => todo!(),
            (CosmicPalette::Light(p), ComponentType::Destructive) => todo!(),
            (CosmicPalette::Light(p), ComponentType::Warning) => todo!(),
            (CosmicPalette::Light(p), ComponentType::Success) => todo!(),
            (CosmicPalette::HighContrastLight(_), ComponentType::Background) |
            (CosmicPalette::HighContrastLight(_), ComponentType::Primary) |
            (CosmicPalette::HighContrastLight(_), ComponentType::Secondary) |
            (CosmicPalette::HighContrastLight(_), ComponentType::Destructive) |
            (CosmicPalette::HighContrastLight(_), ComponentType::Warning) |
            (CosmicPalette::HighContrastLight(_), ComponentType::Success) |
            (CosmicPalette::HighContrastDark(_), ComponentType::Background) |
            (CosmicPalette::HighContrastDark(_), ComponentType::Primary) |
            (CosmicPalette::HighContrastDark(_), ComponentType::Secondary) |
            (CosmicPalette::HighContrastDark(_), ComponentType::Destructive) |
            (CosmicPalette::HighContrastDark(_), ComponentType::Warning) |
            (CosmicPalette::HighContrastDark(_), ComponentType::Success) => todo!(),
        }
    }
}
