//! Empty marker files under the OS temp directory advertise running GHDL adapter WebSocket ports.

use std::path::Path;
use std::path::PathBuf;

use crate::SimulationId;

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

/// Returns the path to the marker file for `port` and `simulation_id`.
///
/// File name format: `{port}-{simulation_id:014x}.server` (lowercase hex).
#[must_use]
pub fn marker_path(port: u16, simulation_id: SimulationId) -> PathBuf {
    markers_directory().join(format!("{port}-{simulation_id}{SERVER_MARKER_SUFFIX}"))
}

/// Information from simulation marker file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Marker {
    pub port: u16,
    pub simulation_id: SimulationId,
}

impl Marker {
    /// Parses the port and simulation ID from a marker path or file name such as `54321-01a2b3c4d5e6f7.server`.
    #[must_use]
    pub fn try_from_path(path: &Path) -> Option<Self> {
        let file_name = path.file_name()?.to_str()?;
        let stem = file_name.strip_suffix(SERVER_MARKER_SUFFIX)?;
        let (port_str, id_str) = stem.rsplit_once('-')?;
        if id_str.len() != SimulationId::HEX_DIGITS {
            return None;
        }
        let simulation_id = id_str.parse().ok()?;
        let port = port_str.parse().ok()?;

        Some(Self {
            port,
            simulation_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_path_round_trip() {
        let id = 0x01a2b3c4d5e6f7u64.try_into().unwrap();
        let path = marker_path(54321, id);
        assert_eq!(
            path.file_name().unwrap(),
            Path::new("54321-01a2b3c4d5e6f7.server"),
        );
        assert_eq!(
            Marker::try_from_path(&path),
            Some(Marker {
                port: 54321,
                simulation_id: id
            })
        );
    }

    #[test]
    fn rejects_legacy_port_only_marker() {
        assert!(Marker::try_from_path(Path::new("54321.server")).is_none());
    }
}
