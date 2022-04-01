use crate::{Config, CSS_DIR, NAME};
use anyhow::{bail, Result};
use futures::{channel::mpsc::Receiver, SinkExt, StreamExt};
use gtk4::{gdk::Display, CssProvider, StyleContext};
use notify::{
    event::{AccessKind, AccessMode, DataChange, EventKind, ModifyKind},
    Event, INotifyWatcher, RecursiveMode, Watcher,
};
use std::path::PathBuf;

fn async_watcher() -> notify::Result<(INotifyWatcher, Receiver<notify::Result<Event>>)> {
    use futures::channel::mpsc::channel;
    let (mut tx, rx) = channel(1);

    let watcher = INotifyWatcher::new(move |res| {
        futures::executor::block_on(async {
            if let Err(e) = tx.send(res).await {
                dbg!(e);
            }
        })
    })?;

    Ok((watcher, rx))
}

pub async fn load_cosmic_gtk_theme(provider: CssProvider) -> Result<()> {
    if !gtk4::is_initialized() {
        bail!("gtk is not initialized.");
    }

    let config_path = Config::config_path()?;
    let mut config = Config::load()?;

    let css_path: PathBuf = [NAME, CSS_DIR].iter().collect();
    let css_dirs = xdg::BaseDirectories::with_prefix(css_path)?;

    let (mut watcher, mut rx) = async_watcher()?;
    let mut theme_css_path =
        if let Some(p) = css_dirs.find_data_file(format!("{}.css", config.active_name())) {
            let _ = watcher.watch(&p, RecursiveMode::NonRecursive);
            provider.load_from_path(&p);
            // Add the provider to the default screen

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
                provider.load_from_path(&theme_css_path);
            }

            Err(e) => {
                eprintln!("{}", e);
            }
            _ => {}
        }
    }
    Ok(())
}
