use std::collections::{HashSet, VecDeque};

use super::AdventOfCode2021;
use crate::aoc::ParseInput;
use crate::aoc::{Day, Part, Solution};

impl ParseInput<'_, { Day::Nine }> for AdventOfCode2021<{ Day::Nine }> {
    type Parsed = Vec<Vec<u32>>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|digit| digit.to_digit(10).expect("Expected Digit"))
                    .collect()
            })
            .collect()
    }
}

fn get_low_points(height_map: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    let num_rows = height_map.len() as i32;
    let num_col = height_map[0].len() as i32;
    for i in 0..num_rows {
        for j in 0..num_col {
            let neighbors = get_neighbors(i, j, num_rows, num_col);
            if neighbors
                .iter()
                .all(|(x, y)| height_map[*x][*y] > height_map[i as usize][j as usize])
            {
                ret.push((i as usize, j as usize));
            }
        }
    }
    ret
}

fn get_neighbors(i: i32, j: i32, num_rows: i32, num_col: i32) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::with_capacity(4);
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let new_x = i + dx;
        let new_y = j + dy;
        if new_x >= 0 && new_x < num_rows && new_y >= 0 && new_y < num_col {
            neighbors.push((new_x as usize, new_y as usize))
        }
    }
    neighbors
}

fn calculate_basin_size(height_map: &[Vec<u32>], low_point: (usize, usize)) -> u32 {
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::from([low_point]);
    let mut visited: HashSet<(usize, usize)> = HashSet::from([low_point]);
    let mut ret = 0;
    while !frontier.is_empty() {
        let node = frontier.pop_front().unwrap();
        ret += 1;
        for neighbor in get_neighbors(
            node.0 as i32,
            node.1 as i32,
            height_map.len() as i32,
            height_map[0].len() as i32,
        ) {
            if height_map[neighbor.0][neighbor.1] > height_map[node.0][node.1]
                && height_map[neighbor.0][neighbor.1] != 9
                && !visited.contains(&neighbor)
            {
                visited.insert(neighbor);
                frontier.push_back(neighbor);
            }
        }
    }
    ret
}

impl Solution<'_, { Day::Nine }, { Part::One }> for AdventOfCode2021<{ Day::Nine }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        get_low_points(input)
            .iter()
            .map(|(x, y)| input[*x][*y] + 1)
            .sum()
    }
}

impl Solution<'_, { Day::Nine }, { Part::Two }> for AdventOfCode2021<{ Day::Nine }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut basin_sizes: Vec<u32> = get_low_points(input)
            .iter()
            .filter(|(x, y)| input[*x][*y] != 9)
            .map(|low_point| calculate_basin_size(input, *low_point))
            .collect();
        basin_sizes.sort_unstable();
        basin_sizes.iter().rev().take(3).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::PartOneVerifier;
    use crate::aoc::PartTwoVerifier;

    #[test]
    fn test() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let problem = super::AdventOfCode2021::<{ Day::Nine }>;
        (&&&problem).test_part1(input, 15);
        (&&&problem).test_part2(input, 1134);
    }
}
