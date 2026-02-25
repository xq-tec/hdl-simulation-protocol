use std::ops::Range;

use serde::Deserialize;
use serde::Serialize;

use crate::design_hierarchy::DesignHierarchy;
use crate::design_hierarchy::SignalInstanceId;
use crate::time::LogicalTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SimulationUpdate {
    SimulationStarted,
    SimulationPaused,
    SimulationResumed,
    SimulationStopped,
    DesignHierarchy(DesignHierarchy),
    Events(EventsUpdate),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventsUpdate {
    pub time_range: Range<LogicalTime>,
    pub signals: Vec<SignalEvents>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignalEvents {
    pub signal_instance_id: SignalInstanceId,
    pub events: Vec<Event>,
}

impl SignalEvents {
    pub fn new(signal_instance_id: SignalInstanceId) -> Self {
        Self {
            signal_instance_id,
            events: vec![],
        }
    }

    pub fn clone_empty(&self) -> Self {
        Self {
            signal_instance_id: self.signal_instance_id,
            events: vec![],
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Event {
    pub time: LogicalTime,
    pub value: RawValue,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RawValue(pub u64);

impl From<f64> for RawValue {
    fn from(value: f64) -> Self {
        Self(value.to_bits())
    }
}
