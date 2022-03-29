// SPDX-License-Identifier: GPL-3.0-only
use crate::{Hex, Theme, NAME};
use anyhow::{bail, Result};
use palette::Srgba;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt,
    fs::{create_dir_all, File},
    io::prelude::*,
    path::PathBuf,
};

const CSS_DIR: &'static str = "css";
const SASS_DIR: &'static str = "sass";
const THEME_DIR: &'static str = "themes";

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
        let sass_path: PathBuf = [NAME, SASS_DIR].iter().collect();

        let ron_dirs = xdg::BaseDirectories::with_prefix(ron_path)?;
        let css_dirs = xdg::BaseDirectories::with_prefix(css_path)?;
        let sass_dirs = xdg::BaseDirectories::with_prefix(sass_path)?;

        let ron_name = format!("{}.ron", self.get_name());
        let css_name = format!("{}.css", self.get_name());
        let sass_name = format!("{}.sass", self.get_name());

        if let Ok(p) = ron_dirs.place_data_file(ron_name) {
            let mut f = File::create(p)?;
            f.write_all(ron_str.as_bytes())?;
        } else {
            bail!("Failed to write RON theme.")
        }

        // if let Ok(p) = sass_dirs.place_data_file(sass_name) {
        //     create_dir_all(&p)?;
        //     let mut f = File::create(p)?;
        //     f.write_all(sass_str.as_bytes())?;
        // } else {
        //     bail!("Failed to write RON theme.")
        // }

        if let Ok(p) = css_dirs.place_data_file(css_name) {
            let mut f = File::create(p)?;
            f.write_all(css_str.as_bytes())?;
        } else {
            bail!("Failed to write RON theme.")
        }

        Ok(())
    }
}
