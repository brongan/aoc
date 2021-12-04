use std::fs::File;
use std::io::{BufRead, BufReader};

fn most_common_elements(lines: Vec<String>) -> Vec<char> {
    return (0..lines[0].len())
        .map(|col| {
            let mut one_count: i64 = 0;
            for line in lines.iter() {
                if line.as_bytes()[col] == b'1' {
                    one_count += 1
                }
            }
            if one_count > lines.len() as i64 - one_count {
                '1'
            } else {
                '0'
            }
        })
        .collect();
}
fn main() {
    let f = File::open("input").expect("Failed to open input");
    let f = BufReader::new(f);
    let lines: Vec<String> = f.lines().flatten().collect();
    let line_length = lines[0].len();

    // calculate number of ones in each column
    // Part 1
    let mut gamma_rate: Vec<char> = Vec::with_capacity(line_length);
    let mut epsilon_rate: Vec<char> = Vec::with_capacity(line_length);
    println!("Line Length: {}", line_length);
    for elem in most_common_elements(lines) {
        if elem == '1' {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }
    let gamma_rate_string: String = gamma_rate.clone().into_iter().collect();
    let epsilon_rate_string: String = epsilon_rate.clone().into_iter().collect();
    println!("Gamma Rate: {:?} {}", &gamma_rate, gamma_rate_string);
    println!("Epsilon Rate: {:?} {}", &epsilon_rate, epsilon_rate_string);
    let result = u32::from_str_radix(&gamma_rate_string, 2).unwrap()
        * u32::from_str_radix(&epsilon_rate_string, 2).unwrap();
    println!("Result: {}", result);

    // Part 2
    // Oxygen generator rating + co2 scrubber rating
    let mut oxygen_gen_lines: Vec<String> = lines.clone();
    let mut col: usize = 0;
    while oxygen_gen_lines.len() > 1 {
        let commonz = most_common_elements(oxygen_gen_lines);
        oxygen_gen_lines = oxygen_gen_lines
            .into_iter()
            .filter(|line| line.as_bytes()[col] == commonz[col] as u8)
            .collect();
        col += 1;
    }
}
