use std::cell::RefCell;
use std::fmt::{self, Debug};
use std::rc::Rc;

use super::AOC2022;
use anyhow::{Context, Result};
use aoc_runner::{Day, ParseInput, Part, Solution};
use camino::Utf8PathBuf;
use indexmap::IndexMap;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{newline, space0, space1};
use nom::character::is_alphabetic;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

fn parse_path(s: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while(|c: char| is_alphabetic(c as u8) || c == '.' || c == '/'),
        Into::into,
    )(s)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Command {
    LS,
    CD,
}

fn parse_binary(s: &str) -> IResult<&str, Command> {
    alt((
        map(tag("ls"), |_| Command::LS),
        map(tag("cd"), |_| Command::CD),
    ))(s)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Entry {
    Dir(Utf8PathBuf),
    File(Utf8PathBuf, u64),
}

fn parse_entry(s: &str) -> IResult<&str, Entry> {
    let parse_dir = map(
        preceded(tag("dir"), preceded(space1, parse_path)),
        Entry::Dir,
    );
    let parse_file = map(
        separated_pair(nom::character::complete::u64, space1, parse_path),
        |(size, path)| Entry::File(path, size),
    );
    alt((parse_dir, parse_file))(s)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum TerminalOutput {
    Command(Command, Utf8PathBuf),
    Entry(Entry),
}

fn parse_line(s: &str) -> IResult<&str, TerminalOutput> {
    let parse_command = map(
        preceded(
            tag("$"),
            preceded(space1, separated_pair(parse_binary, space0, parse_path)),
        ),
        |(binary, path)| TerminalOutput::Command(binary, path),
    );
    let parse_file_entry = map(parse_entry, TerminalOutput::Entry);
    alt((parse_command, parse_file_entry))(s)
}

fn parse_lines(s: &str) -> IResult<&str, Vec<TerminalOutput>> {
    separated_list0(newline, parse_line)(s)
}

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Debug, Default)]
pub struct Node {
    size: u64,
    children: IndexMap<Utf8PathBuf, NodeHandle>,
    parent: Option<NodeHandle>,
}

impl ParseInput<'_, { Day::Day7 }> for AOC2022<{ Day::Day7 }> {
    type Parsed = NodeHandle;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, terminal_output) = parse_lines(input).map_err(|e| e.to_owned())?;

        let root = Rc::new(RefCell::new(Node::default()));
        let mut curr = root.clone();

        for line in terminal_output {
            match line {
                TerminalOutput::Command(binary, path) => match binary {
                    Command::LS => {}
                    Command::CD => match path.as_str() {
                        "/" => {
                            curr = root.clone();
                        }
                        ".." => {
                            let node = curr.borrow().parent.clone().unwrap();
                            curr = node;
                        }
                        _ => {
                            let node = curr.borrow_mut().children.entry(path).or_default().clone();
                            curr = node;
                        }
                    },
                },
                TerminalOutput::Entry(entry) => match entry {
                    Entry::Dir(name) => {
                        let child = curr.borrow_mut().children.entry(name).or_default().clone();
                        child.borrow_mut().parent = Some(curr.clone());
                    }
                    Entry::File(name, size) => {
                        let child = curr.borrow_mut().children.entry(name).or_default().clone();
                        child.borrow_mut().size = size;
                        child.borrow_mut().parent = Some(curr.clone());
                    }
                },
            }
        }
        Ok(root)
    }
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn total_size(&self) -> u64 {
        self.size
            + self
                .children
                .values()
                .map(|c| c.borrow().total_size())
                .sum::<u64>()
    }
}

struct PrettyNode<'a>(&'a NodeHandle, Utf8PathBuf, u64);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();

        let depth = "  ".repeat(self.2 as usize);
        if this.size == 0 {
            writeln!(f, "{}- {} (dir)", depth, self.1)?;
        } else {
            writeln!(f, "{}- {} (file, size={})", depth, self.1, this.size)?;
        }

        for (name, child) in &this.children {
            write!(f, "{:?}", PrettyNode(child, name.clone(), self.2 + 1))?;
        }
        Ok(())
    }
}

fn all_dirs(node: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    #[allow(clippy::needless_collect)]
    let children = node.borrow().children.values().cloned().collect::<Vec<_>>();
    Box::new(
        std::iter::once(node).chain(
            children
                .into_iter()
                .filter_map(|child| {
                    if child.borrow().is_dir() {
                        Some(all_dirs(child))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}

impl Solution<'_, { Day::Day7 }, { Part::One }> for AOC2022<{ Day::Day7 }> {
    type Input = NodeHandle;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(all_dirs(input.clone())
            .map(|d| d.borrow().total_size())
            .filter(|&s| s < 100_000)
            .sum())
    }
}

impl Solution<'_, { Day::Day7 }, { Part::Two }> for AOC2022<{ Day::Day7 }> {
    type Input = NodeHandle;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let max_space_possible = 70000000;
        let needed_free_space = 30000000;
        let curr_space_used = input.clone().borrow().total_size();
        let curr_free = max_space_possible - curr_space_used;
        let need_to_delete = needed_free_space - curr_free;
        all_dirs(input.clone())
            .map(|d| d.borrow().total_size())
            .filter(|&s| s >= need_to_delete)
            .min()
            .context("No possible directories to delete")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parsing() {
        assert_eq!(parse_path("/"), Ok(("", Utf8PathBuf::from("/"))));
        assert_eq!(
            parse_path("bob.doodle.doo"),
            Ok(("", Utf8PathBuf::from("bob.doodle.doo")))
        );
        assert_eq!(parse_binary("ls"), Ok(("", Command::LS)));
        assert_eq!(parse_binary("cd"), Ok(("", Command::CD)));
        assert_eq!(
            parse_line("$ cd  /"),
            Ok(("", TerminalOutput::Command(Command::CD, "/".into())))
        );
        assert_eq!(
            parse_line("72 bob"),
            Ok(("", TerminalOutput::Entry(Entry::File("bob".into(), 72))))
        );
        assert_eq!(
            parse_line("$ ls /"),
            Ok(("", TerminalOutput::Command(Command::LS, "/".into())))
        );
        assert_eq!(parse_entry("dir d"), Ok(("", (Entry::Dir("d".into())))));
        assert_eq!(
            parse_entry("69 file"),
            Ok(("", (Entry::File("file".into(), 69))))
        );

        assert_eq!(
            parse_line("69 file"),
            Ok(("", (TerminalOutput::Entry(Entry::File("file".into(), 69)))))
        );

        assert_eq!(
            parse_lines("dir d\n$ cd a"),
            Ok((
                "",
                vec![
                    (TerminalOutput::Entry(Entry::Dir("d".into()))),
                    (TerminalOutput::Command(Command::CD, "a".into()))
                ]
            ))
        );
        assert_eq!(
            parse_lines(
                r#"dir a
14848514 b.txt"#
            ),
            Ok((
                "",
                vec![
                    (TerminalOutput::Entry(Entry::Dir("a".into()))),
                    (TerminalOutput::Entry(Entry::File("b.txt".into(), 14848514)))
                ]
            ))
        );
    }

    #[test]
    fn test() -> Result<()> {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let expected = "- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)
";
        let problem = super::AOC2022::<{ Day::Day7 }>;
        assert_eq!(
            format!(
                "{:?}",
                PrettyNode(&problem.parse_input(input)?, "/".into(), 0)
            ),
            expected
        );
        problem.test_part1(input, 95437)
    }
}
