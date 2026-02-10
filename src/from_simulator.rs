use std::ops::Range;

use serde::Deserialize;
use serde::Serialize;

use crate::Logic;
use crate::SignalInstanceId;
use crate::SignalValueType;
use crate::design_hierarchy::DesignHierarchy;
use crate::time::LogicalTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SimulationUpdate {
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
    pub fn new(signal_instance_id: SignalInstanceId, value_type: SignalValueType) -> Self {
        match value_type {
            SignalValueType::F64 => NewValuesEnum::F64(NewValues::new(signal_instance_id)),
            SignalValueType::Logic => NewValuesEnum::Logic(NewValues::new(signal_instance_id)),
            SignalValueType::U8 => NewValuesEnum::U8(NewValues::new(signal_instance_id)),
        }
    }

    #[must_use]
    pub fn get_signal_instance_id(&self) -> SignalInstanceId {
        match self {
            NewValuesEnum::F64(new_values) => new_values.signal_instance_id,
            NewValuesEnum::Logic(new_values) => new_values.signal_instance_id,
            NewValuesEnum::U8(new_values) => new_values.signal_instance_id,
        }
    }

    pub fn clone_empty(&self) -> Self {
        match self {
            NewValuesEnum::F64(new_values) => NewValuesEnum::F64(new_values.clone_empty()),
            NewValuesEnum::Logic(new_values) => NewValuesEnum::Logic(new_values.clone_empty()),
            NewValuesEnum::U8(new_values) => NewValuesEnum::U8(new_values.clone_empty()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewValues<Value> {
    pub signal_instance_id: SignalInstanceId,
    pub timestamps: Vec<LogicalTime>,
    pub values: Vec<Value>,
}

impl<Value> NewValues<Value> {
    pub fn new(signal_instance_id: SignalInstanceId) -> Self {
        Self {
            signal_instance_id,
            timestamps: vec![],
            values: vec![],
        }
    }

    pub fn clone_empty(&self) -> Self {
        Self {
            signal_instance_id: self.signal_instance_id,
            timestamps: vec![],
            values: vec![],
        }
    }
}
