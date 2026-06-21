use serde::Deserialize;
use serde::Serialize;

use crate::design_hierarchy::SignalElementId;
use crate::time::PhysicalTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Command {
    #[serde(rename_all = "camelCase")]
    RunSimulation { until: RunUntil },
    #[serde(rename_all = "camelCase")]
    PauseSimulation,
    #[serde(rename_all = "camelCase")]
    StopSimulation,

    #[serde(rename_all = "camelCase")]
    Subscribe(Vec<SignalElementId>),
    #[serde(rename_all = "camelCase")]
    Unsubscribe(Vec<SignalElementId>),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RunUntil {
    /// Run until the simulation ends.
    #[serde(rename_all = "camelCase")]
    UntilEnd,

    /// Run until the given time is reached.
    #[serde(rename_all = "camelCase")]
    UntilTime { deadline: PhysicalTime },

    /// Run for the given duration.
    #[serde(rename_all = "camelCase")]
    ForTime { duration: PhysicalTime },
}
