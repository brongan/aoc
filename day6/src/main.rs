use std::fs::read_to_string;

fn read_input() -> String {
    read_to_string("input").expect("Failed to read input.")
}
fn parse_input(input: &str) -> [u64; 9] {
    let mut counts = [0u64; 9];
    for num in input
        .trim()
        .split(',')
        .map(|num| num.parse::<usize>().expect("Failed to parse number."))
    {
        counts[num] += 1;
    }
    counts
}

fn part1(mut counts: [u64; 9], num_iterations: u32) -> u64 {
    println!("Initial: {:?}", counts);
    for _ in 0..num_iterations {
        let breeders = counts[0];
        for age_bucket in 0..counts.len() - 1 {
            counts[age_bucket] = counts[age_bucket + 1];
        }
        counts[6] += breeders;
        counts[8] = breeders;
    }
    counts.iter().sum()
}

fn main() {
    let input = read_input();
    println!("Part 1: {}", part1(parse_input(&input), 80));
    println!("Part 1: {}", part1(parse_input(&input), 256));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pt_1() {
        assert_eq!(part1(parse_input("3,4,3,1,2"), 18), 26);
        assert_eq!(part1(parse_input("3,4,3,1,2"), 80), 5934);
    }

    #[test]
    fn test_pt_2() {
        assert_eq!(part1(parse_input("3,4,3,1,2"), 256), 26984457539);
    }
}
