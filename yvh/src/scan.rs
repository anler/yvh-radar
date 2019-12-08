use super::Enemies;

/// Coordinate of a radar scan object.
pub struct Coordinate {
    /// The X coordinate.
    pub x: u32,
    /// The Y coordinate.
    pub y: u32,
}

/// Denotes a radar scan, which yields the enemies residing at a given
/// coordinate.
pub struct Scan {
    coordinates: Coordinate,
    enemies: Enemies,
}
