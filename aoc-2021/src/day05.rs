use std::cmp::{max, Ordering};
use std::vec;

use super::AOC2021;
use aoc_runner::{Day, ParseInput, Part, Solution};
use aoc_runner::point2d::Point2D;

type VentLine = (Point2D<i32>, Point2D<i32>);
type Seafloor = Vec<Vec<i32>>;

fn to_vent_line(line: &str) -> Option<VentLine> {
    let (p1, p2) = line.split_once(" -> ")?;
    let p1 = p1.split_once(',')?;
    let p2 = p2.split_once(',')?;
    let y1 = p1.0.parse().ok()?;
    let y2 = p2.0.parse().ok()?;
    let x1 = p1.1.parse().ok()?;
    let x2 = p2.1.parse().ok()?;
    Some((Point2D { x: x1, y: y1 }, Point2D { x: x2, y: y2 }))
}
fn seafloor_from_lines(
    lines: &[VentLine],
    filter: fn(VentLine) -> bool,
    max_index: usize,
) -> Seafloor {
    let mut seafloor = vec![vec![0i32; max_index]; max_index];

    for line in lines.iter().filter(|line| filter(**line)) {
        let delta_x: i32 = match (line.1.x).partial_cmp(&line.0.x) {
            Some(Ordering::Greater) => 1,
            Some(Ordering::Equal) => 0,
            Some(Ordering::Less) => -1,
            None => panic!("How this this Possible"),
        };
        let delta_y: i32 = match (line.1.y).partial_cmp(&line.0.y) {
            Some(Ordering::Greater) => 1,
            Some(Ordering::Equal) => 0,
            Some(Ordering::Less) => -1,
            None => panic!("How this this Possible"),
        };
        for i in 0..(max(
            i32::abs_diff(line.0.x, line.1.x),
            i32::abs_diff(line.0.y, line.1.y),
        ) + 1) as i32
        {
            let x = line.0.x + (delta_x * i);
            let y = line.0.y + (delta_y * i);
            seafloor[x as usize][y as usize] += 1;
        }
    }
    seafloor
}

impl ParseInput<'_, { Day::Day5 }> for AOC2021<{ Day::Day5 }> {
    type Parsed = Vec<VentLine>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input.lines().flat_map(to_vent_line).collect()
    }
}

fn part1_filter(line: VentLine) -> bool {
    line.0.x == line.1.x || line.0.y == line.1.y
}

impl Solution<'_, { Day::Day5 }, { Part::One }> for AOC2021<{ Day::Day5 }> {
    type Input = Vec<VentLine>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let seafloor = seafloor_from_lines(input, part1_filter, 1000);
        seafloor.iter().flatten().filter(|x| **x >= 2).count()
    }
}

impl Solution<'_, { Day::Day5 }, { Part::Two }> for AOC2021<{ Day::Day5 }> {
    type Input = Vec<VentLine>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        fn part2_filter(line: VentLine) -> bool {
            part1_filter(line)
                || (i32::abs_diff(line.0.x, line.1.x) == i32::abs_diff(line.0.y, line.1.y))
        }
        let seafloor = seafloor_from_lines(input, part2_filter, 1000);
        seafloor.iter().flatten().filter(|x| **x >= 2).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    fn input() -> &'static str {
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
    }

    #[test]
    fn test() -> Result<(), String> {
        let problem = super::AOC2021::<{ Day::Day5 }>;
        (&&&problem).test_part1(&input(), 5)?;
        (&&&problem).test_part2(&input(), 12)
    }
}
