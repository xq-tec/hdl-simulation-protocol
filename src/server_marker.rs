//! Empty marker files under the OS temp directory advertise running GHDL adapter WebSocket ports.

use std::path::PathBuf;

/// Subdirectory of [`std::env::temp_dir()`] where marker files are stored.
pub const SERVER_MARKER_SUBDIR: &str = "hdl-sim";

/// Filename suffix for a marker file (`{port}-{id}.server` with 14-digit lowercase hex `id`).
pub const SERVER_MARKER_SUFFIX: &str = ".server";

/// Width of the hexadecimal simulation instance id in marker file names (covers 53 bits).
pub const SIMULATION_ID_HEX_DIGITS: usize = 14;

/// Returns the directory containing marker files.
#[must_use]
pub fn markers_directory() -> PathBuf {
    std::env::temp_dir().join(SERVER_MARKER_SUBDIR)
}

/// Returns the path to the marker file for `port` and `simulation_id` (53-bit value).
///
/// File name format: `{port}-{simulation_id:014x}.server` (lowercase hex).
#[must_use]
pub fn marker_path(port: u16, simulation_id: u64) -> PathBuf {
    markers_directory().join(format!(
        "{port}-{:0width$x}{SERVER_MARKER_SUFFIX}",
        simulation_id,
        width = SIMULATION_ID_HEX_DIGITS
    ))
}

/// Parses `port` and `simulation_id` from a marker filename such as `54321-01a2b3c4d5e6f7.server`.
#[must_use]
pub fn parse_marker_file_name(file_name: &str) -> Option<(u16, u64)> {
    let stem = file_name.strip_suffix(SERVER_MARKER_SUFFIX)?;
    let (port_str, id_str) = stem.rsplit_once('-')?;
    if id_str.len() != SIMULATION_ID_HEX_DIGITS {
        return None;
    }
    let simulation_id = u64::from_str_radix(id_str, 16).ok()?;
    let port = port_str.parse().ok()?;
    Some((port, simulation_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_path_round_trip() {
        let path = marker_path(54321, 0x01a2b3c4d5e6f7u64);
        let name = path.file_name().unwrap().to_str().unwrap();
        assert_eq!(name, "54321-0001a2b3c4d5e6f7.server");
        assert_eq!(
            parse_marker_file_name(name),
            Some((54321, 0x01a2b3c4d5e6f7u64))
        );
    }

    #[test]
    fn rejects_legacy_port_only_marker() {
        assert!(parse_marker_file_name("54321.server").is_none());
    }
}
