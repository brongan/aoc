use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::recognize;
use nom::sequence::separated_pair;
use nom::IResult;
use num::Signed;
use std::cmp::{max, min};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub fn manhattan_distance<T>(l: &Point2D<T>, r: &Point2D<T>) -> T
where
    T: Signed + std::cmp::Ord + Copy,
{
    max(l.x, r.x).abs_sub(&min(l.x, r.x)) + max(l.y, r.y).abs_sub(&min(l.y, r.y))
}

pub fn recognize_point2d(input: &str) -> IResult<&str, &str> {
    recognize(separated_pair(digit1, tag(","), digit1))(input)
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

impl<T> Display for Point2D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T> Debug for Point2D<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?},{:?})", self.x, self.y)
    }
}

impl<T> Add for Point2D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Add<T> for Point2D<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T> Sub for Point2D<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Point2D<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recognize() {
        assert_eq!(recognize_point2d("123,456\n "), Ok(("\n ", "123,456")));
        assert!(recognize_point2d("a,b").is_err());
    }
}
