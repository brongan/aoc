use super::AOC$YEAR;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

impl ParseInput<'_, { Day::$DAY }> for AOC2024<{ Day::$DAY }> {
    type Parsed = $IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
    }
}

impl Solution<'_, { Day::$DAY }, { Part::One }> for AOC2024<{ Day::$DAY }> {
    type Input = $IR;
    type Output = $TODO;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let list1 = input.iter().map(|(l, _r)| l.to_owned()).collect();
        let list2 = input.iter().map(|(_l, r)| r.to_owned()).collect();

        Ok(list_distance(list1, list2))
    }
}

impl Solution<'_, { Day::$DAY }, { Part::Two }> for AOC2024<{ Day::$DAY }> {
    type Input = $IR;
    type Output = $TODO;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}
