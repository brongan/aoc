use std::fmt::Debug;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Clone, Copy, Default)]
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

#[derive(Clone, Default)]
struct BingoGame {
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
        .into_iter()
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
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines.next().unwrap();
        let input: Vec<u32> = first_line
            .split(',')
            .into_iter()
            .filter_map(|s| s.parse().ok())
            .collect();
        let lines: Vec<&str> = lines.filter(|l| !l.is_empty()).collect();
        Ok(BingoGame {
            input,
            boards: lines
                .chunks(BOARD_SIZE)
                .map(|chonk| parse_bingo_board(chonk.to_vec()))
                .collect::<Option<Vec<BingoBoard>>>()
                .expect("Failed to parse boards"),
        })
    }
}

// the chosen number is marked on all boards on which it appears.
// (Numbers may not appear on all boards.)
fn part1(mut boards: Vec<BingoBoard>, input: &[u32]) -> u32 {
    for num in input {
        for board in &mut boards {
            if pull(board, *num) && is_complete(board) {
                return score(board, *num);
            }
        }
    }
    0
}

// Figure out which board will win last. Once it wins, what would its final score be?
fn part2(mut boards: Vec<BingoBoard>, input: &[u32]) -> Option<u32> {
    let mut num_iter = input.iter();
    while boards.len() > 1 {
        let num = num_iter.next()?;
        for board in &mut boards {
            pull(board, *num);
        }
        boards.retain(|board| !is_complete(board));
    }

    let mut board = boards[0];
    let mut num: u32 = 0;
    while !is_complete(&board) {
        num = *num_iter.next()?;
        pull(&mut board, num);
    }
    Some(score(&board, num))
}

fn read_input(path: &str) -> String {
    read_to_string(path).expect("Failed to read input file")
}

fn main() {
    let game = BingoGame::from_str(&read_input("input")).expect("Failed to parse input file");
    println!("Part 1: {}", part1(game.boards.clone(), &game.input));
    println!(
        "Part 2: {}",
        part2(game.boards, &game.input).expect("Went through input without finishing bingo board.")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let game = BingoGame::from_str(&read_input("test")).expect("Failed to parse input file");
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
    }

    #[test]
    fn test_pt_1() {
        let game = BingoGame::from_str(&read_input("test")).expect("Failed to parse input file");
        assert_eq!(part1(game.boards, &game.input), 4512);
    }

    #[test]
    fn test_pt_2() {
        let game = BingoGame::from_str(&read_input("test")).expect("Failed to parse input file");
        assert_eq!(part2(game.boards, &game.input).unwrap(), 1924);
    }
}
