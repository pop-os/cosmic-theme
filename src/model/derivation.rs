use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Container<C> {
    pub prefix: ContainerType,
    pub container: C,
    pub container_component: Widget<C>,
    pub container_divider: C,
    pub container_text: C,
    pub container_text_opacity_80: C,
}

// TODO special styling for switches in gtk4

#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ContainerType {
    Background,
    Primary,
    Secondary,
}

impl Default for ContainerType {
    fn default() -> Self {
        Self::Background
    }
}

impl fmt::Display for ContainerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ContainerType::Background => write!(f, "Background"),
            ContainerType::Primary => write!(f, "Primary Container"),
            ContainerType::Secondary => write!(f, "Secondary Container"),
        }
    }
}

#[derive(Debug)]
pub struct ContainerDerivation<C> {
    pub container: Container<C>,
    pub errors: Vec<anyhow::Error>,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Accent<C> {
    pub accent: C,
    pub accent_text: C,
    pub accent_nav_handle_text: C,
    pub suggested: Widget<C>,
}

#[derive(Debug)]
pub struct AccentDerivation<C> {
    pub accent: Accent<C>,
    pub errors: Vec<anyhow::Error>,
}


#[derive(Copy, Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Destructive<C> {
    pub destructive: Widget<C>,
}

#[derive(Debug)]
pub struct DestructiveDerivation<C> {
    pub destructive: Destructive<C>,
    pub errors: Vec<anyhow::Error>,
}

#[derive(Copy, Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Widget<C> {
    pub default: C,
    pub hover: C,
    pub pressed: C,
    pub focused: C,
    pub divider: C,
    pub text: C,
    // XXX this should ideally maintain AAA contrast, and failing that, color chooser should raise warnings
    pub text_opacity_80: C,
    // these are transparent but are not required to maintain contrast
    pub disabled: C,
    pub disabled_text: C,
}

pub struct WidgetDerivation<C> {
    pub widget: Widget<C>,
    pub errors: Vec<anyhow::Error>,
}
