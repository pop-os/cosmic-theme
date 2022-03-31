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
pub trait AsGtkCss<C>
where
    C: Copy + Into<Srgba> + From<Srgba>,
{
    fn as_css(&self) -> String;
}

impl<C> AsGtkCss<C> for Container<C>
where
    C: Copy + Clone + fmt::Debug + Default + Into<Srgba> + From<Srgba> + fmt::Display,
{
    fn as_css(&self) -> String {
        let Self {
            prefix,
            container,
            container_component,
            container_divider,
            container_text,
            container_text_opacity_80,
        } = self;
        let Widget {
            default,
            hover,
            pressed,
            focused,
            divider,
            text,
            // XXX this should ideally maintain AAA contrast, and failing that, color chooser should raise warnings
            text_opacity_80,
            // these are transparent but are not required to maintain contrast
            disabled,
            disabled_text,
        } = container_component;

        let prefix_lower = match prefix {
            ContainerType::Background => "background",
            ContainerType::Primary => "primary-container",
            ContainerType::Secondary => "secondary-container",
        };

        format!(
            r#"
/* {prefix_lower} CSS */
*.{prefix_lower} {{
  background-color: {container};
  color: {container_text};
}}

*.{prefix_lower}-component {{
  transition-duration: 50ms;
  background-color: {default};
  color: {text};
  border-color: {default};
}}

*.{prefix_lower}-component:hover {{
  transition-duration: 50ms;
  background-color: {hover};
  color: {text};
  border-color: {default};
}}

*.{prefix_lower}-component:selected {{
  transition-duration: 50ms;
  background-color: {focused};
  transition-duration: 50ms;
  outline-color: {default};
  color: {text};
  border-color: {default};
}}

/* slider and switch are examples of widgets which likely want sass */
*.{prefix_lower}-component:checked {{
  transition-duration: 50ms;
  background-color: {pressed};
  outline-color: {default};
  color: {text};
  border-color: {default};
}}

*.{prefix_lower}-component slider {{
  transition-duration: 50ms;
  background-color: {text};
  outline-color: {default};
  color: {text};
  border-color: {default};
}}

*.{prefix_lower}-component:active {{
  transition-duration: 50ms;
  background-color: {pressed};
  color: {text};
  border-color: {default};
}}

*.{prefix_lower}-component:disabled {{
  transition-duration: 50ms;
  background-color: {disabled};
  color: {text};
  border-color: {default};
}}

*.{prefix_lower}-divider {{
  background-color: {container_divider};
}}

*.{prefix_lower}-component-divider {{
  background-color: {divider};
}}
"#
        )
    }
}

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

impl<C> AsGtkCss<C> for Accent<C>
where
    C: Copy
        + Clone
        + fmt::Debug
        + Default
        + Into<Srgba>
        + From<Srgba>
        + fmt::Display
        + Serialize
        + DeserializeOwned,
{
    fn as_css(&self) -> String {
        let Accent {
            accent,
            accent_text,
            accent_nav_handle_text,
            suggested,
        } = self;

        let Widget {
            default,
            hover,
            pressed,
            focused,
            divider,
            text,
            // XXX this should ideally maintain AAA contrast, and failing that, color chooser should raise warnings
            text_opacity_80,
            // these are transparent but are not required to maintain contrast
            disabled,
            disabled_text,
        } = suggested;

        format!(
            r#"/* Suggested CSS */
*.suggested-action {{
  background-color: {default};
  color: {text};
  border-color: {default};
}}

*.suggested-action:hover {{
  background-color: {hover};
  color: {text};
  border-color: {default};
}}

*.suggested-action:selected {{
  background-color: {focused};
  outline-color: {default};
  color: {text};
  border-color: {default};
}}

*.suggested-action:active {{
  background-color: {pressed};
  color: {text};
  border-color: {default};
}}

*.suggested-action:disabled {{
  background-color: {disabled};
  color: {text};
  border-color: {default};
}}

"#
        )
    }
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

impl<C> AsGtkCss<C> for Destructive<C>
where
    C: Copy
        + Clone
        + fmt::Debug
        + Default
        + Into<Srgba>
        + From<Srgba>
        + fmt::Display
        + Serialize
        + DeserializeOwned,
{
    fn as_css(&self) -> String {
        let Destructive { destructive } = &self;
        let Widget {
            default,
            hover,
            pressed,
            focused,
            divider,
            text,
            text_opacity_80,
            disabled,
            disabled_text,
        } = destructive;

        format!(
            r#"/* Destructive CSS */
*.destructive-action {{
  background-color: {default};
  color: {text};
  border-color: {default};
}}

*.destructive-action:hover {{
  background-color: {hover};
  color: {text};
  border-color: {default};
}}

*.destructive-action:selected {{
  background-color: {focused};
  outline-color: {default};
  color: {text};
  border-color: {default};
}}

*.destructive-action:active {{
  background-color: {pressed};
  color: {text};
  border-color: {default};
}}

*.destructive-action:disabled {{
  background-color: {disabled};
  color: {text};
  border-color: {default};
}}

"#
        )
    }
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
