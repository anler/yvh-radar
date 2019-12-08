use serde::{Deserialize, Serialize};

/// Scan Request.
#[derive(Serialize, Deserialize)]
pub struct Scan {
    pub scan: Vec<yvh::Scan>,
    pub protocols: Vec<ProtocolName>,
}

/// Protocol names that can be used in requests.
#[derive(Serialize, Deserialize)]
pub enum ProtocolName {
    #[serde(rename = "closest-enemies")]
    ClosestEnemies,
    #[serde(rename = "furthest-enemies")]
    FurthestEnemies,
    #[serde(rename = "assist-allies")]
    AssistAllies,
    #[serde(rename = "avoid-crossfire")]
    AvoidCrossfire,
    #[serde(rename = "prioritize-mech")]
    PrioritizeMech,
    #[serde(rename = "avoid-mech")]
    AvoidMech,
}
