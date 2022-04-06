use futures::{channel::mpsc::Receiver, SinkExt};
use notify::{Event, INotifyWatcher, Watcher};

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

#[cfg(feature = "gtk4-theme")]
pub mod gtk4_provider;
#[cfg(feature = "gtk4-theme")]
pub use gtk4_provider::*;
