use std::fmt::Debug;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> FromStr for Point2D<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once(',').expect("Failed to find comma");
        Ok(Point2D {
            x: split.0.parse().expect("Failed to parse coordinate"),
            y: split.1.parse().expect("Failed to parse coordinate"),
        })
    }
}
