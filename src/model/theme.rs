// SPDX-License-Identifier: GPL-3.0-only

use crate::{Accent, Container, Destructive, Suggested};
use palette::Srgba;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

/// Cosmic Theme data structure with all colors and its name
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Theme<C> {
    /// name of the theme
    pub name: String,
    /// background element colors
    pub background: Container<C>,
    /// primary element colors
    pub primary: Container<C>,
    /// secondary element colors
    pub secondary: Container<C>,
    /// accent element colors
    pub accent: Accent<C>,
    /// suggested element colors
    pub suggested: Suggested<C>,
    /// destructive element colors
    pub destructive: Destructive<C>,

    // TODO derived surface colors which don't fit neatly in a category
    /// window header background color
    pub window_header_background: C,
    /// text button text color
    pub text_button_text: C,
}

// TODO better eq check
impl<C> PartialEq for Theme<C> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<C> Theme<C>
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
    /// create a new theme from its elements
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

    /// Convert the theme to a high-contrast variant
    pub fn to_high_contrast(&self) -> Self {
        todo!();
    }
}
