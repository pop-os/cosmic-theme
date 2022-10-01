use csscolorparser::Color;
use palette::Srgba;
use serde::{Deserialize, Serialize};

/// utility wrapper for serializing and deserializing colors with arbitrary CSS
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CssColor {
    c: Color,
}

impl From<Srgba> for CssColor {
    fn from(c: Srgba) -> Self {
        Self {
            c: Color {
                r: c.red as f64,
                g: c.green as f64,
                b: c.blue as f64,
                a: c.alpha as f64,
            },
        }
    }
}

impl Into<Srgba> for CssColor {
    fn into(self) -> Srgba {
        Srgba::new(
            self.c.r as f32,
            self.c.g as f32,
            self.c.b as f32,
            self.c.a as f32,
        )
    }
}

/// blend colors in the Srgb space
pub fn over(a: Srgba, b: Srgba) -> Srgba {
    // A over B example
    // TODO is the new alpha correct?
    let c_a = 1.0 - (1.0 - a.alpha) * (1.0 - b.alpha);
    let c_r = a.alpha * a.red + (1.0 - a.alpha) *b.red;
    let c_g = a.alpha * a.green +(1.0 - a.alpha) * b.green;
    let c_b = a.alpha * a.blue + (1.0 - a.alpha) *b.blue;
    
    Srgba::new(c_r, c_g, c_b, c_a)
}
