use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

use crate::CosmicPalette;

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
}

/// Derived theme element from a palette and constraints
#[derive(Debug)]
pub struct Derivation<E> {
    /// Derived  theme element
    pub derived: E,
    /// Derivation errors (Failed constraints)
    pub errors: Vec<anyhow::Error>,
}

impl<C> From<(C, C, C)> for Component<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    fn from((base, neutral, accent): (C, C, C)) -> Self {
        let neutral = neutral.clone().into().into_linear();
        let mut neutral_05 = neutral.clone();
        let mut neutral_10 = neutral.clone();
        let mut neutral_20 = neutral.clone();
        neutral_05.alpha = 0.1;
        neutral_10.alpha = 0.1;
        neutral_20.alpha = 0.2;

        let base: Srgba = base.into();
        let mut base_50 = base.clone().into_linear();
        base_50.alpha = 0.5;

        let on_20 = neutral.clone();
        let mut on_50 = on_20.clone();

        on_50.alpha = 0.5;

        Component {
            base: base.clone().into(),
            hover: Srgba::from_linear(base.clone().into_linear() + neutral_10).into(),
            pressed: Srgba::from_linear(base.clone().into_linear() + neutral_20).into(),
            selected: Srgba::from_linear(base.clone().into_linear() + neutral_20).into(),
            selected_text: accent.clone(),
            divider: Srgba::from_linear(on_20).into(),
            on: Srgba::from_linear(neutral).into(),
            disabled: Srgba::from_linear(base_50).into(),
            on_disabled: Srgba::from_linear(on_50).into(),
            focus: accent,
        }
    }
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
                let base: Srgba = p.gray_1.clone().into();
                let mut base_50 = base.clone().into_linear();
                base_50.alpha = 0.5;
                let neutral_1 = p.neutral_1.clone().into().into_linear();
                let mut neutral_1_10 = neutral_1.clone();
                let mut neutral_1_20 = neutral_1.clone();

                neutral_1_10.alpha = 0.1;
                neutral_1_20.alpha = 0.2;

                let mut on_20 = p.neutral_7.clone().into().into_linear();
                let mut on_50 = on_20.clone();

                on_20.alpha = 0.2;
                on_50.alpha = 0.5;

                Component {
                    base: base.clone().into(),
                    hover: Srgba::from_linear(base.clone().into_linear() + neutral_1_10).into(),
                    pressed: Srgba::from_linear(base.clone().into_linear() + neutral_1_20).into(),
                    selected: Srgba::from_linear(base.clone().into_linear() + neutral_1_20).into(),
                    selected_text: p.blue.clone(),
                    focus: p.blue.clone(),
                    divider: Srgba::from_linear(on_20).into(),
                    on: p.neutral_7.clone(),
                    disabled: Srgba::from_linear(base_50).into(),
                    on_disabled: Srgba::from_linear(on_50).into(),
                }
            }
            (CosmicPalette::Dark(p), ComponentType::Primary) => {
                let neutral_1 = p.neutral_1.clone().into().into_linear();
                let mut neutral_1_05 = neutral_1.clone();
                let mut neutral_1_10 = neutral_1.clone();
                let mut neutral_1_20 = neutral_1.clone();
                neutral_1_05.alpha = 0.1;
                neutral_1_10.alpha = 0.1;
                neutral_1_20.alpha = 0.2;

                let base: Srgba = p.gray_1.clone().into();
                let base = base.clone().into_linear() + neutral_1_05;
                let mut base_50 = base.clone().into_linear();
                base_50.alpha = 0.5;

                let mut on_20 = p.neutral_8.clone().into().into_linear();
                let mut on_50 = on_20.clone();

                on_20.alpha = 0.2;
                on_50.alpha = 0.5;

                Component {
                    base: Srgba::from_linear(base.clone()).into(),
                    hover: Srgba::from_linear(base.clone() + neutral_1_10).into(),
                    pressed: Srgba::from_linear(base.clone() + neutral_1_20).into(),
                    selected: Srgba::from_linear(base.clone() + neutral_1_20).into(),
                    selected_text: p.blue.clone(),
                    focus: p.blue.clone(),
                    divider: Srgba::from_linear(on_20).into(),
                    on: p.neutral_8.clone(),
                    disabled: Srgba::from_linear(base_50).into(),
                    on_disabled: Srgba::from_linear(on_50).into(),
                }
            }
            (CosmicPalette::Dark(p), ComponentType::Secondary) => {
                let neutral_1 = p.neutral_1.clone().into().into_linear();
                let mut neutral_1_05 = neutral_1.clone();
                let mut neutral_1_10 = neutral_1.clone();
                let mut neutral_1_20 = neutral_1.clone();
                neutral_1_05.alpha = 0.1;
                neutral_1_10.alpha = 0.1;
                neutral_1_20.alpha = 0.2;

                let base: Srgba = p.gray_2.clone().into();
                let base = base.clone().into_linear() + neutral_1_05;
                let mut base_50 = base.clone().into_linear();
                base_50.alpha = 0.5;

                let mut on_20 = p.neutral_8.clone().into().into_linear();
                let mut on_50 = on_20.clone();

                on_20.alpha = 0.2;
                on_50.alpha = 0.5;

                Component {
                    base: Srgba::from_linear(base.clone()).into(),
                    hover: Srgba::from_linear(base.clone() + neutral_1_10).into(),
                    pressed: Srgba::from_linear(base.clone() + neutral_1_20).into(),
                    selected: Srgba::from_linear(base.clone() + neutral_1_20).into(),
                    selected_text: p.blue.clone(),
                    focus: p.blue.clone(),
                    divider: Srgba::from_linear(on_20).into(),
                    on: p.neutral_8.clone(),
                    disabled: Srgba::from_linear(base_50).into(),
                    on_disabled: Srgba::from_linear(on_50).into(),
                }
            }
            (CosmicPalette::Dark(p), ComponentType::Destructive)
            | (CosmicPalette::Light(p), ComponentType::Destructive) => {
                (p.red.clone(), p.neutral_1.clone(), p.blue.clone()).into()
            }
            (CosmicPalette::Dark(p), ComponentType::Warning)
            | (CosmicPalette::Light(p), ComponentType::Warning) => {
                (p.yellow.clone(), p.neutral_1, p.blue.clone()).into()
            }
            (CosmicPalette::Dark(p), ComponentType::Success)
            | (CosmicPalette::Light(p), ComponentType::Success) => {
                (p.green.clone(), p.neutral_1, p.blue.clone()).into()
            }
            (CosmicPalette::Dark(p), ComponentType::Accent)
            | (CosmicPalette::Light(p), ComponentType::Accent) => {
                (p.blue.clone(), p.neutral_1, p.blue.clone()).into()
            }
            (CosmicPalette::Light(p), ComponentType::Background) => {
                let base: Srgba = p.gray_1.clone().into();
                let mut base_50 = base.clone().into_linear();
                base_50.alpha = 0.5;
                let neutral_1 = p.neutral_1.clone().into().into_linear();
                let mut neutral_1_10 = neutral_1.clone();
                let mut neutral_1_20 = neutral_1.clone();

                neutral_1_10.alpha = 0.1;
                neutral_1_20.alpha = 0.2;

                let mut on_20 = p.neutral_8.clone().into().into_linear();
                let mut on_50 = on_20.clone();

                on_20.alpha = 0.2;
                on_50.alpha = 0.5;

                Component {
                    base: base.clone().into(),
                    hover: Srgba::from_linear(base.clone().into_linear() + neutral_1_10).into(),
                    pressed: Srgba::from_linear(base.clone().into_linear() + neutral_1_20).into(),
                    selected: Srgba::from_linear(base.clone().into_linear() + neutral_1_20).into(),
                    selected_text: p.blue.clone(),
                    focus: p.blue.clone(),
                    divider: Srgba::from_linear(on_20).into(),
                    on: p.neutral_8.clone(),
                    disabled: Srgba::from_linear(base_50).into(),
                    on_disabled: Srgba::from_linear(on_50).into(),
                }
            }
            (CosmicPalette::Light(p), ComponentType::Primary) => {
                let neutral_1 = p.neutral_1.clone().into().into_linear();
                let mut neutral_1_75 = neutral_1.clone();
                let mut neutral_1_10 = neutral_1.clone();
                let mut neutral_1_20 = neutral_1.clone();
                neutral_1_75.alpha = 0.75;
                neutral_1_10.alpha = 0.1;
                neutral_1_20.alpha = 0.2;

                let base: Srgba = p.gray_1.clone().into();
                let base = base.clone().into_linear() + neutral_1_75;
                let mut base_50 = base.clone().into_linear();
                base_50.alpha = 0.5;

                let mut on_20 = p.neutral_8.clone().into().into_linear();
                let mut on_50 = on_20.clone();

                on_20.alpha = 0.2;
                on_50.alpha = 0.5;

                Component {
                    base: Srgba::from_linear(base.clone()).into(),
                    hover: Srgba::from_linear(base.clone() + neutral_1_10).into(),
                    pressed: Srgba::from_linear(base.clone() + neutral_1_20).into(),
                    selected: Srgba::from_linear(base.clone() + neutral_1_20).into(),
                    selected_text: p.blue.clone(),
                    focus: p.blue.clone(),
                    divider: Srgba::from_linear(on_20).into(),
                    on: p.neutral_8.clone(),
                    disabled: Srgba::from_linear(base_50).into(),
                    on_disabled: Srgba::from_linear(on_50).into(),
                }
            }
            (CosmicPalette::Light(p), ComponentType::Secondary) => {
                let neutral_1 = p.neutral_1.clone().into().into_linear();
                let mut neutral_1_90 = neutral_1.clone();
                let mut neutral_1_10 = neutral_1.clone();
                let mut neutral_1_20 = neutral_1.clone();
                neutral_1_90.alpha = 0.9;
                neutral_1_10.alpha = 0.1;
                neutral_1_20.alpha = 0.2;

                let base: Srgba = p.gray_2.clone().into();
                let base = base.clone().into_linear() + neutral_1_90;
                let mut base_50 = base.clone().into_linear();
                base_50.alpha = 0.5;

                let mut on_20 = p.neutral_8.clone().into().into_linear();
                let mut on_50 = on_20.clone();

                on_20.alpha = 0.2;
                on_50.alpha = 0.5;

                Component {
                    base: Srgba::from_linear(base.clone()).into(),
                    hover: Srgba::from_linear(base.clone() + neutral_1_10).into(),
                    pressed: Srgba::from_linear(base.clone() + neutral_1_20).into(),
                    selected: Srgba::from_linear(base.clone() + neutral_1_20).into(),
                    selected_text: p.blue.clone(),
                    focus: p.blue.clone(),
                    divider: Srgba::from_linear(on_20).into(),
                    on: p.neutral_8.clone(),
                    disabled: Srgba::from_linear(base_50).into(),
                    on_disabled: Srgba::from_linear(on_50).into(),
                }
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
