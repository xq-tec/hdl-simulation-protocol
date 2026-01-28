use serde::Deserialize;
use serde::Serialize;

use crate::SignalInstanceId;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    StartSimulation,
    PauseSimulation,
    ResumeSimulation,
    RestartSimulation,
    StopSimulation,

    TrackSignals(SignalTrackingRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignalTrackingRequest {
    pub signal_instance_ids: Vec<SignalInstanceId>,
    /// True => tracking is required for this client. False => tracking no longer required for this client
    pub enabled: bool,
    // TODO better handling for unTracking
    pub subscribe: bool,
}
