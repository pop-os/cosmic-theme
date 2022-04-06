// SPDX-License-Identifier: GPL-3.0-only

use palette::{named, IntoColor, Lch, Srgba};
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, Default)]
pub struct Selection<C> {
    pub background: C,
    pub primary_container: C,
    pub secondary_container: C,
    pub accent: C,
    pub accent_text: Option<C>,
    pub accent_nav_handle_text: Option<C>,
    pub destructive: C,
}

// vector should be in order of most common
impl<C> TryFrom<Vec<Srgba>> for Selection<C>
where
    C: Clone + From<Srgba>,
{
    type Error = anyhow::Error;

    fn try_from(mut colors: Vec<Srgba>) -> Result<Self, Self::Error> {
        if colors.len() < 5 {
            anyhow::bail!("length of inputted vector must be at least 5.")
        } else {
            let lch_colors: Vec<Lch> = colors
                .iter()
                .map(|x| {
                    let srgba: Srgba = x.clone().into();
                    srgba.color.into_format().into_color()
                })
                .collect();

            let red_lch: Lch = named::CRIMSON.into_format().into_color();
            let mut reddest_i = 1;
            for (i, c) in lch_colors[1..].iter().enumerate() {
                let d_cur = (c.hue.to_degrees() - red_lch.hue.to_degrees()).abs();
                let reddest_d = (lch_colors[reddest_i].hue.to_degrees().abs()
                    - red_lch.hue.to_degrees().abs())
                .abs();
                if d_cur < reddest_d {
                    reddest_i = i;
                }
            }

            let red = colors.remove(reddest_i);

            Ok(Self {
                background: colors[0].into(),
                primary_container: colors[1].into(),
                secondary_container: colors[3].into(),
                accent: colors[2].into(),
                accent_text: Some(colors[2].into()),
                accent_nav_handle_text: Some(colors[2].into()),
                destructive: red.into(),
            })
        }
    }
}
