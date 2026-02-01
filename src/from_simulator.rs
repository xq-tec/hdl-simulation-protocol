use std::ops::Range;

use serde::Deserialize;
use serde::Serialize;

use crate::Logic;
use crate::SignalInstanceId;
use crate::SimulationId;
use crate::design_hierarchy::DesignHierarchy;
use crate::time::LogicalTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulationUpdate {
    pub simulation_id: SimulationId,
    pub message: Notification,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Notification {
    SimulationStarted,
    SimulationPaused,
    SimulationResumed,
    SimulationStopped,
    DesignHierarchy(DesignHierarchy),
    SignalValuesInRange(SignalValuesInRange),
    NewSimulationTime(LogicalTime),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignalValuesInRange {
    pub time_range: Range<LogicalTime>,
    pub values_in_range: Vec<NewValuesEnum>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NewValuesEnum {
    F64(NewValues<f64>),
    Logic(NewValues<Logic>),
    U8(NewValues<u8>),
}

impl NewValuesEnum {
    #[must_use]
    pub fn get_signal_instance_id(&self) -> SignalInstanceId {
        match self {
            NewValuesEnum::F64(new_values) => new_values.signal_instance_id,
            NewValuesEnum::Logic(new_values) => new_values.signal_instance_id,
            NewValuesEnum::U8(new_values) => new_values.signal_instance_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewValues<Value> {
    pub signal_instance_id: SignalInstanceId,
    pub timestamps: Vec<LogicalTime>,
    pub values: Vec<Value>,
}
