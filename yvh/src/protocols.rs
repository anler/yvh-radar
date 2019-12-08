use super::Scan;

/// Protocols to apply to a radar scan.
///
/// This list of protocols will always contain compatible protocols.
pub struct Protocols {
    protocols: Vec<Box<dyn Protocol>>,
}

/// Protocol Interface.
pub trait Protocol {
    /// Receives a list or radar scans and returns the objects
    /// conforming to the protocol rules.
    fn apply(&self, scan: Vec<Scan>) -> Vec<Scan>;
}

/// Protocol that ignores enemies being to far away.
pub struct IgnoreOutOfRange {
    range: u32,
}

impl IgnoreOutOfRange {
    /// Ignore enemies that reside at more than `range_in_meters`
    /// distance.
    pub fn new(range_in_meters: u32) -> Self {
        Self {
            range: range_in_meters,
        }
    }
}

impl Protocol for IgnoreOutOfRange {
    fn apply(&self, scan: Vec<Scan>) -> Vec<Scan> {
        unimplemented!()
    }
}

/// Protocol that prioritizes the closest point with enemies.
pub struct ClosestEnemies;

impl Protocol for ClosestEnemies {
    fn apply(&self, scan: Vec<Scan>) -> Vec<Scan> {
        unimplemented!()
    }
}

/// Protocol that prioritizes the furthest point with enemies.
pub struct FurthestEnemies;

impl Protocol for FurthestEnemies {
    fn apply(&self, scan: Vec<Scan>) -> Vec<Scan> {
        unimplemented!()
    }
}

/// Protocol that prioritizes points with allies.
pub struct AssistAllies;

impl Protocol for AssistAllies {
    fn apply(&self, scan: Vec<Scan>) -> Vec<Scan> {
        unimplemented!()
    }
}

/// Protocol that prioritizes points with mech enemies.
pub struct PrioritizeMech;

impl Protocol for PrioritizeMech {
    fn apply(&self, scan: Vec<Scan>) -> Vec<Scan> {
        unimplemented!()
    }
}

/// Protocol that avoids attacking mech enemies.
pub struct AvoidMech;

impl Protocol for AvoidMech {
    fn apply(&self, scan: Vec<Scan>) -> Vec<Scan> {
        unimplemented!()
    }
}
