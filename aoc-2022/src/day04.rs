    use std::str::FromStr;

    use super::AOC2022;
    use anyhow::Error;
    use aoc_runner::{Day, ParseInput, Part, Solution};

    use anyhow::Context;
    use anyhow::Result;

    pub struct Range<T> {
        min: T,
        max: T,
    }

    impl<T> Range<T>
    where
        T: std::cmp::Ord,
    {
        fn contains(&self, other: &Range<T>) -> bool {
            self.min <= other.min && self.max >= other.max
        }

        fn bi_contains(&self, other: &Range<T>) -> bool {
            self.contains(other) || other.contains(self)
        }

        fn overlaps(&self, other: &Range<T>) -> bool {
            (self.min <= other.max && self.max >= other.min)
                || (other.min <= self.max && other.max >= self.min)
        }
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

impl<T> FromStr for Range<T>
where
    T: FromStr + std::cmp::Ord,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = parse_pair(s, '-').context("Failed to parse range")?;
        Ok(Self { min, max })
    }
}

pub struct Pair<T> {
    first: Range<T>,
    second: Range<T>,
}

impl<T> FromStr for Pair<T>
where
    T: std::cmp::Ord + FromStr,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = parse_pair(s, ',').context("Failed to parse pair")?;
        Ok(Self { first, second })
    }
}

impl ParseInput<'_, { Day::Day4 }> for AOC2022<{ Day::Day4 }> {
    type Parsed = Vec<Pair<u32>>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .lines()
            .map(|line| line.parse().context("Failed to parse line: {}"))
            .collect::<Result<Self::Parsed>>()
    }
}
impl Solution<'_, { Day::Day4 }, { Part::One }> for AOC2022<{ Day::Day4 }> {
    type Input = Vec<Pair<u32>>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .filter(|pair| pair.first.bi_contains(&pair.second))
            .count())
    }
}

impl Solution<'_, { Day::Day4 }, { Part::Two }> for AOC2022<{ Day::Day4 }> {
    type Input = Vec<Pair<u32>>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .filter(|pair| pair.first.overlaps(&pair.second))
            .count())
    }
}
