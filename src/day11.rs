use super::AOC2021;
use crate::aoc::{Day, ParseInput, Part, Solution};
use std::collections::HashSet;

impl ParseInput<'_, { Day::Day11 }> for AOC2021<{ Day::Day11 }> {
    type Parsed = Vec<Vec<u32>>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input
            .split('\n')
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).expect("failed to parse digit"))
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

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut input = input.clone();
        let mut ret = 0;
        for _ in 0..100 {
            ret += step(&mut input);
        }
        ret
    }
}

fn step(energy_levels: &mut Vec<Vec<u32>>) -> usize {
    let mut to_flash: HashSet<(usize, usize)> = HashSet::new();
    let mut on_cooldown: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in energy_levels.iter_mut().enumerate() {
        for (j, elem) in row.iter_mut().enumerate() {
            *elem += 1;
            if *elem > 9 {
                to_flash.insert((i, j));
            }
        }
    }
    while !to_flash.is_empty() {
        let flash_index = *to_flash.iter().next().unwrap();
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

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut input = input.clone();
        let octopus_count = input.iter().flatten().count();
        let mut ret = 1;
        while step(&mut input) != octopus_count {
            ret += 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::PartOneVerifier;
    use crate::aoc::PartTwoVerifier;

    #[test]
    fn test() -> Result<(), String> {
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
        (&&&problem).test_part1(input, 1656)?;
        (&&&problem).test_part2(input, 195)
    }
}
