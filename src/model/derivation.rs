use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

use crate::{util::over, CosmicPalette};

// TODO Ashley: derivation can probably be cleaned up a lot if using this method

/// Theme Container colors of a theme, can be a theme background container, primary container, or secondary container
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Container<C> {
    /// the color of the container
    pub base: C,
}

impl<C> Container<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    /// convert to srgba
    pub fn into_srgba(self) -> Container<Srgba> {
        Container {
            base: self.base.into(),
        }
    }

    pub(crate) fn new(bg: C) -> Self {
        Self { base: bg }
    }
}

impl<C> From<(CosmicPalette<C>, ContainerType)> for Container<C>
where
    C: Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + Serialize + DeserializeOwned,
{
    fn from((p, t): (CosmicPalette<C>, ContainerType)) -> Self {
        match (p, t) {
            (CosmicPalette::Dark(p), ContainerType::Background) => Self::new(p.gray_1.clone()),
            (CosmicPalette::Dark(p), ContainerType::Primary) => Self::new(p.gray_2.clone()),
            (CosmicPalette::Dark(p), ContainerType::Secondary) => Self::new(p.gray_3.clone()),
            (CosmicPalette::HighContrastDark(p), ContainerType::Background) => {
                Self::new(p.gray_1.clone())
            }
            (CosmicPalette::HighContrastDark(p), ContainerType::Primary) => {
                Self::new(p.gray_2.clone())
            }
            (CosmicPalette::HighContrastDark(p), ContainerType::Secondary) => {
                Self::new(p.gray_3.clone())
            }
            (CosmicPalette::Light(p), ContainerType::Background) => Self::new(p.gray_1.clone()),
            (CosmicPalette::Light(p), ContainerType::Primary) => Self::new(p.gray_2.clone()),
            (CosmicPalette::Light(p), ContainerType::Secondary) => Self::new(p.gray_3.clone()),
            (CosmicPalette::HighContrastLight(p), ContainerType::Background) => {
                Self::new(p.gray_1.clone())
            }
            (CosmicPalette::HighContrastLight(p), ContainerType::Primary) => {
                Self::new(p.gray_2.clone())
            }
            (CosmicPalette::HighContrastLight(p), ContainerType::Secondary) => {
                Self::new(p.gray_3.clone())
            }
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
    // the color of text with opacity 80 for this widget
    // pub text_opacity_80: C,
    /// the color of the widget when it is disabled
    pub disabled: C,
    /// the color of text on the widget
    pub on: C,
    /// the color of text on a disabled widget
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
            disabled: self.disabled.into(),
            on: self.on.into(),
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
            selected: over(neutral_10, base).into(),
            selected_text: accent.clone(),
            disabled: base_50.into(),
            focus: accent,
            on: neutral.into(),
            on_disabled: on_50.into(),
        }
    }

    pub(crate) fn dark_component(
        base: C,
        component_state_overlay: C,
        base_overlay: C,
        accent: C,
        on_component: C,
        _is_high_contrast: bool,
    ) -> Self {
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
            selected: over(component_state_overlay_10, base).into(),
            selected_text: accent.clone(),
            focus: accent.clone(),
            disabled: base_50.into(),
            on: on_component,
            on_disabled: on_50.into(),
        }
    }

    pub(crate) fn light_component(
        base: C,
        overlay: C,
        accent: C,
        on_component: C,
        _is_high_contrast: bool,
    ) -> Self {
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
            selected: over(overlay_10, base).into(),
            selected_text: accent.clone(),
            focus: accent.clone(),
            disabled: base_50.into(),
            on: on_component,
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
    Basic,
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
        let (base_overlay, mut on_basic): (_, Srgba) = match &p {
            CosmicPalette::Dark(p) | CosmicPalette::HighContrastDark(p) => {
                let mut c: Srgba = p.neutral_10.clone().into();
                c.alpha = 0.05;
                (c.into(), p.neutral_10.clone().into())
            }
            CosmicPalette::Light(p) | CosmicPalette::HighContrastLight(p) => {
                let mut c: Srgba = p.neutral_1.clone().into();
                c.alpha = 0.8;
                (c.into(), p.neutral_10.clone().into())
            }
        };
        // TODO maybe different value depending on light / dark / high contrast theme?
        on_basic.alpha = 0.9;
        match (p, t) {
            (CosmicPalette::Dark(p), ComponentType::Basic) => Self::dark_component(
                base_overlay,
                p.neutral_1,
                p.neutral_10,
                p.blue,
                on_basic.into(),
                false,
            ),
            (CosmicPalette::HighContrastDark(p), ComponentType::Basic) => Self::dark_component(
                base_overlay,
                p.neutral_1,
                p.neutral_10,
                p.blue,
                on_basic.into(),
                true,
            ),

            (CosmicPalette::Light(p), ComponentType::Basic) => Component::light_component(
                base_overlay,
                p.neutral_1.clone(),
                p.blue.clone(),
                on_basic.into(),
                false,
            ),

            (CosmicPalette::HighContrastLight(p), ComponentType::Basic) => {
                Component::light_component(
                    base_overlay,
                    p.neutral_1.clone(),
                    p.blue.clone(),
                    on_basic.into(),
                    true,
                )
            }

            (CosmicPalette::Dark(p), ComponentType::Destructive)
            | (CosmicPalette::Light(p), ComponentType::Destructive)
            | (CosmicPalette::HighContrastLight(p), ComponentType::Destructive)
            | (CosmicPalette::HighContrastDark(p), ComponentType::Destructive) => {
                Component::colored_component(p.red.clone(), p.neutral_1.clone(), p.blue.clone())
            }

            (CosmicPalette::Dark(p), ComponentType::Warning)
            | (CosmicPalette::Light(p), ComponentType::Warning)
            | (CosmicPalette::HighContrastLight(p), ComponentType::Warning)
            | (CosmicPalette::HighContrastDark(p), ComponentType::Warning) => {
                Component::colored_component(p.yellow.clone(), p.neutral_1, p.blue.clone())
            }

            (CosmicPalette::Dark(p), ComponentType::Success)
            | (CosmicPalette::Light(p), ComponentType::Success)
            | (CosmicPalette::HighContrastLight(p), ComponentType::Success)
            | (CosmicPalette::HighContrastDark(p), ComponentType::Success) => {
                Component::colored_component(p.green.clone(), p.neutral_1, p.blue.clone())
            }

            (CosmicPalette::Dark(p), ComponentType::Accent)
            | (CosmicPalette::Light(p), ComponentType::Accent)
            | (CosmicPalette::HighContrastDark(p), ComponentType::Accent)
            | (CosmicPalette::HighContrastLight(p), ComponentType::Accent) => {
                Component::colored_component(p.blue.clone(), p.neutral_1, p.blue.clone())
            }
        }
    }
}
