use std::fmt;
use std::num::NonZeroU32;

use compact_str::CompactString;
use serde::Deserialize;
use serde::Serialize;

/// Identifier for an instantiated signal in the design hierarchy.
///
/// These IDs are only stable during a simulation, not across multiple simulations.
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignalInstanceId(pub NonZeroU32);

impl fmt::Display for SignalInstanceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0.get(), formatter)
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignalElementId {
    pub signal_id: SignalInstanceId,
    pub element_index: u32,
}

/// Elaborated design tree for one simulation instance.
///
/// `simulation_id` is a 53-bit instance identifier assigned by the simulator; it matches the
/// marker filename and must be set before the hierarchy is sent to clients.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DesignHierarchy {
    /// Stable identifier for this simulation run (53-bit value, carried as `u64`).
    pub simulation_id: u64,
    pub root_modules: Vec<Module>,
}

/// Either a design entity or a package.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Module {
    /// The instance name for [design entities](ModuleKind::DesignEntity),
    /// the package name for [packages](ModuleKind::Package).
    pub name: Option<CompactString>,

    pub kind: ModuleKind,
    pub submodules: Vec<Module>,
    pub signals: Vec<Signal>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ModuleKind {
    DesignEntity {
        entity: CompactString,
        architecture: CompactString,
    },
    Package,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Signal {
    pub name: CompactString,
    pub id: SignalInstanceId,
    pub typ: SignalType,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SignalType {
    Bit,
    /// IEEE 1164: U, X, 0, 1, Z, W, L, H, -
    Logic,
    Integer {
        min: i64,
        max: i64,
        direction: Direction,
    },
    Real {
        min: f64,
        max: f64,
        direction: Direction,
    },
    Array {
        left: i32,
        right: i32,
        direction: Direction,
        /// The total number of scalar elements in the array, including nested arrays.
        element_count: u32,
        element_type: Box<SignalType>,
    },
    Unsupported,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Direction {
    To,
    Downto,
}

impl Direction {
    pub fn length_for(&self, left: i32, right: i32) -> u32 {
        match self {
            Direction::To => {
                if right >= left {
                    (right - left + 1) as u32
                } else {
                    0
                }
            },
            Direction::Downto => {
                if left >= right {
                    (left - right + 1) as u32
                } else {
                    0
                }
            },
        }
    }
}
