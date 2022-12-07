use anyhow::Error;
use std::str::FromStr;
use anyhow::Context;

pub struct Range<T> {
    min: T,
    max: T,
}

impl<T> Range<T> where T: std::cmp::Ord {
    fn contains(&self, other: &Range<T>) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn bi_contains(&self, other: &Range<T>) -> bool {
        self.contains(other) || other.contains(self)
    }

    fn overlaps(&self, other: &Range<T>) -> bool {
        (self.min <= other.max && self.max >= other.min) || (other.min <= self.max && other.max >= self.min)
    }
}

fn range<T>(input: &str) -> IResult<&str, Range<T>> {
    let (input, (min, _, max)) = tuple((number, tag(","), number))(input)?;
    Ok((input, Range {min, max}))
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

impl<T> FromStr for Range<T> where T: FromStr + std::cmp::Ord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = parse_pair(s, '-').context("Failed to parse range")?;
        Ok(Self { min, max })
    }
}
