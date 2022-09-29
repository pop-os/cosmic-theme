use serde::{Deserialize, Serialize};
use std::fmt;

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

// /// The destructive colors of a theme
// #[derive(Copy, Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
// pub struct Destructive<C> {
//     /// The destructive colors of a theme
//     pub destructive: Widget<C>,
// }

// /// The suggested colors of a theme
// #[derive(Copy, Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
// pub struct Suggested<C> {
//     /// The destructive colors of a theme
//     pub suggested: Widget<C>,
// }

// /// The suggested colors of a theme
// #[derive(Copy, Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
// pub struct Warning<C> {
//     /// The destructive colors of a theme
//     pub warning: Widget<C>,
// }

/// The colors for a widget of the Cosmic theme
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Widget<C> {
    /// The base color of the widget
    pub base: C,
    /// The color of the widget when it is hovered
    pub hover: C,
    /// the color of the widget when it is pressed
    pub pressed: C,
    /// the color of the widget when it is focused
    pub focused: C,
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
