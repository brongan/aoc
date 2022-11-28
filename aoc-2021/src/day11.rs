use super::AOC2021;
use anyhow::{Result, Context};
use aoc_runner::{Day, ParseInput, Part, Solution};
use std::collections::{BTreeSet, HashSet};

impl ParseInput<'_, { Day::Day11 }> for AOC2021<{ Day::Day11 }> {
    type Parsed = Vec<Vec<u32>>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .split('\n')
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).context("failed to parse digit"))
                    .collect()
            })
        .collect()
    }
}

fn get_neighbors(i: i32, j: i32, num_rows: i32, num_col: i32) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::with_capacity(4);
    for (dx, dy) in [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ] {
        let new_x = i + dx;
        let new_y = j + dy;
        if new_x >= 0 && new_x < num_rows && new_y >= 0 && new_y < num_col {
            neighbors.push((new_x as usize, new_y as usize))
        }
    }
    neighbors
}

impl Solution<'_, { Day::Day11 }, { Part::One }> for AOC2021<{ Day::Day11 }> {
    type Input = Vec<Vec<u32>>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut input = input.clone();
        let mut ret = 0;
        for _ in 0..100 {
            ret += step(&mut input);
        }
        Ok(ret)
    }
}

fn step(energy_levels: &mut Vec<Vec<u32>>) -> usize {
    let mut to_flash: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut on_cooldown: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in energy_levels.iter_mut().enumerate() {
        for (j, elem) in row.iter_mut().enumerate() {
            *elem += 1;
            if *elem > 9 {
                to_flash.insert((i, j));
            }
        }
    }
    while let Some(flash_index) = to_flash.pop_first() {
        to_flash.remove(&flash_index);
        on_cooldown.insert(flash_index);

        for neighbor in get_neighbors(
            flash_index.0 as i32,
            flash_index.1 as i32,
            energy_levels.len() as i32,
            energy_levels[0].len() as i32,
        ) {
            let elem = &mut energy_levels[neighbor.0][neighbor.1];
            *elem += 1;
            if *elem > 9 && !on_cooldown.contains(&neighbor) {
                to_flash.insert(neighbor);
            }
        }
    }
    for octopus in &on_cooldown {
        energy_levels[octopus.0][octopus.1] = 0;
    }
    on_cooldown.len()
}

impl Solution<'_, { Day::Day11 }, { Part::Two }> for AOC2021<{ Day::Day11 }> {
    type Input = Vec<Vec<u32>>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut input = input.clone();
        let octopus_count = input.iter().flatten().count();
        let mut ret = 1;
        while step(&mut input) != octopus_count {
            ret += 1;
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    #[test]
    fn test() -> Result<()> {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let problem = super::AOC2021::<{ Day::Day11 }>;
        problem.test_part1(input, 1656)?;
        problem.test_part2(input, 195)
    }
}
