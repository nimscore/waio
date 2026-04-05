//! File watcher for hot-reload of .wa files.
//!
//! Uses `notify-debouncer-mini` with 2s debounce to handle editor write-rename sequences.
//! Events are sent via a calloop channel for processing in the main event loop.

use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult};
use smithay_client_toolkit::reexports::calloop::channel::{self, Channel, Sender};
use std::path::PathBuf;
use std::time::Duration;
use tracing::{error, info};

/// Event from the file watcher.
pub enum FileWatchEvent {
    /// File was modified or deleted — path to the .wa file.
    Changed(PathBuf),
}

/// Manages file watching for hot-reload.
pub struct FileWatcher {
    _debouncer: notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>,
}

impl FileWatcher {
    /// Create a new file watcher and return the receiver channel for calloop insertion.
    pub fn new() -> Result<(Self, Channel<FileWatchEvent>), notify::Error> {
        let (tx, rx) = channel::channel::<FileWatchEvent>();
        let tx_for_watcher = tx.clone();

        let debouncer = new_debouncer(
            Duration::from_secs(2),
            move |res: DebounceEventResult| {
                Self::handle_event(res, &tx_for_watcher);
            },
        )?;

        Ok((Self { _debouncer: debouncer }, rx))
    }

    /// Watch a file for changes.
    pub fn watch(&mut self, path: &std::path::Path) -> notify::Result<()> {
        self._debouncer
            .watcher()
            .watch(path, RecursiveMode::NonRecursive)
    }

    /// Stop watching a file.
    pub fn unwatch(&mut self, path: &std::path::Path) -> notify::Result<()> {
        self._debouncer
            .watcher()
            .unwatch(path)
    }

    fn handle_event(res: DebounceEventResult, tx: &Sender<FileWatchEvent>) {
        match res {
            Ok(events) => {
                for event in events {
                    if event.path.is_dir() {
                        continue;
                    }
                    let path = event.path.clone();
                    // Debounced events don't carry the original event kind.
                    // Just notify that something changed — the main loop will
                    // try to read the file and act accordingly.
                    info!("File changed: {}", path.display());
                    if tx.send(FileWatchEvent::Changed(path)).is_err() {
                        error!("Failed to send file watch event — channel disconnected");
                    }
                }
            }
            Err(e) => {
                error!("File watcher error: {:?}", e);
            }
        }
    }
}
