//! Reads a `.env` file into the process environment.
//!
//! Hand rolled rather than pulled from a crate. This is a development
//! convenience that runs once at startup and does one simple thing, so carrying
//! a dependency for it is not worth the weight. In any real deployment the
//! environment is set by the container or the orchestrator and no file exists at
//! all, which is why a missing file is not an error.

use std::fs;

/// Loads `path` if it exists, setting any variable not already present.
///
/// Values already in the environment win. A real environment variable should
/// beat a checked in default, otherwise a stale file could silently override
/// what a deployment explicitly set.
pub fn load(path: &str) {
    let Ok(contents) = fs::read_to_string(path) else {
        return;
    };

    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        let key = key.trim();
        if key.is_empty() || std::env::var_os(key).is_some() {
            continue;
        }

        // Quotes are how a value carrying spaces or a '#' is written, so strip
        // them from the value rather than storing them.
        let value = value.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|v| v.strip_suffix('"'))
            .or_else(|| value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')))
            .unwrap_or(value);

        // Safety: this runs once at startup, before any threads are spawned.
        // Setting environment variables is only unsound with concurrent readers.
        unsafe {
            std::env::set_var(key, value);
        }
    }
}
