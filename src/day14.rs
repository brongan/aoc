use std::{collections::HashMap, fs::read_to_string};

use counter::Counter;

fn parse_input(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let input = input.trim().split_once("\n\n").expect("invalid input");
    let template = input.0.trim().chars().collect();
    let rules: HashMap<(char, char), char> = input
        .1
        .trim()
        .split('\n')
        .map(|line| {
            let terms = line.split_once(" -> ").expect("invalid line");
            let mut pair_input = terms.0.chars();
            (
                (
                    pair_input.next().expect("invalid rule"),
                    pair_input.next().expect("invalid rule"),
                ),
                terms.1.chars().next().expect("invalid rule"),
            )
        })
        .collect();
    (template, rules)
}

type Pair = (char, char);

fn run_polymerization(
    template: &[char],
    rules: HashMap<(char, char), char>,
    steps: usize,
) -> usize {
    let mut pair_counts: Counter<Pair> =
        Counter::init(template.windows(2).map(|window| (window[0], window[1])));
    for _ in 0..steps {
        let mut new_pair_counts: Counter<Pair> = Counter::new();
        for ((a, c), count) in pair_counts.iter() {
            let b = rules[&(*a, *c)];
            new_pair_counts[&(*a, b)] += count;
            new_pair_counts[&(b, *c)] += count;
        }
        pair_counts = new_pair_counts;
    }

    let mut char_counts: Counter<char> = Counter::new();
    for ((a, b), count) in pair_counts.iter() {
        char_counts[a] += count;
        char_counts[b] += count;
    }
    for (_c, count) in char_counts.iter_mut() {
        *count /= 2;
    }
    return char_counts.values().max().unwrap() - char_counts.values().min().unwrap() + 1;
}

pub fn main(input_path: &str) {
    let input = read_to_string(input_path).expect("failed to read input");
    let (template, rules) = parse_input(&input);
    println!(
        "Part 1: {}",
        run_polymerization(&template, rules.clone(), 10)
    );
    println!("Part 2: {}", run_polymerization(&template, rules, 40));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let (template, rules) = parse_input(input);
        assert_eq!(run_polymerization(&template, rules.clone(), 10), 1588);
        assert_eq!(run_polymerization(&template, rules, 40), 2188189693529);
    }
}
