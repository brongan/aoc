use std::str::FromStr;

use super::AOC2022;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Context;
use anyhow::Result;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;

enum Command {
    LS(String),
    CD(String),
}

impl FromStr for Command {
    type Err = nom::Err<(String, nom::error::ErrorKind)>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

enum Entry {
    Dir(String),
    File(String, usize),
}

enum TerminalOutput {
    Command(Command),
    FileEntry(Entry),
}

fn command(s: &str) -> IResult<&str, Command> {
    tuple((tag("$ "), command_name, tag(" "), command_arg))(s)
        .map(|(i, (_, name, _, arg))| (i, Command::new(name, arg)))
}

fn terminal_line(s: &str) -> IResult<&str, TerminalOutput> {
    alt((command, file_entry))(s)
}

fn terminal_output(s: &str) -> IResult<&str, Vec<TerminalOutput>> {
    separated_list0(tag("\n"), terminal_line)(s)
}

impl ParseInput<'_, { Day::Day7 }> for AOC2022<{ Day::Day7 }> {
    type Parsed = Vec<TerminalOutput>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, terminal_output) = terminal_output(input)?;
        Ok(terminal_output)
    }
}
impl Solution<'_, { Day::Day7 }, { Part::One }> for AOC2022<{ Day::Day7 }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        todo!()
    }
}

/*
impl Solution<'_, { Day::Day7 }, { Part::Two }> for AOC2022<{ Day::Day7 }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {}
}
*/
