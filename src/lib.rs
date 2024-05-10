//!  # WatchIt!
//!
//! We've got our eyes on your files. WatchIt will run your callback when a file changes. It's easy to use and simple to understand. WatchIt is cross platform and works on Linux, BSD, Mac and Windows.
//!
//! ## Usage
//!
//! Add watchit to your cargo.toml:
//!
//! ```toml
//! [dependencies]
//!     watchit = "0.1"
//! ```
//!
//! Create and instance of the Watcher with a callback:
//!
//! ```Rust
//! let mut watcher = Watcher::new(|event| println!(event));
//! ```
//!
//! Add a file to be watched:
//!
//! ```Rust
//! watcher.watch("file.txt");
//! ```

use std::{path::Path, time::Duration};

pub use notify::Error;
use notify::{RecursiveMode, Watcher as _};
use notify_debouncer_full::{self, new_debouncer, DebounceEventHandler};
use tracing;

/// A watcher that monitors files for changes and debounces events.
///
/// The `Watcher` struct is responsible for setting up a file watcher and debouncing
/// file change events. It uses the `notify` crate to watch for file changes, and the
/// `notify-debouncer-full` crate to debounce those events.
pub struct Watcher {
    debouncer: notify_debouncer_full::Debouncer<
        notify::RecommendedWatcher,
        notify_debouncer_full::FileIdMap,
    >,
}

impl Watcher {
    /// Creates a new file watcher with the provided debounce event handler.
    ///
    /// The file watcher will debounce events for 2 seconds before triggering the provided handler.
    /// This helps to reduce the number of events that need to be processed, especially when
    /// many files are being watched and modified in quick succession.
    ///
    /// # Arguments
    /// * `handler` - The debounce event handler to call when a file change is detected.
    ///
    /// # Returns
    /// A new instance of the file watcher.
    pub fn new(handler: impl DebounceEventHandler) -> Self {
        let result = Self {
            debouncer: new_debouncer(Duration::from_secs(2), None, handler).unwrap(),
        };
        tracing::debug!("Created new file watcher");
        result
    }

    /// Watches the specified file for changes.
    ///
    /// This function sets up a file watcher to monitor the specified file for any changes.
    /// When a change is detected, the file is added to the debouncer's cache to be processed later.
    /// The function returns a `Result` indicating whether the file watcher was successfully set up.
    ///
    /// # Arguments
    /// * `filename` - The path to the file to be watched.
    ///
    /// # Returns
    /// A `Result` containing either an empty `()` value on success, or an `Error` on failure.
    pub fn watch(&mut self, filename: &str) -> Result<(), Error> {
        let result = self
            .debouncer
            .watcher()
            .watch(Path::new(filename), RecursiveMode::NonRecursive);

        self.debouncer
            .cache()
            .add_root(Path::new(filename), RecursiveMode::NonRecursive);

        tracing::debug!("Watching file for changes: {}", filename);

        result
    }
}

#[cfg(test)]
/// This module contains tests for the functionality of the `Watcher` struct.
///
/// The `it_works` test verifies that the `Watcher` correctly detects changes to a file.
/// It creates a file, sets up a `Watcher` to monitor the file, and then checks that the
/// `Watcher` correctly reports the file change event.
mod tests {
    use std::{fs::File, io::Write as _, thread::sleep};

    use super::*;

    #[test]
    fn it_works() {
        static mut FILE_CHANGED: bool = false;
        let mut file = File::create(Path::new("test.testfile")).unwrap();
        let mut watcher = Watcher::new(move |event| {
            tracing::debug!("Event: {:#?}", event);
            unsafe { FILE_CHANGED = true };
        });
        watcher.watch("test.testfile").unwrap();
        assert_eq!(unsafe { FILE_CHANGED }, false);
        file.write_all(b"test").unwrap();
        file.flush().unwrap();
        drop(file);
        sleep(Duration::from_secs(4));
        assert_eq!(unsafe { FILE_CHANGED }, true);
        std::fs::remove_file(Path::new("test.testfile")).unwrap();
    }
}
