// SPDX-License-Identifier: GPL-3.0-only

use hex::encode;
use palette::{Pixel, Srgba};
use std::fmt;

/// Wrapper type for Hex color strings
#[derive(Debug, Clone)]
pub struct Hex {
    hex_string: String,
}

impl<C: Into<Srgba>> From<C> for Hex {
    fn from(c: C) -> Self {
        let srgba: Srgba = c.into();
        let hex_string = encode::<[u8; 4]>(Srgba::into_raw(srgba.into_format()));
        Hex { hex_string }
    }
}

impl Into<String> for Hex {
    fn into(self) -> String {
        self.hex_string
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self)
    }
}
