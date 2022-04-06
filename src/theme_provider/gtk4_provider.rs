use super::async_watcher;
use crate::{Config, CSS_DIR, NAME};
use anyhow::{bail, Result};
use futures::StreamExt;
use gtk4::CssProvider;
use notify::{
    event::{AccessKind, AccessMode, EventKind, ModifyKind},
    RecursiveMode, Watcher,
};
use std::path::PathBuf;

/// Function for loading and watching the cosmic gtk theme
pub async fn load_cosmic_gtk4_theme(provider: CssProvider) -> Result<()> {
    if !gtk4::is_initialized() {
        bail!("gtk is not initialized.");
    }

    let config_path = Config::config_path()?;
    let mut config = Config::load()?;

    let css_path: PathBuf = [NAME, CSS_DIR].iter().collect();
    let css_dirs = xdg::BaseDirectories::with_prefix(css_path)?;

    let base_theme = if let Some(path) = css_dirs.find_data_file("base_theme.css") {
        std::fs::read_to_string(path)?
    } else {
        bail!("Failed to find base theme")
    };

    let (mut watcher, mut rx) = async_watcher()?;
    let mut theme_css_path =
        if let Some(p) = css_dirs.find_data_file(format!("{}.css", config.active_name())) {
            let _ = watcher.watch(&p, RecursiveMode::NonRecursive);
            let theme_colors = std::fs::read_to_string(&p).unwrap_or_default();
            provider.load_from_data(&format!("{base_theme}\n{theme_colors}\n").as_bytes());
            p
        } else {
            dbg!(css_dirs.get_data_home());
            dbg!(css_dirs.get_data_dirs());
            PathBuf::new()
        };

    // if configs do not exist, they will not be monitored
    watcher.watch(&config_path, RecursiveMode::NonRecursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event)
                if event.paths.iter().find(|p| **p == config_path).is_some()
                    && event.kind == EventKind::Access(AccessKind::Close(AccessMode::Write))
                    || event.kind == EventKind::Modify(ModifyKind::Any) =>
            {
                let _ = watcher.unwatch(&theme_css_path);
                config = Config::load()?;
                if let Some(p) = css_dirs.find_data_file(format!("{}.css", config.active_name())) {
                    let _ = watcher.watch(&p, RecursiveMode::NonRecursive);
                    theme_css_path = p
                }
            }
            Ok(event)
                if event.paths.iter().find(|p| **p == theme_css_path).is_some()
                    && event.kind == EventKind::Access(AccessKind::Close(AccessMode::Write))
                    || event.kind == EventKind::Modify(ModifyKind::Any) =>
            {
                let theme_colors = std::fs::read_to_string(&theme_css_path).unwrap_or_default();
                provider.load_from_data(&format!("{base_theme}\n{theme_colors}\n").as_bytes());
            }

            Err(e) => {
                eprintln!("{}", e);
            }
            _ => {}
        }
    }
    Ok(())
}
