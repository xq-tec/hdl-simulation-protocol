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
    Subscribe(Vec<SignalElementId>),
    #[serde(rename_all = "camelCase")]
    Unsubscribe(Vec<SignalElementId>),
}
