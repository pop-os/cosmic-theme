// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

//! Cosmic theme library.
//!
//! Provides utilities for creating custom cosmic themes.
//!

pub use color_picker::*;
pub use config::*;
pub use hex_color::*;
pub use model::*;
pub use output::*;
pub use theme_provider::*;

mod color_picker;
mod config;
mod hex_color;
mod model;
mod output;
mod theme_provider;

use hex::encode;
use image::EncodableLayout;
use kmeans_colors::{get_kmeans_hamerly, Kmeans, Sort};
use palette::{rgb::Srgba, Pixel};
use palette::{IntoColor, Lab};
use std::path::Path;

const NAME: &'static str = "cosmic-theme";

/// Create a hex String from an Srgba
pub fn hex_from_rgba(rgba: &Srgba) -> String {
    let hex = encode::<[u8; 4]>(Srgba::into_raw(rgba.into_format()));
    format!("#{hex}")
}

/// Create a palette from an image
/// The palette is sorted by how often a color occurs in the image, most often first
pub fn palette_from_image<P: AsRef<Path>>(path: P) -> Option<Vec<Srgba>> {
    // calculate kmeans colors from file
    // let pixbuf = Pixbuf::from_file(path);
    let img = image::open(path);
    match img {
        Ok(img) => {
            let lab: Vec<Lab> = Srgba::from_raw_slice(img.to_rgba8().into_raw().as_bytes())
                .iter()
                .map(|x| x.color.into_format().into_color())
                .collect();

            let mut result = Kmeans::new();

            // TODO random seed
            for i in 0..2 {
                let run_result = get_kmeans_hamerly(5, 20, 5.0, false, &lab, i as u64);
                if run_result.score < result.score {
                    result = run_result;
                }
            }
            let mut res = Lab::sort_indexed_colors(&result.centroids, &result.indices);
            res.sort_unstable_by(|a, b| (b.percentage).partial_cmp(&a.percentage).unwrap());
            let colors: Vec<Srgba> = res.iter().map(|x| x.centroid.into_color()).collect();
            Some(colors)
        }
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}
