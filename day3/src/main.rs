use std::fs::File;
use std::io::{BufRead, BufReader};

fn most_common_elements(lines: &[String]) -> Vec<char> {
    (0..lines[0].len())
        .map(|col| {
            let one_count = lines
                .iter()
                .map(|l| l.as_bytes()[col])
                .filter(|c| *c == b'1')
                .count();
            if one_count >= lines.len() - one_count {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}

fn part1(lines: &[String]) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    for elem in most_common_elements(lines) {
        gamma *= 2;
        epsilon *= 2;
        if elem == '1' {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    gamma * epsilon
}

fn part2(lines: &[String]) -> u32 {
    let mut oxygen_gen_lines: Vec<String> = lines.to_vec();
    let mut col: usize = 0;
    while oxygen_gen_lines.len() > 1 {
        let commonz = most_common_elements(&oxygen_gen_lines);
        oxygen_gen_lines = oxygen_gen_lines
            .into_iter()
            .filter(|line| line.as_bytes()[col] == commonz[col] as u8)
            .collect();
        col += 1;
    }
    let mut co2_gen_lines = lines.to_vec();
    let mut col = 0;
    while co2_gen_lines.len() > 1 {
        let commonz = most_common_elements(&co2_gen_lines);
        co2_gen_lines = co2_gen_lines
            .into_iter()
            .filter(|line| line.as_bytes()[col] != commonz[col] as u8)
            .collect();
        col += 1;
    }

    let oxygen_gen_rate = u32::from_str_radix(&oxygen_gen_lines[0], 2).unwrap();
    let co2_gen_rate = u32::from_str_radix(&co2_gen_lines[0], 2).unwrap();
    oxygen_gen_rate * co2_gen_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt_2() {
        let input: Vec<String> = "00100
								  11110
							  	  10110
								  10111
								  10101
								  01111
								  00111
								  11100
								  10000
								  11001
								  00010
								  01010"
            .to_string()
            .split_whitespace()
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        assert_eq!(part2(&input), 230);
    }
}

fn main() {
    let f = File::open("input").expect("Failed to open input");
    let f = BufReader::new(f);
    let lines: Vec<String> = f.lines().flatten().collect();
    println!("Part 1 Result: {}", part1(&lines));
    println!("Part 2 Result: {}", part2(&lines));
}
