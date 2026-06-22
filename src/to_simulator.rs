use serde::Deserialize;
use serde::Serialize;

use crate::design_hierarchy::SignalElementId;
use crate::time::PhysicalTime;

/// A command to control the simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    /// Starts or resumes the simulation.
    RunSimulation { until: RunUntil },

    /// Pauses the simulation.
    PauseSimulation,

    /// Stops the simulation.
    StopSimulation,

    /// Subscribes to the given signals.
    Subscribe(Vec<SignalElementId>),

    /// Unsubscribes from the given signals.
    Unsubscribe(Vec<SignalElementId>),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum RunUntil {
    /// Run until the simulation ends.
    UntilEnd,

    /// Run until the given time is reached.
    UntilTime { deadline: PhysicalTime },

    /// Run for the given duration.
    ForTime { duration: PhysicalTime },
}
