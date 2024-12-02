use super::AOC$YEAR;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

type IR = todo!();
type Num = todo!();

impl ParseInput<'_, { Day::$DAY }> for AOC2024<{ Day::$DAY }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        todo!()
    }
}

impl Solution<'_, { Day::$DAY }, { Part::One }> for AOC2024<{ Day::$DAY }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        todo!()
    }
}

impl Solution<'_, { Day::$DAY }, { Part::Two }> for AOC2024<{ Day::$DAY }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        todo!()
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
