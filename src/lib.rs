pub mod design_hierarchy;
pub mod from_simulator;
pub mod time;
pub mod to_simulator;

use std::fmt;
use std::fmt::Display;
use std::num::NonZeroU32;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

pub type DesignHierarchyEntryName = String;
pub type SignalName = String;
pub type SimulationId = u64;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum SimulationStatus {
    Paused = 0,
    Running = 1,
    Stopped = 2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SignalValueType {
    F64,
    U8,
    Logic,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum Logic {
    #[default]
    U = 0,
    X,
    Zero,
    One,
    Z,
    W,
    L,
    H,
    DontCare,
}

impl Display for Logic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Logic::U => 'U',
            Logic::X => 'X',
            Logic::Zero => '0',
            Logic::One => '1',
            Logic::Z => 'Z',
            Logic::W => 'W',
            Logic::L => 'L',
            Logic::H => 'H',
            Logic::DontCare => '-',
        };
        Display::fmt(&symbol, f)
    }
}

impl FromStr for Logic {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "U" => Ok(Logic::U),
            "X" => Ok(Logic::X),
            "0" => Ok(Logic::Zero),
            "1" => Ok(Logic::One),
            "Z" => Ok(Logic::Z),
            "W" => Ok(Logic::W),
            "L" => Ok(Logic::L),
            "H" => Ok(Logic::H),
            "-" => Ok(Logic::DontCare),
            _ => Err("input does not match any enum"),
        }
    }
}

impl TryFrom<u8> for Logic {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Logic::U),
            1 => Ok(Logic::X),
            2 => Ok(Logic::Zero),
            3 => Ok(Logic::One),
            4 => Ok(Logic::Z),
            5 => Ok(Logic::W),
            6 => Ok(Logic::L),
            7 => Ok(Logic::H),
            _ => Err(()),
        }
    }
}

/// Identifier for an instantiated signal in the design hierarchy.
///
/// These IDs are only stable during a simulation, not across multiple simulations.
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct SignalInstanceId(pub NonZeroU32);

impl Display for SignalInstanceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0.get(), formatter)
    }
}
