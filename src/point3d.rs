use num::Signed;
use std::cmp::{max, min};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

pub fn manhattan_distance<T>(l: &Point3D<T>, r: &Point3D<T>) -> T
where
    T: Signed + std::cmp::Ord + Copy,
{
    max(l.x, r.x).abs_sub(&min(l.x, r.x))
        + max(l.y, r.y).abs_sub(&min(l.y, r.y))
        + max(l.z, r.z).abs_sub(&min(l.z, r.z))
}

pub fn euclidean_distance_squared<T>(l: Point3D<T>, r: Point3D<T>) -> T
where
    T: Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Copy,
    T: Signed,
{
    (r - l) * (r - l)
}

impl<T> FromStr for Point3D<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        Ok(Point3D {
            x: split
                .next()
                .unwrap()
                .parse()
                .expect("Failed to parse coordinate"),
            y: split
                .next()
                .unwrap()
                .parse()
                .expect("Failed to parse coordinate"),
            z: split
                .next()
                .unwrap()
                .parse()
                .expect("Failed to parse coordinate"),
        })
    }
}

impl<T> Display for Point3D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Point3D { x, y, z } = &self;
        write!(f, "({x},{y},{z})")
    }
}

impl<T> Debug for Point3D<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Point3D { x, y, z } = &self;
        write!(f, "({x:?},{y:?},{z:?})")
    }
}

impl<T> Add for Point3D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Add<T> for Point3D<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T> Sub for Point3D<T>
where
    T: Sub<Output = T>,
    T: Signed,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<T> for Point3D<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Mul for Point3D<T>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = T;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}
