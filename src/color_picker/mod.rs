use crate::{
    Accent, AccentDerivation, Container, ContainerDerivation, ContainerType, Destructive,
    DestructiveDerivation, Selection, Theme, ThemeConstraints, ThemeDerivation, Widget,
    WidgetDerivation,
};
use anyhow::{anyhow, Result};
use palette::{IntoColor, Lcha, Shade, Srgba};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt;

pub use exact::*;
mod exact;

pub trait ColorPicker<
    C: Into<Srgba>
        + From<Srgba>
        + Copy
        + Clone
        + fmt::Debug
        + Default
        + fmt::Display
        + Serialize
        + DeserializeOwned,
>
{
    fn pick_color(
        &self,
        color: C,
        contrast: Option<f32>,
        grayscale: bool,
        lighten: Option<bool>,
    ) -> Result<C>;

    fn pick_color_text(
        &self,
        color: C,
        grayscale: bool,
        lighten: Option<bool>,
    ) -> (C, Option<anyhow::Error>);

    fn pick_color_graphic(
        &self,
        color: C,
        contrast: f32,
        grayscale: bool,
        lighten: Option<bool>,
    ) -> (C, Option<anyhow::Error>);

    fn get_selection(&self) -> Selection<C>;

    fn get_constraints(&self) -> ThemeConstraints;

    fn theme_derivation(&self) -> ThemeDerivation<C> {
        let selection = self.get_selection();
        let mut theme_errors = Vec::new();
        let window_header_background = selection.background;
        let (text_button_text, err) = self.pick_color_text(selection.background, true, None);
        if let Some(err) = err {
            theme_errors.push(err)
        };
        let ContainerDerivation {
            container: background,
            errors: mut errs,
        } = self.container_derivation(ContainerType::Background);
        theme_errors.append(&mut errs);

        let ContainerDerivation {
            container: primary,
            errors: mut errs,
        } = self.container_derivation(ContainerType::Primary);
        theme_errors.append(&mut errs);

        let ContainerDerivation {
            container: secondary,
            mut errors,
        } = self.container_derivation(ContainerType::Secondary);
        theme_errors.append(&mut errors);

        let AccentDerivation { accent, mut errors } = self.accent_derivation();
        theme_errors.append(&mut errors);

        let DestructiveDerivation {
            destructive,
            mut errors,
        } = self.destructive_derivation();
        theme_errors.append(&mut errors);

        ThemeDerivation {
            theme: Theme::new(
                background,
                primary,
                secondary,
                accent,
                destructive,
                window_header_background,
                text_button_text,
            ),
            errors: theme_errors,
        }
    }

    fn container_derivation(&self, container_type: ContainerType) -> ContainerDerivation<C> {
        let selection = self.get_selection();
        let constraints = self.get_constraints();

        let mut errors = Vec::new();

        let Selection {
            background,
            primary_container,
            secondary_container,
            ..
        } = selection;

        let ThemeConstraints {
            elevated_contrast_ratio,
            divider_contrast_ratio,
            divider_gray_scale,
            lighten,
            ..
        } = constraints;

        let container = match container_type {
            ContainerType::Background => background,
            ContainerType::Primary => primary_container,
            ContainerType::Secondary => secondary_container,
        };
        let (container_divider, err) = self.pick_color_graphic(
            container,
            divider_contrast_ratio,
            divider_gray_scale,
            Some(lighten),
        );
        if let Some(e) = err {
            errors.push(e);
        };

        let (container_text, err) = self.pick_color_text(container, true, None);
        if let Some(err) = err {
            let err = anyhow!("{} => \"container text\" failed: {}", container_type, err);
            errors.push(err);
        };

        // TODO revisit this and adjust constraints for transparency
        let mut container_text_opacity_80: Srgba = container_text.into();
        container_text_opacity_80.alpha *= 0.8;

        let (component_default, err) =
            self.pick_color_graphic(container, elevated_contrast_ratio, false, Some(lighten));
        if let Some(e) = err {
            let err = anyhow!(
                "{} => \"container component\" failed: {}",
                container_type,
                e
            );
            errors.push(err);
        };

        let WidgetDerivation {
            widget: container_component,
            errors: errs,
        } = self.widget_derivation(component_default);
        for e in errs {
            let err = anyhow!(
                "{} => \"container component derivation\" failed: {}",
                container_type,
                e
            );
            errors.push(err);
        }

        ContainerDerivation {
            container: Container {
                prefix: container_type,
                container,
                container_divider,
                container_text,
                container_text_opacity_80: container_text_opacity_80.into(),
                container_component,
            },
            errors,
        }
    }

    fn destructive_derivation(&self) -> DestructiveDerivation<C> {
        let selection = self.get_selection();

        let mut errors = Vec::<anyhow::Error>::new();

        let WidgetDerivation {
            widget: destructive,
            errors: errs,
        } = self.widget_derivation(selection.destructive);
        for e in errs {
            errors.push(anyhow!(
                "\"Destructive component derivation\" failed: {}",
                e
            ));
        }
        DestructiveDerivation {
            destructive: Destructive { destructive },
            errors,
        }
    }

    fn widget_derivation(&self, default: C) -> WidgetDerivation<C> {
        let ThemeConstraints {
            divider_contrast_ratio,
            divider_gray_scale,
            lighten,
            ..
        } = self.get_constraints();

        let mut errors = Vec::new();

        let rgba: Srgba = default.into();
        let lch = Lcha {
            color: rgba.color.into_color(),
            alpha: rgba.alpha,
        };

        // TODO define constraints for different states...
        // & add color self methods and errors if these fail
        let hover = if lighten {
            lch.lighten(0.1)
        } else {
            lch.darken(0.1)
        };

        let pressed = if lighten {
            hover.lighten(0.1)
        } else {
            hover.darken(0.1)
        };
        let pressed = C::from(Srgba {
            color: pressed.color.into_color(),
            alpha: pressed.alpha,
        });

        // TODO is this actually a different color? or just outlined?
        let focused = default;

        let mut disabled: Srgba = default.into();
        disabled.alpha = 0.5;

        let (divider, error) = self.pick_color_graphic(
            pressed,
            divider_contrast_ratio,
            divider_gray_scale,
            Some(lighten),
        );
        if let Some(error) = error {
            errors.push(error);
        }

        let (text, error) = self.pick_color_text(pressed, true, None);
        if let Some(error) = error {
            errors.push(error);
        }

        let mut text_opacity_80: Srgba = text.into();
        text_opacity_80.alpha = 0.8;

        let mut disabled_text = text.into();
        disabled_text.alpha = 0.5;

        WidgetDerivation {
            widget: Widget {
                default,
                hover: C::from(Srgba {
                    color: hover.color.into_color(),
                    alpha: hover.alpha,
                }),
                pressed,
                focused,
                divider,
                text,
                text_opacity_80: text_opacity_80.into(),
                disabled: disabled.into(),
                disabled_text: disabled_text.into(),
            },
            errors,
        }
    }

    fn accent_derivation(&self) -> AccentDerivation<C> {
        let Selection {
            accent,
            accent_text,
            accent_nav_handle_text,
            ..
        } = self.get_selection();
        let mut errors = Vec::<anyhow::Error>::new();

        let WidgetDerivation {
            widget: suggested,
            errors: errs,
        } = self.widget_derivation(accent);
        for e in errs {
            errors.push(anyhow!("\"Accent component derivation\" failed: {}", e));
        }
        let accent_text = accent_text.unwrap_or(accent);
        let accent_nav_handle_text = accent_nav_handle_text.unwrap_or(accent);

        AccentDerivation {
            accent: Accent {
                accent,
                accent_text,
                accent_nav_handle_text,
                suggested,
            },
            errors,
        }
    }
}
