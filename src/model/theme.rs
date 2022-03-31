// SPDX-License-Identifier: GPL-3.0-only

use crate::{Accent, AsGtkCss, Container, Destructive};
use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Theme<C> {
    pub name: String,
    background: Container<C>,
    primary: Container<C>,
    secondary: Container<C>,
    accent: Accent<C>,
    destructive: Destructive<C>,

    // TODO derived surface colors which don't fit neatly in a category
    window_header_background: C,
    text_button_text: C,
}

impl<C> Theme<C>
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
    pub fn new(
        background: Container<C>,
        primary: Container<C>,
        secondary: Container<C>,
        accent: Accent<C>,
        destructive: Destructive<C>,
        window_header_background: C,
        text_button_text: C,
    ) -> Self {
        Self {
            background,
            primary,
            secondary,
            accent,
            destructive,
            window_header_background,
            text_button_text,
            ..Default::default()
        }
    }

    pub fn preview_gtk_css(&self) -> String {
        let Self {
            background,
            primary,
            secondary,
            accent,
            destructive,
            ..
        } = self;
        let mut css = String::new();

        css.push_str(&background.as_css());
        css.push_str(&primary.as_css());
        css.push_str(&secondary.as_css());
        css.push_str(&accent.as_css());
        css.push_str(&destructive.as_css());
        let accent = accent.accent;
        let background_color = background.container;
        css.push_str(&format!(
            r#"/* Accent CSS */
popover {{
  background-color: transparent;
  background-image: none;
}}

popover contents {{
  border-width: 0px;
  border-radius: 12px;
  padding: 12px;
  background: {background_color};
}}

popover arrow {{
  border-width: 0px;
  border-radius: 12px;
  padding: 12px;
  background: {background_color};
}}

* {{
  background-image: none;
  outline-color: {accent};
}}

*.padding-x-small {{
  padding: 4px;
}}

*.padding-small {{
  padding: 8px;
}}

*.padding-medium {{
  padding: 12px;
}}

*.padding-large {{
  padding: 16px;
}}

*.padding-x-large {{
  padding: 20px;
}}

*.border-radius-x-small {{
  border-radius: 4px;
}}

*.border-radius-small {{
  border-radius: 8px;
}}

*.border-radius-medium {{
  border-radius: 12px;
}}

*.border-radius-large {{
  border-radius: 16px;
}}

*.border-radius-x-large {{
  border-radius: 20px;
}}
"#
        ));
        css
    }
}

#[derive(Debug, Default)]
pub struct ThemeDerivation<C> {
    pub theme: Theme<C>,
    pub errors: Vec<anyhow::Error>,
}
