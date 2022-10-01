use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

use crate::{CosmicPalette, util::over};

/// Theme Container colors of a theme, can be a theme background container, primary container, or secondary container
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Container<C> {
    /// the color of the container
    pub base: C,
    /// the color of components in the container
    pub component: Component<C>,
    /// the color of dividers in the container
    pub divider: C,
    /// the color of text in the container
    pub on: C,
}

impl<C> Container<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    /// convert to srgba
    pub fn into_srgba(self) -> Container<Srgba> {
        Container {
            base: self.base.into(),
            component: self.component.into_srgba(),
            divider: self.divider.into(),
            on: self.on.into(),
        }
    }
}

impl<C> From<(CosmicPalette<C>, ContainerType)> for Container<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    fn from((p, t): (CosmicPalette<C>, ContainerType)) -> Self {
        match (p, t) {
            (CosmicPalette::Dark(p), ContainerType::Background) => {
                let mut on_bg: Srgba = p.neutral_7.clone().into();
                on_bg.alpha = 0.2;
                let divider: C = on_bg.into();
                Self {
                    base: p.gray_1.clone(),
                    component: (CosmicPalette::Dark(p.clone()), ComponentType::Background).into(),
                    divider,
                    on: p.neutral_7.clone(),
                }
            }
            (CosmicPalette::Dark(p), ContainerType::Primary) => {
                let mut on: Srgba = p.neutral_8.clone().into();
                on.alpha = 0.2;
                let divider: C = on.into();
                Self {
                    base: p.gray_2.clone(),
                    component: (CosmicPalette::Dark(p.clone()), ComponentType::Primary).into(),
                    divider,
                    on: p.neutral_8.clone(),
                }
            }
            (CosmicPalette::Dark(p), ContainerType::Secondary) => {
                let mut on: Srgba = p.neutral_8.clone().into();
                on.alpha = 0.2;
                let divider: C = on.into();
                Self {
                    base: p.gray_3.clone(),
                    component: (CosmicPalette::Dark(p.clone()), ComponentType::Secondary).into(),
                    divider,
                    on: p.neutral_8.clone(),
                }
            }
            (CosmicPalette::Light(p), ContainerType::Background) => {
                let mut on: Srgba = p.neutral_9.clone().into();
                on.alpha = 0.2;
                let divider: C = on.into();
                Self {
                    base: p.gray_1.clone(),
                    component: (CosmicPalette::Dark(p.clone()), ComponentType::Background).into(),
                    divider,
                    on: p.neutral_9.clone(),
                }
            }
            (CosmicPalette::Light(p), ContainerType::Primary) => {
                let mut on: Srgba = p.neutral_8.clone().into();
                on.alpha = 0.2;
                let divider: C = on.into();
                Self {
                    base: p.gray_2.clone(),
                    component: (CosmicPalette::Dark(p.clone()), ComponentType::Primary).into(),
                    divider,
                    on: p.neutral_8.clone(),
                }
            }
            (CosmicPalette::Light(p), ContainerType::Secondary) => {
                let mut on: Srgba = p.neutral_8.clone().into();
                on.alpha = 0.2;
                let divider: C = on.into();
                Self {
                    base: p.gray_3.clone(),
                    component: (CosmicPalette::Dark(p.clone()), ComponentType::Secondary).into(),
                    divider,
                    on: p.neutral_8.clone(),
                }
            }
            (CosmicPalette::HighContrastLight(_), ContainerType::Background)
            | (CosmicPalette::HighContrastLight(_), ContainerType::Primary)
            | (CosmicPalette::HighContrastLight(_), ContainerType::Secondary)
            | (CosmicPalette::HighContrastDark(_), ContainerType::Background)
            | (CosmicPalette::HighContrastDark(_), ContainerType::Primary)
            | (CosmicPalette::HighContrastDark(_), ContainerType::Secondary) => todo!(),
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

/// The colors for a widget of the Cosmic theme
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Component<C> {
    /// The base color of the widget
    pub base: C,
    /// The color of the widget when it is hovered
    pub hover: C,
    /// the color of the widget when it is pressed
    pub pressed: C,
    /// the color of the widget when it is selected
    pub selected: C,
    /// the color of the widget when it is selected
    pub selected_text: C,
    /// the color of the widget when it is focused
    pub focus: C,
    /// the color of dividers for this widget
    pub divider: C,
    /// the color of text for this widget
    pub on: C,
    // the color of text with opacity 80 for this widget
    // pub text_opacity_80: C,
    /// the color of the widget when it is disabled
    pub disabled: C,
    /// the color of text in the widget when it is disabled
    pub on_disabled: C,
}

impl<C> Component<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    /// get @hover_state_color
    pub fn hover_state_color(&self) -> Srgba {
        self.hover.clone().into()
    }
    /// get @pressed_state_color
    pub fn pressed_state_color(&self) -> Srgba {
        self.pressed.clone().into()
    }
    /// get @selected_state_color
    pub fn selected_state_color(&self) -> Srgba {
        self.selected.clone().into()
    }
    /// get @selected_state_text_color
    pub fn selected_state_text_color(&self) -> Srgba {
        self.selected_text.clone().into()
    }
    /// get @focus_color
    pub fn focus_color(&self) -> Srgba {
        self.focus.clone().into()
    }
    /// convert to srgba
    pub fn into_srgba(self) -> Component<Srgba> {
        Component {
            base: self.base.into(),
            hover: self.hover.into(),
            pressed: self.pressed.into(),
            selected: self.selected.into(),
            selected_text: self.selected_text.into(),
            focus: self.focus.into(),
            divider: self.divider.into(),
            on: self.on.into(),
            disabled: self.disabled.into(),
            on_disabled: self.on_disabled.into(),
        }
    }

    pub(crate) fn colored_component(base: C, neutral: C, accent: C) -> Self {
        let neutral = neutral.clone().into();
        let mut neutral_05 = neutral.clone();
        let mut neutral_10 = neutral.clone();
        let mut neutral_20 = neutral.clone();
        neutral_05.alpha = 0.05;
        neutral_10.alpha = 0.1;
        neutral_20.alpha = 0.2;

        let base: Srgba = base.into();
        let mut base_50 = base.clone();
        base_50.alpha = 0.5;

        let on_20 = neutral.clone();
        let mut on_50 = on_20.clone();

        on_50.alpha = 0.5;

        Component {
            base: base.clone().into(),
            hover: over(neutral_10, base).into(),
            pressed: over(neutral_20, base).into(),
            selected: over(neutral_20, base).into(),
            selected_text: accent.clone(),
            divider: on_20.into(),
            on: neutral.into(),
            disabled: base_50.into(),
            on_disabled: on_50.into(),
            focus: accent,
        }
    }

    pub(crate) fn dark_component(base: C, component_state_overlay: C, base_overlay: C, accent : C, on_component: C) -> Self {
        let component_state_overlay = component_state_overlay.clone().into();
        let mut component_state_overlay_10 = component_state_overlay.clone();
        let mut component_state_overlay_20 = component_state_overlay.clone();
        component_state_overlay_10.alpha = 0.1;
        component_state_overlay_20.alpha = 0.2;
        let mut base_overlay_05 = base_overlay.clone().into();
        base_overlay_05.alpha = 0.05;

        let base = base.into();
        let base = over(base_overlay_05, base);
        let mut base_50 = base.clone();
        base_50.alpha = 0.5;

        let mut on_20 = on_component.clone().into();
        let mut on_50 = on_20.clone();

        on_20.alpha = 0.2;
        on_50.alpha = 0.5;

        Component {
            base: base.clone().into(),
            hover: over(component_state_overlay_10, base).into(),
            pressed: over(component_state_overlay_20, base).into(),
            selected: over(component_state_overlay_20, base).into(),
            selected_text: accent.clone(),
            focus: accent.clone(),
            divider: on_20.into(),
            on: on_component.clone(),
            disabled: base_50.into(),
            on_disabled: on_50.into(),
        }
    }

    pub(crate) fn light_component(base: C, overlay: C, accent : C, on_component: C)-> Self {
        let base: Srgba = base.into();
        let mut base_50 = base.clone();
        base_50.alpha = 0.5;
        let overlay = overlay.into();
        let mut overlay_10 = overlay.clone();
        let mut overlay_20 = overlay.clone();

        overlay_10.alpha = 0.1;
        overlay_20.alpha = 0.2;

        let mut on_20 = on_component.clone().into();
        let mut on_50 = on_20.clone();

        on_20.alpha = 0.2;
        on_50.alpha = 0.5;

        Component {
            base: base.clone().into(),
            hover: over(overlay_10, base).into(),
            pressed: over(overlay_20, base).into(),
            selected: over(overlay_20, base).into(),
            selected_text: accent.clone(),
            focus: accent.clone(),
            divider: on_20.into(),
            on: on_component.clone(),
            disabled: base_50.into(),
            on_disabled: on_50.into(),
        }
    }
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
    Accent,
}

impl<C> From<(CosmicPalette<C>, ComponentType)> for Component<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    fn from((p, t): (CosmicPalette<C>, ComponentType)) -> Self {
        match (p, t) {
            (CosmicPalette::Dark(p), ComponentType::Background) => {
                Self::dark_component(p.gray_1, p.neutral_1, p.neutral_10, p.blue, p.neutral_8)
            }
            (CosmicPalette::Dark(p), ComponentType::Primary) => {
                Self::dark_component(p.gray_2, p.neutral_1, p.neutral_10, p.blue, p.neutral_8)
            }
            (CosmicPalette::Dark(p), ComponentType::Secondary) => {
                Self::dark_component(p.gray_3, p.neutral_1, p.neutral_10, p.blue, p.neutral_9)
            }
            (CosmicPalette::Dark(p), ComponentType::Destructive)
            | (CosmicPalette::Light(p), ComponentType::Destructive) => {
                Component::colored_component(p.red.clone(), p.neutral_1.clone(), p.blue.clone())
            }
            (CosmicPalette::Dark(p), ComponentType::Warning)
            | (CosmicPalette::Light(p), ComponentType::Warning) => {
                Component::colored_component(p.yellow.clone(), p.neutral_1, p.blue.clone())
            }
            (CosmicPalette::Dark(p), ComponentType::Success)
            | (CosmicPalette::Light(p), ComponentType::Success) => {
                Component::colored_component(p.green.clone(), p.neutral_1, p.blue.clone())
            }
            (CosmicPalette::Dark(p), ComponentType::Accent)
            | (CosmicPalette::Light(p), ComponentType::Accent) => {
                Component::colored_component(p.blue.clone(), p.neutral_1, p.blue.clone())
            }
            (CosmicPalette::Light(p), ComponentType::Background) => {
                Component::light_component(p.gray_1.clone(), p.neutral_1.clone(),  p.blue.clone(), p.neutral_8)
            }
            (CosmicPalette::Light(p), ComponentType::Primary) => {
                Component::light_component(p.gray_2.clone(), p.neutral_1.clone(),  p.blue.clone(), p.neutral_8)
            }
            (CosmicPalette::Light(p), ComponentType::Secondary) => {
                Component::light_component(p.gray_3.clone(), p.neutral_1.clone(), p.blue.clone(), p.neutral_8)
            }
            (CosmicPalette::HighContrastLight(_), ComponentType::Background)
            | (CosmicPalette::HighContrastLight(_), ComponentType::Accent)
            | (CosmicPalette::HighContrastLight(_), ComponentType::Primary)
            | (CosmicPalette::HighContrastLight(_), ComponentType::Secondary)
            | (CosmicPalette::HighContrastLight(_), ComponentType::Destructive)
            | (CosmicPalette::HighContrastLight(_), ComponentType::Warning)
            | (CosmicPalette::HighContrastLight(_), ComponentType::Success)
            | (CosmicPalette::HighContrastDark(_), ComponentType::Accent)
            | (CosmicPalette::HighContrastDark(_), ComponentType::Background)
            | (CosmicPalette::HighContrastDark(_), ComponentType::Primary)
            | (CosmicPalette::HighContrastDark(_), ComponentType::Secondary)
            | (CosmicPalette::HighContrastDark(_), ComponentType::Destructive)
            | (CosmicPalette::HighContrastDark(_), ComponentType::Warning)
            | (CosmicPalette::HighContrastDark(_), ComponentType::Success) => todo!(),
        }
    }
}
