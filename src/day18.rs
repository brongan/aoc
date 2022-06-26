use std::str::FromStr;

enum SnailFishNumber {
    Pair(Box<SnailFishPair>),
    Num(u32),
}

struct SnailFishPair {
    left: SnailFishNumber,
    right: SnailFishNumber,
}

impl FromStr for SnailFishPair {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SnailFishPair {
            left: SnailFishNumber::Num(0),
            right: SnailFishNumber::Num(0),
        })
    }
}

pub fn solve(input_path: &str) {}
