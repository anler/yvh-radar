use super::Coordinates;

/// Calculate the distance from a given coordinate to the origin.
pub fn to_origin(point: &Coordinates) -> u32 {
    // assuming no overflow
    ((point.x.pow(2) + point.y.pow(2)) as f64).sqrt().floor() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to_origin() {
        assert_eq!(2, to_origin(&Coordinates { x: 2, y: 2 }));
        assert_eq!(4, to_origin(&Coordinates { x: 3, y: 3 }));
    }
}
