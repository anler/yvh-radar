use std::cmp::Ordering;

use super::{AugmentedScan, EnemyKind};

/// Protocol Interface.
pub trait Protocol {
    /// Receives a list or radar scans and returns the objects
    /// conforming to the protocol rules.
    fn apply(&self, scan: Vec<AugmentedScan>) -> Vec<AugmentedScan>;
}

/// A collection of combined protocols.
pub struct Protocols {
    protocols: Vec<Box<dyn Protocol>>,
}

impl<I> From<I> for Protocols
where
    I: Iterator<Item = Box<dyn Protocol>>,
{
    fn from(iterator: I) -> Self {
        Self {
            protocols: iterator.collect(),
        }
    }
}

impl Protocol for Protocols {
    fn apply(&self, mut scan: Vec<AugmentedScan>) -> Vec<AugmentedScan> {
        for protocol in &self.protocols {
            scan = protocol.apply(scan)
        }
        scan
    }
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
    fn apply(&self, scan: Vec<AugmentedScan>) -> Vec<AugmentedScan> {
        scan.into_iter()
            .filter(|scan| scan.distance() <= self.range)
            .collect()
    }
}

/// Protocol that prioritizes the closest point with enemies.
pub struct ClosestEnemies;

impl Protocol for ClosestEnemies {
    fn apply(&self, mut scan: Vec<AugmentedScan>) -> Vec<AugmentedScan> {
        scan.sort_by(|a, b| a.distance().cmp(&b.distance()));
        scan
    }
}

/// Protocol that prioritizes the furthest point with enemies.
pub struct FurthestEnemies;

impl Protocol for FurthestEnemies {
    fn apply(&self, mut scan: Vec<AugmentedScan>) -> Vec<AugmentedScan> {
        scan.sort_by(|a, b| b.distance().cmp(&a.distance()));
        scan
    }
}

/// Protocol that prioritizes points with more allies.
pub struct AssistAllies;

impl Protocol for AssistAllies {
    fn apply(&self, mut scan: Vec<AugmentedScan>) -> Vec<AugmentedScan> {
        scan.sort_by(|a, b| match (a.scan().allies, b.scan().allies) {
            (Some(allies_a), Some(allies_b)) => allies_a.cmp(&allies_b),
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            _ => Ordering::Equal,
        });
        scan
    }
}

/// Protocol that prioritizes points with more mech enemies.
pub struct PrioritizeMech;

impl Protocol for PrioritizeMech {
    fn apply(&self, mut scan: Vec<AugmentedScan>) -> Vec<AugmentedScan> {
        scan.sort_by(|a, b| {
            let enemies_a = &a.scan().enemies;
            let enemies_b = &b.scan().enemies;

            match (&enemies_a.kind, &enemies_b.kind) {
                (EnemyKind::Mech, EnemyKind::Mech) => Ordering::Equal,
                (EnemyKind::Mech, _) => Ordering::Less,
                (_, EnemyKind::Mech) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        });
        scan
    }
}

/// Protocol that avoids attacking mech enemies.
pub struct AvoidMech;

impl Protocol for AvoidMech {
    fn apply(&self, scan: Vec<AugmentedScan>) -> Vec<AugmentedScan> {
        scan.into_iter()
            .filter(|scan| !scan.scan().enemies.is_mech())
            .collect()
    }
}

/// Protocol that avoids attacking enemies being attacked by allies.
pub struct AvoidCrossfire;

impl Protocol for AvoidCrossfire {
    fn apply(&self, scan: Vec<AugmentedScan>) -> Vec<AugmentedScan> {
        scan.into_iter()
            .filter(|scan| scan.scan().allies.is_none())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_ignore_out_of_range_protocol() {
        let scan = vec![
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 3, y: 3 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 2,
                },
                allies: None,
            }),
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 5, y: 5 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 2,
                },
                allies: None,
            }),
        ];
        let protocol = IgnoreOutOfRange::new(5);

        assert_eq!(1, protocol.apply(scan).len());
    }

    #[test]
    fn test_closest_enemies_protocol() {
        let scan = vec![
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 5, y: 35 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 10,
                },
                allies: None,
            }),
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 10, y: 30 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 20,
                },
                allies: None,
            }),
        ];
        let protocol = ClosestEnemies;
        let distances = protocol
            .apply(scan)
            .into_iter()
            .map(|scan| scan.distance())
            .collect::<Vec<_>>();

        assert_eq!(vec![31, 35], distances);
    }

    #[test]
    fn test_furthest_enemies_protocol() {
        let scan = vec![
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 5, y: 35 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 10,
                },
                allies: None,
            }),
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 10, y: 30 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 20,
                },
                allies: None,
            }),
        ];
        let protocol = FurthestEnemies;
        let distances = protocol
            .apply(scan)
            .into_iter()
            .map(|scan| scan.distance())
            .collect::<Vec<_>>();

        assert_eq!(vec![35, 31], distances);
    }

    #[test]
    fn test_assist_allies_protocol() {
        let scan = vec![
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 5, y: 35 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 10,
                },
                allies: Some(3),
            }),
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 10, y: 30 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 20,
                },
                allies: None,
            }),
        ];
        let protocol = AssistAllies;
        let results = protocol.apply(scan);
        let first = results.first().unwrap();

        assert_eq!(5, first.scan().coordinates.x);
        assert_eq!(35, first.scan().coordinates.y);
    }

    #[test]
    fn test_prioritize_mech_protocol() {
        let scan = vec![
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 0, y: 40 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 10,
                },
                allies: None,
            }),
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 0, y: 80 },
                enemies: Enemies {
                    kind: EnemyKind::Mech,
                    number: 1,
                },
                allies: Some(5),
            }),
        ];
        let protocol = PrioritizeMech;
        let results = protocol.apply(scan);
        let first = results.first().unwrap();

        assert_eq!(0, first.scan().coordinates.x);
        assert_eq!(80, first.scan().coordinates.y);
    }

    #[test]
    fn test_avoid_mech_protocol() {
        let scan = vec![
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 0, y: 40 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 10,
                },
                allies: None,
            }),
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 0, y: 80 },
                enemies: Enemies {
                    kind: EnemyKind::Mech,
                    number: 1,
                },
                allies: Some(5),
            }),
        ];
        let protocol = AvoidMech;
        let results = protocol.apply(scan);
        let first = results.first().unwrap();

        assert_eq!(0, first.scan().coordinates.x);
        assert_eq!(40, first.scan().coordinates.y);
    }

    #[test]
    fn test_avoid_crossfire_protocol() {
        let scan = vec![
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 5, y: 35 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 10,
                },
                allies: Some(3),
            }),
            AugmentedScan::from(Scan {
                coordinates: Coordinates { x: 35, y: 5 },
                enemies: Enemies {
                    kind: EnemyKind::Soldier,
                    number: 20,
                },
                allies: None,
            }),
        ];
        let protocol = AvoidCrossfire;
        let results = protocol.apply(scan);
        let first = results.first().unwrap();

        assert_eq!(35, first.scan().coordinates.x);
        assert_eq!(5, first.scan().coordinates.y);
    }
}
