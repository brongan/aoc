use super::AOC2024;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type IR = Vec<Vec<char>>;
type Num = usize;

impl ParseInput<'_, { Day::Day4 }> for AOC2024<{ Day::Day4 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.lines().map(|line| line.chars().collect()).collect())
    }
}

#[derive(EnumIter, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

fn move_inbounds(input: &IR, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
    let width = input.len();
    let height = input[0].len();

    let outside = |x, y| x < 0 || x >= width as i32 || y < 0 || y >= height as i32;
    let move_dir = |x, y| match direction {
        Direction::Up => (x, y + 1),
        Direction::Down => (x, y - 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
        Direction::UpRight => (x + 1, y + 1),
        Direction::UpLeft => (x - 1, y + 1),
        Direction::DownRight => (x + 1, y - 1),
        Direction::DownLeft => (x - 1, y - 1),
    };

    let (x, y) = move_dir(x as i32, y as i32);
    if outside(x, y) {
        return None;
    }
    return Some((x as usize, y as usize));
}

fn check_xmas(input: &IR, mut x: usize, mut y: usize, direction: Direction) -> bool {
    if input[x][y] != 'X' {
        return false;
    }

    for char in ['M', 'A', 'S'] {
        (x, y) = match move_inbounds(input, x, y, direction) {
            Some((x, y)) => (x, y),
            None => return false,
        };
        if input[x][y] != char {
            return false;
        }
    }

    true
}

// M.S
// .A.
// M.S
// Return true if both diagonals spell MAS
fn check_mas_x(input: &IR, x: usize, y: usize) -> bool {
    if input[x][y] != 'A' {
        return false;
    }
    let up_left = match move_inbounds(input, x, y, Direction::UpLeft) {
        Some((x, y)) => input[x][y],
        None => return false,
    };

    let up_right = match move_inbounds(input, x, y, Direction::UpRight) {
        Some((x, y)) => input[x][y],
        None => return false,
    };

    let down_left = match move_inbounds(input, x, y, Direction::DownLeft) {
        Some((x, y)) => input[x][y],
        None => return false,
    };

    let down_right = match move_inbounds(input, x, y, Direction::DownRight) {
        Some((x, y)) => input[x][y],
        None => return false,
    };

    let left_diag = (up_left == 'M' && down_right == 'S') || (up_left == 'S' && down_right == 'M');
    let right_diag = (up_right == 'M' && down_left == 'S') || (up_right == 'S' && down_left == 'M');

    left_diag && right_diag
}

impl Solution<'_, { Day::Day4 }, { Part::One }> for AOC2024<{ Day::Day4 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut ret = 0;
        for x in 0..input.len() {
            for y in 0..input[0].len() {
                for direction in Direction::iter() {
                    if check_xmas(input, x, y, direction) {
                        ret += 1;
                    }
                }
            }
        }
        Ok(ret)
    }
}

impl Solution<'_, { Day::Day4 }, { Part::Two }> for AOC2024<{ Day::Day4 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut ret = 0;
        for x in 0..input.len() {
            for y in 0..input[0].len() {
                if check_mas_x(input, x, y) {
                    ret += 1;
                }
            }
        }
        Ok(ret)
    }
}
