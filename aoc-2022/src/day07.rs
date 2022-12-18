use std::str::FromStr;

use super::AOC2022;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::IResult;

fn parse_path(s: &str) -> IResult<&str, Utf8Path> {
    map(
        take_while(|c: char| c.is_alphanumeric() || c == '.' || c == '-')(s)?,
        Utf8Path::from_str,
    )
}

#[derive(Debug)]
enum Binary {
    LS,
    CD,
}

fn parse_binary(s: &str) -> IResult<&str, Binary> {
    alt((
        map(tag("ls"), |_| Binary::LS),
        map(tag("cd"), |_| Binary::CD),
    ))(s)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8Path),
    File(Utf8Path, usize),
}

fn parse_entry(s: &str) -> IResult<&str, Entry> {
    alt((parse_dir, parse_file))(s)
}

#[derive(Debug)]
enum TerminalOutput {
    Command(Binary, Utf8Path),
    FileEntry(Entry),
}

fn parse_terminal_line(s: &str) -> IResult<&str, TerminalOutput> {
    alt((command, file_entry))(s)
}

fn parse_terminal_output(s: &str) -> IResult<&str, Vec<TerminalOutput>> {
    separated_list0(tag("\n"), terminal_line)(s)
}

impl ParseInput<'_, { Day::Day7 }> for AOC2022<{ Day::Day7 }> {
    type Parsed = Vec<TerminalOutput>;

    fn parse_inpu(&self, input: &'_ str) -> Result<Self::Parsed> {
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
