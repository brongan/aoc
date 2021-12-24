#![feature(int_abs_diff)]
use std::{fs::read_to_string, ops::Range};

fn median(mut list: Vec<usize>) -> usize {
    list.sort_unstable();
    list[list.len() / 2]
}

fn part1_fuel_cost(list: &[usize], value: usize) -> usize {
    list.iter().map(|elem| usize::abs_diff(*elem, value)).sum()
}

fn part2_fuel_cost(list: &[usize], value: usize) -> usize {
    list.iter()
        .map(|elem| {
            let n = usize::abs_diff(*elem, value);
            n * (n + 1) / 2
        })
        .sum()
}

fn part1(input: &str) -> usize {
    let crabs = parse_input(input);
    let median = median(crabs.clone());
    part1_fuel_cost(&crabs, median)
}

fn parse_input(input: &str) -> Vec<usize> {
    let crabs: Vec<usize> = input
        .trim()
        .split(',')
        .map(|num| num.parse::<usize>().expect("Failed to parse number"))
        .collect();
    crabs
}

fn part2(input: &str) -> usize {
    let crabs = parse_input(input);
    let possible_range: Range<usize> =
        (*crabs.iter().min().expect("has min"))..(*crabs.iter().max().expect("has max"));
    possible_range
        .into_iter()
        .map(|crab| part2_fuel_cost(&crabs, crab))
        .min()
        .expect("possible_range has a min")
}

fn main() {
    let input = read_to_string("input").expect("Failed to read input.");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pt_1() {
        assert_eq!(part1("16,1,2,0,4,2,7,1,2,14"), 37);
    }

    #[test]
    fn test_pt_2() {
        assert_eq!(part2("16,1,2,0,4,2,7,1,2,14"), 168);
    }
}
