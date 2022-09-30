use std::hash::Hash;

use crate::{config, Theme, NAME};
use futures::{channel::mpsc, SinkExt, StreamExt};
use iced::{theme::Palette, Color};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use palette::Srgba;

// Just a little utility function
/// utility function for iced cosmic theme subscription
#[cfg(feature = "iced")]
pub fn theme<I: 'static + Hash + Copy + Send + Sync>(
    id: I,
) -> iced::Subscription<(I, ThemeUpdate)> {
    use iced::subscription;

    subscription::unfold(id, State::Ready, move |state| load_theme(id, state))
}

#[cfg(feature = "iced")]
async fn load_theme<I: Copy>(id: I, state: State) -> (Option<(I, ThemeUpdate)>, State) {
    match state {
        State::Ready => {
            if let Ok(watcher) = ThemeWatcher::new() {
                let (palette, color_overrides) = ThemeWatcher::palette();
                (
                    Some((id, ThemeUpdate::Palette(palette, color_overrides))),
                    State::Waiting(watcher),
                )
            } else {
                (Some((id, ThemeUpdate::Errored)), State::Error)
            }
        }
        State::Waiting(mut t) => {
            if let Some((palette, color_overrides)) = t.palette_change().await {
                (
                    Some((id, ThemeUpdate::Palette(palette, color_overrides))),
                    State::Waiting(t),
                )
            } else {
                (Some((id, ThemeUpdate::Errored)), State::Error)
            }
        }
        State::Error => iced::futures::future::pending().await,
    }
}

/// state of the theme subscription
#[cfg(feature = "iced")]
#[derive(Debug)]
pub enum State {
    /// Ready
    Ready,
    /// Waiting for an update
    Waiting(ThemeWatcher),
    /// Error state
    Error,
}

#[cfg(feature = "iced")]
#[derive(Debug, Clone)]
/// theme update message
pub enum ThemeUpdate {
    /// Updated iced palette and theme
    Palette(Palette, Theme<Srgba>),
    /// Errored
    Errored,
}

#[cfg(feature = "iced")]
#[derive(Debug)]
/// Theme watcher
pub struct ThemeWatcher {
    rx: mpsc::Receiver<notify::Event>,
    prev_palette: (Palette, Theme<Srgba>),
}
#[cfg(feature = "iced")]
impl ThemeWatcher {
    pub(crate) fn new() -> anyhow::Result<Self> {
        let prev_palette = Self::palette();
        let (mut tx, rx) = mpsc::channel(20);
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                if let Ok(e) = res {
                    futures::executor::block_on(async {
                        let _ = tx.send(e).await;
                    })
                }
            },
            notify::Config::default(),
        )?;
        for config_dir in xdg_dirs.get_config_dirs() {
            let _ = watcher.watch(&config_dir, RecursiveMode::Recursive);
        }
        for data_dir in xdg_dirs.get_data_dirs() {
            let _ = watcher.watch(&&data_dir.as_ref(), RecursiveMode::Recursive);
        }

        Ok(Self { rx, prev_palette })
    }

    /// Updated iced palette and theme
    pub(crate) fn palette() -> (Palette, Theme<Srgba>) {
        let config = config::Config::load().unwrap_or_default();
        let (mut palette, theme) = config
            .get_active()
            .map(|color_overrides| {
                (
                    if !config.is_dark {
                        Palette::LIGHT
                    } else {
                        Palette::DARK
                    },
                    color_overrides,
                )
            })
            .unwrap_or_else(|_| {
                let (palette, theme) = if config.is_dark {
                    (Palette::DARK, Theme::<Srgba>::default()) // TODO dark default
                } else {
                    (Palette::LIGHT, Theme::<Srgba>::default()) // TODO light default
                };
                if config.is_high_contrast {
                    (palette, theme.to_high_contrast())
                } else {
                    (palette, theme)
                }
            });

        let bg = theme.background.clone();
        palette.background = Color::from_rgba(
            bg.container.red as f32,
            bg.container.green as f32,
            bg.container.blue as f32,
            bg.container.alpha as f32,
        );
        palette.text = Color::from_rgba(
            bg.on_container.red as f32,
            bg.on_container.green as f32,
            bg.on_container.blue as f32,
            bg.on_container.alpha as f32,
        );

        let accent = theme.accent.clone();
        palette.primary = Color::from_rgba(
            accent.base.red as f32,
            accent.base.green as f32,
            accent.base.blue as f32,
            accent.base.alpha,
        );

        let success = theme.success.clone();
        palette.success = Color::from_rgba(
            success.base.red as f32,
            success.base.green as f32,
            success.base.blue as f32,
            success.base.alpha as f32,
        );

        let destructive = theme.destructive.clone();
        palette.danger = Color::from_rgba(
            destructive.base.red as f32,
            destructive.base.green as f32,
            destructive.base.blue as f32,
            destructive.base.alpha as f32,
        );

        (palette, theme)
    }

    pub(crate) async fn palette_change(&mut self) -> Option<(Palette, Theme<Srgba>)> {
        while let Some(e) = self.rx.next().await {
            match e.kind {
                // TODO only notify for changed data file if it is the active file
                notify::EventKind::Create(_)
                | notify::EventKind::Modify(_)
                | notify::EventKind::Remove(_) => {
                    let new_palette = Self::palette();
                    if self.prev_palette != new_palette {
                        self.prev_palette = new_palette.clone();
                        return Some(new_palette);
                    }
                }
                _ => {}
            }
        }
        None
    }
}
