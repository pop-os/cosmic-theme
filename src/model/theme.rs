// SPDX-License-Identifier: GPL-3.0-only

use crate::{Accent, AsGtkCss, Container, Destructive, Hex};
use palette::Srgba;
use std::fmt;

#[derive(Copy, Clone, Debug, Default)]
pub struct Theme<C>
where
    C: Copy + Clone + fmt::Debug + Default + Into<Hex> + Into<Srgba> + From<Srgba> + fmt::Display,
{
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
    C: Copy + Clone + fmt::Debug + Default + Into<Hex> + Into<Srgba> + From<Srgba> + fmt::Display,
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
        }
    }
}

impl<C> Theme<C>
where
    C: Copy + Clone + fmt::Debug + Default + Into<Hex> + Into<Srgba> + From<Srgba> + fmt::Display,
{
    pub fn as_css(&self) -> String {
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
        css.push_str(&format!(
            r#"/* Accent CSS */
* {{
  background-image: none;
  outline-color: {accent};
}}

"#
        ));
        css
    }
}

pub struct ThemeDerivation<C>
where
    C: Copy + Clone + fmt::Debug + Default + Into<Hex> + Into<Srgba> + From<Srgba> + fmt::Display,
{
    pub theme: Theme<C>,
    pub errors: Vec<anyhow::Error>,
}
