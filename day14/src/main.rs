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

struct Memoize {
    dp: HashMap<(char, char, usize), Counter<char>>,
    rules: HashMap<(char, char), char>,
}

impl Memoize {
    fn memoize(&mut self, a: char, b: char, num: usize) -> Counter<char> {
        if self.dp.contains_key(&(a, b, num)) {
            return self.dp[&(a, b, num)].clone();
        }
        let mid = self.rules[&(a, b)];
        let ret = self.memoize(a, mid, num - 1) + self.memoize(mid, b, num - 1);
        self.dp.get_mut(&(a, b, num)).unwrap().extend(&ret);
        ret
    }
}

fn run_polymerization(
    template: &[char],
    rules: HashMap<(char, char), char>,
    steps: usize,
) -> usize {
    let mut memoize = Memoize {
        dp: HashMap::new(),
        rules,
    };
    let mut char_counts: Counter<char> = Counter::new();
    for pair in template.windows(2) {
        char_counts += memoize.memoize(pair[0], pair[1], steps);
    }
    return char_counts.values().max().unwrap() - char_counts.values().min().unwrap();
}

fn main() {
    let input = read_to_string("input").expect("failed to read input");
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
        //assert_eq!(run_polymerization(&template, rules, 40), 2188189693529);
    }
}
