#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Denotes the number and kind of enemies.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Enemies {
    /// The type of this enemy.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub kind: EnemyKind,
    /// How many units of this enemy are.
    pub number: u32,
}

impl Enemies {
    /// Returns true if it's a mech enemy.
    pub fn is_mech(&self) -> bool {
        match self.kind {
            EnemyKind::Mech => true,
            _ => false,
        }
    }
}

/// Denotes the different kinds of enemy.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum EnemyKind {
    /// Enemies that are soldier.
    Soldier,
    /// Enemies that are mechanical vehicles.
    Mech,
}
