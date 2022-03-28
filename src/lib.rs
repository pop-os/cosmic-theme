// SPDX-License-Identifier: GPL-3.0-only

pub use color_picker::*;
pub use config::*;
pub use hex_color::*;
pub use model::*;

mod color_picker;
mod config;
mod hex_color;
mod model;

use gdk_pixbuf::prelude::FileExt;
use gdk_pixbuf::{Colorspace, Pixbuf};
use gio::File;
use hex::encode;
use kmeans_colors::{get_kmeans_hamerly, Kmeans, Sort};
use palette::{rgb::Srgba, Pixel};
use palette::{IntoColor, Lab, Srgb};
use std::path::Path;

pub fn hex_from_rgba(rgba: &Srgba) -> String {
    let hex = encode::<[u8; 4]>(Srgba::into_raw(rgba.into_format()));
    format!("#{hex}")
}

pub fn palette_from_image<P: AsRef<Path>>(path: P) -> Option<Vec<Srgba>> {
    let f = File::for_path(path);
    // calculate kmeans colors from file
    if let Some(Ok(img)) = f.path().map(|p| Pixbuf::from_file(p)) {
        if img.bits_per_sample() == 8 && img.colorspace() == Colorspace::Rgb {
            let pixels = unsafe { img.pixels() };
            let lab: Vec<Lab> = if img.has_alpha() {
                Srgba::from_raw_slice(pixels)
                    .iter()
                    .map(|x| x.color.into_format().into_color())
                    .collect()
            } else {
                Srgb::from_raw_slice(pixels)
                    .iter()
                    .map(|x| x.into_format().into_color())
                    .collect()
            };

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
        } else {
            eprintln!("unsupported color format");
            // TODO error dialog msg
            None
        }
    } else {
        None
    }
}
