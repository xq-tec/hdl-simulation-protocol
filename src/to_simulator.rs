use serde::Deserialize;
use serde::Serialize;

use crate::design_hierarchy::SignalElementId;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Command {
    #[serde(rename_all = "camelCase")]
    StartSimulation,
    #[serde(rename_all = "camelCase")]
    PauseSimulation,
    #[serde(rename_all = "camelCase")]
    ResumeSimulation,
    #[serde(rename_all = "camelCase")]
    StopSimulation,

    #[serde(rename_all = "camelCase")]
    TrackSignals(SignalTrackingRequest),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignalTrackingRequest {
    pub signal_element_ids: Vec<SignalElementId>,
    /// True => tracking is required for this client. False => tracking no longer required for this client
    pub enabled: bool,
    // TODO better handling for unTracking
    pub subscribe: bool,
}
