pub mod design_hierarchy;
pub mod from_simulator;
pub mod serde_utils;
pub mod server_marker;
pub mod time;
pub mod to_simulator;

use std::fmt;
use std::fmt::Display;
use std::num::NonZeroU64;
use std::ops::RangeInclusive;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

/// Unique identifier for a simulation instance which is safe to use in JavaScript due to its range
/// being within the safe integer range.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SimulationId(u64);

impl SimulationId {
    pub const ZERO: Self = Self(0);

    /// Number of bits which can safely by represented in a JavaScript number.
    const BITS: u32 = 53;
    const MASK: u64 = (1u64 << Self::BITS) - 1;
    const RANGE: RangeInclusive<u64> = NonZeroU64::MIN.get()..=Self::MASK;

    /// Number of hex digits needed to represent the simulation instance id.
    pub const HEX_DIGITS: usize = Self::BITS.div_ceil(4) as usize;

    /// Create a randomly generated new valid simulation instance id.
    pub fn new_random() -> Self {
        // generate a random u64 with fallback to current time if getrandom fails
        let random_u64 = || {
            getrandom::u64().unwrap_or_else(|_| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_or(1u64, |d| d.as_nanos() as u64)
            })
        };
        // retry until we get a non-zero id
        loop {
            if let Some(id) = NonZeroU64::new(random_u64() & Self::MASK) {
                return Self(id.get());
            }
        }
    }

    pub const fn get(&self) -> u64 {
        self.0
    }
}

impl Display for SimulationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0width$x}", self.0, width = Self::HEX_DIGITS)
    }
}

impl FromStr for SimulationId {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        u64::from_str_radix(value, 16)
            .map_err(|error| error.to_string())?
            .try_into()
    }
}

impl TryFrom<u64> for SimulationId {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if !Self::RANGE.contains(&value) {
            return Err(format!(
                "simulation instance id {value} is not in range {start}..={end}",
                start = Self::RANGE.start(),
                end = Self::RANGE.end(),
            ));
        }
        Ok(Self(value))
    }
}

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
    I32,
    I64,
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
