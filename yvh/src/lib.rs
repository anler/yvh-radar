//! # YVH Attack Module

#![deny(rust_2018_idioms)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(missing_docs)]

mod distance;
mod enemies;
mod protocols;
mod scan;

pub use enemies::*;
pub use protocols::*;
pub use scan::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        assert!(true)
    }
}
