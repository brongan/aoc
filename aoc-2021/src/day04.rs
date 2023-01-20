use anyhow::Context;
use anyhow::Result;
use std::fmt::Debug;
use std::str::FromStr;

use super::AOC2021;
use aoc_runner::{Day, ParseInput, Part, Solution};

#[derive(Default, Copy, Clone)]
struct BingoElement {
    value: u32,
    is_set: bool,
}

impl Debug for BingoElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

const BOARD_SIZE: usize = 5;
type BingoBoard = [[BingoElement; BOARD_SIZE]; BOARD_SIZE];

fn parse_bingo_board(lines: Vec<&str>) -> Option<BingoBoard> {
    let mut ret: BingoBoard = [[BingoElement::default(); BOARD_SIZE]; BOARD_SIZE];
    for (i, line) in lines.iter().enumerate() {
        for (j, elem) in line.split_whitespace().enumerate() {
            ret[i][j].value = elem.parse().ok()?;
        }
    }
    Some(ret)
}

#[derive(Default, Clone)]
pub struct BingoGame {
    input: Vec<u32>,
    boards: Vec<BingoBoard>,
}

fn pull(board: &mut BingoBoard, num: u32) -> bool {
    for mut elem in board.iter_mut().flatten() {
        if elem.value == num {
            elem.is_set = true;
            return true;
        }
    }
    false
}

// If all numbers in any row or any column of a board are marked,
// that board wins. (Diagonals don't count.)
fn is_complete(board: &BingoBoard) -> bool {
    if board
        .map(|row| row.iter().all(|elem| elem.is_set))
        .iter()
        .any(|b| *b)
    {
        return true;
    }
    (0..board[0].len())
        .map(|col| {
            board
                .iter()
                .map(|row| row.get(col).unwrap())
                .all(|elem| elem.is_set)
        })
        .any(|b| b)
}

// Start by finding the sum of all unmarked numbers on that board.
// Then, multiply that sum by the number that was just called when the board won to get the final score.
fn score(board: &BingoBoard, num: u32) -> u32 {
    board
        .iter()
        .flatten()
        .filter(|elem| !elem.is_set)
        .map(|elem| elem.value)
        .sum::<u32>()
        * num
}

impl FromStr for BingoGame {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let first_line = lines.next().unwrap();
        let input: Vec<u32> = first_line
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        let lines: Vec<&str> = lines.filter(|l| !l.is_empty()).collect();
        Ok(BingoGame {
            input,
            boards: lines
                .chunks(BOARD_SIZE)
                .map(|chonk| parse_bingo_board(chonk.to_vec()))
                .collect::<Option<Vec<BingoBoard>>>()
                .context("Failed to parse boards")?,
        })
    }
}

impl ParseInput<'_, { Day::Day4 }> for AOC2021<{ Day::Day4 }> {
    type Parsed = BingoGame;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        BingoGame::from_str(input)
    }
}

impl Solution<'_, { Day::Day4 }, { Part::One }> for AOC2021<{ Day::Day4 }> {
    type Input = BingoGame;
    type Output = u32;

    // the chosen number is marked on all boards on which it appears.
    // (Numbers may not appear on all boards.)
    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut boards = input.boards.clone();
        for num in &input.input {
            for board in &mut boards {
                if pull(board, *num) && is_complete(board) {
                    return Ok(score(board, *num));
                }
            }
        }
        Ok(0)
    }
}

impl Solution<'_, { Day::Day4 }, { Part::Two }> for AOC2021<{ Day::Day4 }> {
    type Input = BingoGame;
    type Output = u32;

    // Figure out which board will win last. Once it wins, what would its final score be?
    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut num_iter = input.input.iter();
        let mut boards = input.boards.clone();
        while boards.len() > 1 {
            let num = num_iter.next().context("inputs don't match")?;
            for board in &mut boards {
                pull(board, *num);
            }
            boards.retain(|board| !is_complete(board));
        }

        let mut board = boards[0];
        let mut num: u32 = 0;
        while !is_complete(&board) {
            num = *num_iter.next().context("inputs don't match")?;
            pull(&mut board, num);
        }
        Ok(score(&board, num))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    fn input() -> String {
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
            .to_owned()
    }

    #[test]
    fn test_parsing() -> Result<()> {
        let game = BingoGame::from_str(&input())?;
        let expected_input = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        assert_eq!(game.input, expected_input);
        let boards: Vec<Vec<u32>> = game
            .boards
            .iter()
            .map(|board| board.iter().flatten().map(|elem| elem.value).collect())
            .collect();

        assert_eq!(boards.len(), 3);
        let expected_first_board = vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ];
        let expected_second_board = vec![
            3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12,
            6,
        ];
        let expected_third_board = vec![
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ];

        assert_eq!(boards[0], expected_first_board);
        assert_eq!(boards[1], expected_second_board);
        assert_eq!(boards[2], expected_third_board);
        Ok(())
    }

    #[test]
    fn test() -> Result<()> {
        let problem = super::AOC2021::<{ Day::Day4 }>;
        problem.test_part1(&input(), 4512)?;
        problem.test_part2(&input(), 1924)
    }
}
