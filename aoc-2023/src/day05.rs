use super::AOC2023;
use anyhow::{Context, Result};
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace1, newline},
    combinator::{map, map_res},
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use rayon::prelude::*;

type Num = u64;

#[derive(Debug, PartialEq, Clone)]
pub struct RangeMapping {
    destination_start: Num,
    source_start: Num,
    length: Num,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CategoryMap {
    range_mappings: Vec<RangeMapping>,
}

pub struct Almanac {
    seeds: Vec<Num>,
    maps: Vec<CategoryMap>,
}

fn parse_num(input: &str) -> IResult<&str, Num> {
    map_res(digit1, |num: &str| Num::from_str_radix(num, 10))(input)
}

fn parse_range_mapping(input: &str) -> IResult<&str, RangeMapping> {
    map(
        tuple((
            parse_num,
            preceded(multispace1, parse_num),
            preceded(multispace1, parse_num),
        )),
        |(destination_start, source_start, length)| RangeMapping {
            destination_start,
            source_start,
            length,
        },
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, CategoryMap> {
    let parse_map_title = |input| terminated(take_until("\n"), newline)(input);
    map(
        preceded(
            parse_map_title,
            separated_list1(newline, parse_range_mapping),
        ),
        |mappings| CategoryMap {
            range_mappings: mappings,
        },
    )(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<Num>> {
    preceded(tag("seeds: "), separated_list1(multispace1, parse_num))(input)
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let parse_maps = |input| separated_list1(count(newline, 2), parse_map)(input);

    map(
        separated_pair(parse_seeds, count(newline, 2), parse_maps),
        |(seeds, maps)| Almanac { seeds, maps },
    )(input)
}

impl RangeMapping {
    fn map(&self, value: Num) -> Option<Num> {
        if value >= self.source_start && value < self.source_start + self.length {
            Some(self.destination_start + (value - self.source_start))
        } else {
            None
        }
    }
}

impl CategoryMap {
    fn map(&self, value: Num) -> Num {
        for mapping in &self.range_mappings {
            if let Some(seed) = mapping.map(value) {
                return seed;
            }
        }
        value
    }
}

fn find_location(seed: Num, maps: &[CategoryMap]) -> Num {
    maps.iter()
        .fold(seed, |value: Num, map: &CategoryMap| map.map(value))
}

impl ParseInput<'_, { Day::Day5 }> for AOC2023<{ Day::Day5 }> {
    type Parsed = Almanac;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, ret) = parse_almanac(input).map_err(|e| e.to_owned())?;
        Ok(ret)
    }
}

impl Solution<'_, { Day::Day5 }, { Part::One }> for AOC2023<{ Day::Day5 }> {
    type Input = Almanac;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        input
            .seeds
            .iter()
            .map(|seed| find_location(*seed, &input.maps))
            .min()
            .context("Why no seeds?")
    }
}

impl Solution<'_, { Day::Day5 }, { Part::Two }> for AOC2023<{ Day::Day5 }> {
    type Input = Almanac;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let seeds = input
            .seeds
            .chunks(2)
            .flat_map(|pair| pair[0]..(pair[0] + pair[1]))
            .collect::<Vec<_>>();
        seeds
            .par_iter()
            .map(|seed| find_location(*seed, &input.maps))
            .min()
            .context("Why no seeds?")
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner::PartOneVerifier;

    use super::*;

    #[test]
    fn test_parse_seeds() -> Result<()> {
        let seeds = "seeds: 3489262449 222250568";
        assert_eq!(
            parse_seeds(seeds)?,
            ("", Vec::from([3489262449, 222250568]))
        );
        Ok(())
    }

    #[test]
    fn test_parse_range_mapping() -> Result<()> {
        let input = "0 262295201 34634737";
        assert_eq!(
            parse_range_mapping(input)?,
            (
                "",
                RangeMapping {
                    destination_start: 0,
                    source_start: 262295201,
                    length: 34634737
                }
            )
        );
        Ok(())
    }

    #[test]
    fn test_parse_map() -> Result<()> {
        let input = "seed-to-soil map:
0 1 2
3 4 5";
        assert_eq!(
            parse_map(input)?,
            (
                "",
                CategoryMap {
                    range_mappings: [
                        RangeMapping {
                            destination_start: 0,
                            source_start: 1,
                            length: 2
                        },
                        RangeMapping {
                            destination_start: 3,
                            source_start: 4,
                            length: 5
                        },
                    ]
                    .to_vec()
                }
            )
        );
        Ok(())
    }

    #[test]
    fn test_range_mapping() {
        let mapping = RangeMapping {
            destination_start: 50,
            source_start: 98,
            length: 2,
        };
        assert_eq!(mapping.map(97), None);
        assert_eq!(mapping.map(98), Some(50));
        assert_eq!(mapping.map(99), Some(51));
        assert_eq!(mapping.map(100), None);
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let problem = super::AOC2023::<{ Day::Day5 }>;
        problem.test_part1(input, 35)
    }
}
