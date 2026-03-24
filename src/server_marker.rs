//! Empty marker files under the OS temp directory advertise running GHDL adapter WebSocket ports.

use std::path::PathBuf;

/// Subdirectory of [`std::env::temp_dir()`] where marker files are stored.
pub const SERVER_MARKER_SUBDIR: &str = "hdl-sim";

/// Filename suffix for a marker file (`{port}{SERVER_MARKER_SUFFIX}`).
pub const SERVER_MARKER_SUFFIX: &str = ".server";

/// Returns the directory containing `{port}.server` marker files.
#[must_use]
pub fn markers_directory() -> PathBuf {
    std::env::temp_dir().join(SERVER_MARKER_SUBDIR)
}

/// Returns the path to the marker file for `port`.
#[must_use]
pub fn marker_path(port: u16) -> PathBuf {
    markers_directory().join(format!("{port}{SERVER_MARKER_SUFFIX}"))
}

/// Parses `port` from a marker filename such as `54321.server`.
#[must_use]
pub fn parse_marker_port(file_name: &str) -> Option<u16> {
    let stem = file_name.strip_suffix(SERVER_MARKER_SUFFIX)?;
    stem.parse().ok()
}
