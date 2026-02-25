use std::fmt;
use std::num::NonZeroU32;

use compact_str::CompactString;
use serde::Deserialize;
use serde::Serialize;

/// Identifier for an instantiated signal in the design hierarchy.
///
/// These IDs are only stable during a simulation, not across multiple simulations.
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct SignalInstanceId(pub NonZeroU32);

impl fmt::Display for SignalInstanceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0.get(), formatter)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DesignHierarchy {
    pub root_modules: Vec<Module>,
}

/// Either a design entity or a package.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Module {
    /// The instance name for [design entities](ModuleKind::DesignEntity),
    /// the package name for [packages](ModuleKind::Package).
    pub name: CompactString,

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
    pub sub_id_start: Option<SignalInstanceId>,
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
        element_type: Box<SignalType>,
    },
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Direction {
    To,
    Downto,
}
