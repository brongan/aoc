use super::AOC2021;
use aoc_runner::{Day, ParseInput, Part, Solution};

fn most_common_elements(lines: &[String]) -> Vec<char> {
    (0..lines[0].len())
        .map(|col| {
            let one_count = lines
                .iter()
                .map(|l| l.as_bytes()[col])
                .filter(|c| *c == b'1')
                .count();
            if one_count >= lines.len() - one_count {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}

impl ParseInput<'_, { Day::Day3 }> for AOC2021<{ Day::Day3 }> {
    type Parsed = Vec<String>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input.lines().map(|l| l.to_owned()).collect()
    }
}

impl Solution<'_, { Day::Day3 }, { Part::One }> for AOC2021<{ Day::Day3 }> {
    type Input = Vec<String>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut gamma = 0;
        let mut epsilon = 0;
        for elem in most_common_elements(input) {
            gamma *= 2;
            epsilon *= 2;
            if elem == '1' {
                gamma += 1;
            } else {
                epsilon += 1;
            }
        }
        gamma * epsilon
    }
}

impl Solution<'_, { Day::Day3 }, { Part::Two }> for AOC2021<{ Day::Day3 }> {
    type Input = Vec<String>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut oxygen_gen_lines = input.clone();
        let mut col: usize = 0;
        while oxygen_gen_lines.len() > 1 {
            let commonz = most_common_elements(&oxygen_gen_lines);
            oxygen_gen_lines = oxygen_gen_lines
                .into_iter()
                .filter(|line| line.as_bytes()[col] == commonz[col] as u8)
                .collect();
            col += 1;
        }
        let mut co2_gen_lines = input.clone();
        let mut col = 0;
        while co2_gen_lines.len() > 1 {
            let commonz = most_common_elements(&co2_gen_lines);
            co2_gen_lines = co2_gen_lines
                .into_iter()
                .filter(|line| line.as_bytes()[col] != commonz[col] as u8)
                .collect();
            col += 1;
        }

        let oxygen_gen_rate = u32::from_str_radix(&oxygen_gen_lines[0], 2).unwrap();
        let co2_gen_rate = u32::from_str_radix(&co2_gen_lines[0], 2).unwrap();
        oxygen_gen_rate * co2_gen_rate
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use aoc_runner::PartTwoVerifier;

    use super::*;

    #[test]
    fn test_pt_2() -> Result<(), String> {
        let input: String = "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"
            .to_string()
            .split_whitespace()
            .into_iter()
            .join("\n");
        let problem = super::AOC2021::<{ Day::Day3 }>;
        (&&&problem).test_part2(&input, 230)
    }
}
