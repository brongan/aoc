use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input").expect("Failed to open input");
    let f = BufReader::new(f);
    let mut position = 0;
    let mut depth = 0;
    let lines: Vec<String> = f.lines().flatten().collect();

    for line in lines.iter() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let distance = words[1].parse::<u32>().expect("Failed to parse distance");
        if "forward" == words[0] {
            position += distance;
        } else if "down" == words[0] {
            depth += distance;
        } else if "up" == words[0] {
            depth -= distance;
        }
    }

    println!(
        "Position: {}, Depth: {}, Value: {}",
        position,
        depth,
        position * depth
    );

    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let value = words[1].parse::<u32>().expect("Failed to parse distance");
        if "forward" == words[0] {
            position += value;
            depth += aim * value;
        } else if "down" == words[0] {
            aim += value;
        } else if "up" == words[0] {
            aim -= value;
        }
    }

    println!(
        "Position: {}, Depth: {}, Aim: {}, Value: {}",
        position,
        depth,
        aim,
        position * depth
    )
}
