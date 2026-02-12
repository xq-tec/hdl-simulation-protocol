use std::fmt;
use std::ops;
use std::ops::Range;

use serde::Deserialize;
use serde::Serialize;

// TODO should this be an i64 to reflect VHDL's time type
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize, Hash)]
pub struct PhysicalTime(pub u64);

impl PhysicalTime {
    pub const ZERO: Self = Self(0);
    pub const MIN: Self = Self::ZERO;
    pub const MAX: Self = Self(u64::MAX);

    #[must_use]
    pub fn empty_time_span() -> Range<PhysicalTime> {
        Self::ZERO..Self::ZERO
    }
}

impl From<u64> for PhysicalTime {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl ops::Sub<Self> for PhysicalTime {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // TODO is this what we want?
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl ops::Div<u64> for PhysicalTime {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl ops::Mul<u64> for PhysicalTime {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl ops::Rem for PhysicalTime {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl From<f64> for PhysicalTime {
    fn from(value: f64) -> Self {
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "required for JavaScript value conversion"
        )]
        Self(value as u64)
    }
}

impl ops::Add for PhysicalTime {
    type Output = Self;

    fn add(self, rhs: PhysicalTime) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign for PhysicalTime {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl ops::Div for PhysicalTime {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl ops::Mul for PhysicalTime {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl fmt::Debug for PhysicalTime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "PhysicalTime({time} fs)", time = self.0)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Hash, Default, PartialEq)]
pub struct LogicalTime {
    pub physical: PhysicalTime,
    pub delta: Delta,
}

impl LogicalTime {
    #[must_use]
    pub fn new(physical: PhysicalTime, delta: Delta) -> Self {
        LogicalTime { physical, delta }
    }

    pub const ZERO: Self = Self {
        physical: PhysicalTime::ZERO,
        delta: Delta::ZERO,
    };

    pub const MIN: Self = Self::ZERO;

    pub const MAX: Self = Self {
        physical: PhysicalTime::MAX,
        delta: Delta::MAX,
    };
}

impl fmt::Display for LogicalTime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "({time} fs, {delta})",
            time = self.physical.0,
            delta = self.delta,
        )
    }
}

impl fmt::Debug for LogicalTime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl ops::Add<Delta> for LogicalTime {
    type Output = Self;

    fn add(self, rhs: Delta) -> Self::Output {
        LogicalTime {
            physical: self.physical,
            delta: self.delta + rhs,
        }
    }
}

impl PartialEq<PhysicalTime> for LogicalTime {
    fn eq(&self, other: &PhysicalTime) -> bool {
        &self.physical == other
    }
}

impl Eq for LogicalTime {}

impl PartialOrd for LogicalTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for LogicalTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.physical
            .cmp(&other.physical)
            .then(self.delta.cmp(&other.delta))
    }
}

impl PartialOrd<PhysicalTime> for LogicalTime {
    fn partial_cmp(&self, other: &PhysicalTime) -> Option<std::cmp::Ordering> {
        Some(self.physical.cmp(other))
    }
}

impl From<PhysicalTime> for LogicalTime {
    fn from(time: PhysicalTime) -> Self {
        LogicalTime {
            physical: time,
            delta: Delta::ZERO,
        }
    }
}

impl From<u64> for LogicalTime {
    fn from(time: u64) -> Self {
        LogicalTime {
            physical: PhysicalTime(time),
            delta: Delta::ZERO,
        }
    }
}

impl From<(u64, u64)> for LogicalTime {
    fn from((physical_time, delta): (u64, u64)) -> Self {
        LogicalTime {
            physical: PhysicalTime(physical_time),
            delta: Delta(delta),
        }
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize, Hash,
)]
pub struct Delta(pub u64);

impl Delta {
    pub const ZERO: Delta = Self(0);
    pub const MAX: Delta = Self(u64::MAX);
}

impl From<u64> for Delta {
    fn from(value: u64) -> Self {
        Delta(value)
    }
}

impl ops::Add for Delta {
    type Output = Self;

    fn add(self, rhs: Delta) -> Self::Output {
        Delta(self.0 + rhs.0)
    }
}

impl fmt::Display for Delta {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{delta} δ", delta = self.0)
    }
}

impl ops::Div<Delta> for Delta {
    type Output = Delta;

    fn div(self, rhs: Delta) -> Self::Output {
        Delta(self.0 / rhs.0)
    }
}

impl ops::Mul<u64> for Delta {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl ops::Sub<Delta> for Delta {
    type Output = Self;

    fn sub(self, rhs: Delta) -> Self::Output {
        Delta(self.0 - rhs.0)
    }
}
