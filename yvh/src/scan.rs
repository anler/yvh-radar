#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{distance, Enemies};

/// Coordinates of a radar scan object.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Coordinates {
    /// The X coordinate.
    pub x: i32,
    /// The Y coordinate.
    pub y: i32,
}

/// Denotes a radar scan, which yields the enemies residing at a given
/// coordinate.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Scan {
    /// Coordinates of the radar scan.
    pub coordinates: Coordinates,
    /// Enemies residing at that coordinate.
    pub enemies: Enemies,
    /// Allies residing at that coordinate.
    pub allies: Option<u32>,
}

/// A radar scan with augmented information, e.g. distance.
pub struct AugmentedScan {
    scan: Scan,
    distance: u32,
}

impl AugmentedScan {
    /// Return the distance from this scan to the origin.
    pub fn distance(&self) -> u32 {
        self.distance
    }

    /// Return a reference to the underlying scan.
    pub fn scan(&self) -> &Scan {
        &self.scan
    }
}

impl From<Scan> for AugmentedScan {
    fn from(scan: Scan) -> Self {
        let distance = distance::to_origin(&scan.coordinates);
        Self { scan, distance }
    }
}
