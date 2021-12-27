use std::{fs::read_to_string, str::FromStr};

#[derive(PartialEq)]
enum SyntaxScore {
    Corrupt(char),
    Incomplete(Vec<char>),
}

impl FromStr for SyntaxScore {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => {
                    if stack.pop() != Some(c) {
                        return Ok(SyntaxScore::Corrupt(c));
                    }
                }
            }
        }
        stack.reverse();
        Ok(SyntaxScore::Incomplete(stack))
    }
}

impl SyntaxScore {
    fn score(self) -> usize {
        match self {
            Self::Corrupt(c) => score_corruption_char(c),
            Self::Incomplete(string) => score_line_completion(string),
        }
    }

    fn is_corrupt(&self) -> bool {
        matches!(self, Self::Corrupt(_))
    }

    fn is_incomplete(&self) -> bool {
        matches!(self, Self::Incomplete(_))
    }
}

fn score_line_completion(string: Vec<char>) -> usize {
    string
        .iter()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("what ze fuk 2"),
        })
        .fold(0, |total, val| (5 * total) + val)
}

fn score_corruption_char(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("what ze fuck"),
    }
}

fn part1(lines: &str) -> usize {
    lines
        .split('\n')
        .map(SyntaxScore::from_str)
        .map(|x| x.expect("Failed to parse line"))
        .filter(SyntaxScore::is_corrupt)
        .map(SyntaxScore::score)
        .sum()
}

fn part2(lines: &str) -> usize {
    let mut completion_scores: Vec<usize> = lines
        .split('\n')
        .map(SyntaxScore::from_str)
        .map(|x| x.expect("Failed to parse line"))
        .filter(SyntaxScore::is_incomplete)
        .map(SyntaxScore::score)
        .collect();
    completion_scores.sort_unstable();
    completion_scores[completion_scores.len() / 2]
}

fn main() {
    let input = read_to_string("input").expect("failed to read");
    let input = input.trim();
    println!("Part 1: {}", part1(input));
    println!("Part 1: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(part1(input), 26397);
        assert_eq!(part2(input), 288957);
    }
}
