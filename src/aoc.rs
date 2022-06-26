use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, EnumIter)]
pub enum Day {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

#[derive(PartialEq, Eq)]
pub enum Part {
    One,
    Two,
}

pub trait Solution<'a, const DAY: Day, const PART: Part> {
    type Input;
    type Output: Display + Eq + std::fmt::Debug + PartialEq;

    fn solve(&'a self, input: &Self::Input) -> Self::Output;
}

pub trait ParseInput<'a, const DAY: Day> {
    type Parsed;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed;
}

pub trait SolutionRunner<'a, const DAY: Day> {
    fn run(&'a self, input: &'a str);
}

pub trait PartOneVerifier<'a, const DAY: Day, T> {
    fn test_part1(&'a self, input: &'a str, expected: T);
}

pub trait PartTwoVerifier<'a, const DAY: Day, T> {
    fn test_part2(&'a self, input: &'a str, expected: T);
}

impl<'a, T: 'a, const DAY: Day> SolutionRunner<'a, DAY> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::One }, Input = <Self as ParseInput<'a, DAY>>::Parsed>
        + Solution<'a, DAY, { Part::Two }, Input = <Self as ParseInput<'a, DAY>>::Parsed>,
{
    fn run(&'a self, input: &'a str) {
        let parsed_input = <Self as ParseInput<DAY>>::parse_input(&self, input);
        let part1_output = <Self as Solution<'a, DAY, { Part::One }>>::solve(&self, &parsed_input);
        let part2_output = <Self as Solution<'a, DAY, { Part::Two }>>::solve(&self, &parsed_input);
        println!("Part One: {}", part1_output);
        println!("Part Two: {}", part2_output);
    }
}

impl<'a, T, const DAY: Day> SolutionRunner<'a, DAY> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::One }, Input = <Self as ParseInput<'a, DAY>>::Parsed>,
{
    default fn run(&'a self, input: &'a str) {
        let parsed_input = <Self as ParseInput<DAY>>::parse_input(&self, input);
        let part1_output = <Self as Solution<'a, DAY, { Part::One }>>::solve(&self, &parsed_input);

        println!("Part One: {}", part1_output);
    }
}

impl<'a, T, const DAY: Day, U> PartOneVerifier<'a, DAY, U> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::One }, Input = <Self as ParseInput<'a, DAY>>::Parsed, Output = U>,
    U: is_type::Is<Type = T::Output> + std::fmt::Debug + std::cmp::PartialEq,
{
    fn test_part1(&'a self, input: &'a str, expected: U) {
        let parsed_input = <Self as ParseInput<DAY>>::parse_input(&self, input);
        let part1_output = <Self as Solution<'a, DAY, { Part::One }>>::solve(&self, &parsed_input);
        assert_eq!(part1_output, expected);
    }
}

impl<'a, T, const DAY: Day, U> PartTwoVerifier<'a, DAY, U> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::Two }, Input = <Self as ParseInput<'a, DAY>>::Parsed, Output = U>,
    U: is_type::Is<Type = T::Output> + std::fmt::Debug + std::cmp::PartialEq,
{
    fn test_part2(&'a self, input: &'a str, expected: U) {
        let input = <Self as ParseInput<DAY>>::parse_input(&self, input);
        let output = <Self as Solution<'a, DAY, { Part::Two }>>::solve(&self, &input);
        assert_eq!(output, expected);
    }
}

#[allow(unused_macros)]
macro_rules! run {
    ($day: expr, $input: expr) => {{
        let problem = $day;
        (&&&problem).run($input)
    }};
}
