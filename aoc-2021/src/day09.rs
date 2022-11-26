use std::collections::{HashSet, VecDeque};

use super::AOC2021;
use aoc_runner::{Day, ParseInput, Part, Solution};
use aoc_runner::point2d::Point2D;

impl ParseInput<'_, { Day::Day9 }> for AOC2021<{ Day::Day9 }> {
    type Parsed = Vec<Vec<u32>>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|digit| digit.to_digit(10).expect("Expected Digit"))
                    .collect()
            })
            .collect()
    }
}

fn get_low_points(height_map: &[Vec<u32>]) -> Vec<Point2D<usize>> {
    let mut ret = Vec::new();
    let num_rows = height_map.len() as i32;
    let num_col = height_map[0].len() as i32;
    for i in 0..num_rows {
        for j in 0..num_col {
            let neighbors = get_neighbors(i, j, num_rows, num_col);
            let x = i as usize;
            let y = j as usize;
            if neighbors
                .iter()
                .all(|other| height_map[other.x][other.y] > height_map[x][y])
            {
                ret.push(Point2D { x, y });
            }
        }
    }
    ret
}

fn get_neighbors(i: i32, j: i32, num_rows: i32, num_col: i32) -> Vec<Point2D<usize>> {
    let mut neighbors = Vec::with_capacity(4);
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let new_x = i + dx;
        let new_y = j + dy;
        if new_x >= 0 && new_x < num_rows && new_y >= 0 && new_y < num_col {
            neighbors.push(Point2D {
                x: new_x as usize,
                y: new_y as usize,
            })
        }
    }
    neighbors
}

fn calculate_basin_size(height_map: &[Vec<u32>], low_point: Point2D<usize>) -> u32 {
    let mut frontier: VecDeque<Point2D<usize>> = VecDeque::from([low_point]);
    let mut visited: HashSet<Point2D<usize>> = HashSet::from([low_point]);
    let mut ret = 0;
    while !frontier.is_empty() {
        let node = frontier.pop_front().unwrap();
        ret += 1;
        for neighbor in get_neighbors(
            node.x as i32,
            node.y as i32,
            height_map.len() as i32,
            height_map[0].len() as i32,
        ) {
            if height_map[neighbor.x][neighbor.y] > height_map[node.x][node.y]
                && height_map[neighbor.x][neighbor.y] != 9
                && !visited.contains(&neighbor)
            {
                visited.insert(neighbor);
                frontier.push_back(neighbor);
            }
        }
    }
    ret
}

impl Solution<'_, { Day::Day9 }, { Part::One }> for AOC2021<{ Day::Day9 }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        get_low_points(input)
            .iter()
            .map(|point| input[point.x][point.y] + 1)
            .sum()
    }
}

impl Solution<'_, { Day::Day9 }, { Part::Two }> for AOC2021<{ Day::Day9 }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut basin_sizes: Vec<u32> = get_low_points(input)
            .iter()
            .filter(|point| input[point.x][point.y] != 9)
            .map(|low_point| calculate_basin_size(input, *low_point))
            .collect();
        basin_sizes.sort_unstable();
        basin_sizes.iter().rev().take(3).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    #[test]
    fn test() -> Result<(), String> {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let problem = super::AOC2021::<{ Day::Day9 }>;
        (&&&problem).test_part1(input, 15)?;
        (&&&problem).test_part2(input, 1134)
    }
}
