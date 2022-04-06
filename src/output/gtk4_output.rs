// SPDX-License-Identifier: GPL-3.0-only
use crate::{Hex, Theme, NAME, model::{Container, Accent, Widget, ContainerType, Destructive}};
use anyhow::{bail, Result};
use palette::Srgba;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt, fs::File, io::prelude::*, path::PathBuf};

pub const CSS_DIR: &'static str = "css";
pub const SASS_DIR: &'static str = "sass";
pub const THEME_DIR: &'static str = "themes";

pub trait GtkOutput {
    fn as_sass(&self) -> String;
    fn as_css(&self) -> Result<String>;
    fn get_name(&self) -> String;
    fn write(&self) -> Result<()>;
}

impl<C> GtkOutput for Theme<C>
where
    C: Copy
        + Clone
        + fmt::Debug
        + Default
        + Into<Hex>
        + Into<Srgba>
        + From<Srgba>
        + fmt::Display
        + Serialize
        + DeserializeOwned,
{
    fn as_css(&self) -> Result<String> {
        todo!()
    }

    fn as_sass(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn write(&self) -> Result<()> {
        // TODO sass -> css
        let ron_str = ron::ser::to_string_pretty(self, Default::default())?;
        // let sass_str = self.as_sass();
        // let css_str = self.as_css()?;
        let css_str = self.preview_gtk_css();

        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let css_path: PathBuf = [NAME, CSS_DIR].iter().collect();

        let ron_dirs = xdg::BaseDirectories::with_prefix(ron_path)?;
        let css_dirs = xdg::BaseDirectories::with_prefix(css_path)?;

        let ron_name = format!("{}.ron", self.get_name());
        let css_name = format!("{}.css", self.get_name());

        if let Ok(p) = ron_dirs.place_data_file(ron_name) {
            let mut f = File::create(p)?;
            f.write_all(ron_str.as_bytes())?;
        } else {
            bail!("Failed to write RON theme.")
        }

        if let Ok(p) = css_dirs.place_data_file(css_name) {
            let mut f = File::create(p)?;
            f.write_all(css_str.as_bytes())?;
        } else {
            bail!("Failed to write RON theme.")
        }

        Ok(())
    }
}

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
            ..
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
