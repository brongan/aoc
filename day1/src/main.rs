use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::zip;

fn main() {
    let f = File::open("input").expect("Failed to open input");
    let f = BufReader::new(f);
    let input: Vec<u32> = f
        .lines()
        .into_iter()
        .flatten()
        .map(|s| s.parse::<u32>().expect("Failed to parse input"))
        .collect();

    let mut count = 1;
    for (first, second) in zip(&input, &input[1..]) {
        if second > first && *first != 0 {
            count += 1
        }
    }
    println!("{} Increased!", count);

    let mut count = 1;
    let mut old_sum: u32 = input[0..3].iter().sum();
    let mut new_sum;
    for (i, num) in input[3..].iter().enumerate() {
        new_sum = old_sum + num - input[i];
        if new_sum > old_sum {
            count += 1
        }
        old_sum = new_sum
    }
    println!("{} Increased!", count);
}
