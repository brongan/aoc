use super::AOC2021;
use crate::aoc::{Day, ParseInput, Part, Solution};

use nom::{
  IResult,
  sequence::{separated_pair, delimited}};
use nom::character::complete::{char, digit1};
use nom::branch::alt;
use nom::combinator::{map_res, map};
use core::str::FromStr;
use std::{fmt::Display, iter::Sum, cell::RefCell, rc::Rc, borrow::Borrow};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum SnailFishNumber {
    Pair(Rc<RefCell<SnailFishNumber>>, Rc<RefCell<SnailFishNumber>>),
    Reg(u32),
}

impl Display for SnailFishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailFishNumber::Pair(l, r) => {
                let l: &RefCell<SnailFishNumber> = l.borrow();
                let r: &RefCell<SnailFishNumber> = r.borrow();
                write!(f, "[{},{}]", l.borrow(), r.borrow())
            },
            SnailFishNumber::Reg(num) => write!(f, "{}", num),
        }
    }
}

type Inorder = Vec<(Rc<RefCell<SnailFishNumber>>, u32)>;
impl SnailFishNumber {
    fn inorder_traversal_helper(snailfish_num: &Rc<RefCell<Self>>, depth: u32, inorder: &mut Inorder) {
        match *<Rc<RefCell<SnailFishNumber>> as Borrow<RefCell<SnailFishNumber>>>::borrow(snailfish_num).borrow() {
            SnailFishNumber::Pair(l, r) => {
                Self::inorder_traversal_helper(&l, depth + 1, inorder);
                inorder.push((Rc::clone(&snailfish_num), depth));
                Self::inorder_traversal_helper(&r, depth + 1, inorder);
            }
            SnailFishNumber::Reg(_) => {
                inorder.push((Rc::clone(&snailfish_num), depth));
            }
        }
    }

    fn inorder_traversal(snailfish_num: &Rc<RefCell<Self>>) -> Inorder {
        let mut ret = Vec::new();
        Self::inorder_traversal_helper(snailfish_num ,0, &mut ret);
        ret
    }

    // To reduce a snailfish number, you must repeatedly do the first action in this list that applies to the snailfish number:
    // If any pair is nested inside four pairs, the leftmost such pair explodes.
    // If any regular number is 10 or greater, the leftmost such regular number splits.
    fn number_reduce(snailfish_num: &Rc<RefCell<Self>>) -> bool {
        let mut inorder = Self::inorder_traversal(snailfish_num);
        for (i, (num, depth)) in inorder.iter().enumerate() {
            match (num, depth >= &4) {
                (p, true) => 
                    if let SnailFishNumber::Pair(l, r) = &*p.borrow_mut() {
                        if matches!(*<Rc<RefCell<SnailFishNumber>> as Borrow<RefCell<SnailFishNumber>>>::borrow(snailfish_num).borrow(), SnailFishNumber::Reg(_)) && matches!(*<Rc<RefCell<SnailFishNumber>> as Borrow<RefCell<SnailFishNumber>>>::borrow(snailfish_num).borrow(), SnailFishNumber::Reg(_)) {
                            p.borrow_mut().explode(&mut inorder, i);
                            return true;
                        }
                    },
                _ => (),
            }
        }
        for (num, _) in inorder.iter() {
            if let SnailFishNumber::Reg(val) = &*num.borrow_mut() {
                if val >= &10 {
                    *num.borrow_mut() = SnailFishNumber::Pair(
                        Rc::new(RefCell::new(SnailFishNumber::Reg(val / 2))),
                        Rc::new(RefCell::new(SnailFishNumber::Reg(val / 2 + val % 2)))
                        );
                }
            }
        }

        false
    }

    //To explode a pair, the pair's left value is added to the first regular number to the left of the exploding pair (if any), and the pair's right value is added to the first regular number to the right of the exploding pair (if any). Exploding pairs will always consist of two regular numbers. Then, the entire exploding pair is replaced with the regular number 0.
    fn explode(&mut self, inorder: &mut Inorder, index: usize) {
        let (l, r) = match &*self {
            SnailFishNumber::Pair(l, r) => {
                if let SnailFishNumber::Reg(l) = *l.borrow_mut() {
                    if let SnailFishNumber::Reg(r) = *r.borrow_mut() {
                        (l, r)
                    } else {
                    panic!("cannot explode pair containing non regular numbers");
                    }
                } else {
                    panic!("cannot explode pair containing non regular numbers");
                }
            }
            _ => panic!("cannot explode reg number"),
        };

        for i in (index - 1)..0 {
            let (num, _depth) = &mut inorder[i];
            if let SnailFishNumber::Reg(val) = *<Rc<RefCell<SnailFishNumber>> as Borrow<RefCell<SnailFishNumber>>>::borrow(num).borrow() {
                *num = Rc::new(RefCell::new(SnailFishNumber::Reg(val + l)));
            }
        }

        for i in (index + 1)..inorder.len() {
            let (num, _depth) = &mut inorder[i];
            if let SnailFishNumber::Reg(val) = *<Rc<RefCell<SnailFishNumber>> as Borrow<RefCell<SnailFishNumber>>>::borrow(num).borrow() {
                *num = Rc::new(RefCell::new(SnailFishNumber::Reg(val + r)));
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailFishNumber::Pair(l, r) => 3 * (*<Rc<RefCell<SnailFishNumber>> as Borrow<RefCell<SnailFishNumber>>>::borrow(l).borrow()).magnitude() + 2 * (*<Rc<RefCell<SnailFishNumber>> as Borrow<RefCell<SnailFishNumber>>>::borrow(l).borrow()).magnitude(),
            SnailFishNumber::Reg(num) => *num,
        }
    }
}


fn add(l: Rc<RefCell<SnailFishNumber>>, r: Rc<RefCell<SnailFishNumber>>) -> Rc<RefCell<SnailFishNumber>> {
    let ret = Rc::new(RefCell::new(SnailFishNumber::Pair(l, r)));
    while SnailFishNumber::number_reduce(&ret) {}
    ret
}

impl Sum<Rc<RefCell<SnailFishNumber>>> for SnailFishNumber {
    fn sum<I: Iterator<Item = Rc<RefCell<SnailFishNumber>>>>(mut iter: I) -> Self {
        let ret = iter.next();
        if ret == None { return SnailFishNumber::Reg(0) }
        let mut ret = ret.unwrap();

        loop {
            match iter.next() {
                Some(other) => ret = add(ret, other),
                None => return (*<Rc<RefCell<SnailFishNumber>> as Borrow<RefCell<SnailFishNumber>>>::borrow(&ret).borrow()).clone(),
            };
        }
    }
}

fn map_from_str<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn pair(input: &str) -> IResult<&str, SnailFishNumber> {
    delimited(
        char('['),
        map(separated_pair(snail_number, char(','), snail_number),
        |(l,r)| SnailFishNumber::Pair(Rc::new(RefCell::new(l)), Rc::new(RefCell::new(r)))),
        char(']'),
    )(input)
}

fn value(input: &str) -> IResult<&str, SnailFishNumber> {
    map(map_from_str, SnailFishNumber::Reg)(input)
}

fn snail_number(input: &str) -> IResult<&str, SnailFishNumber> {
    alt((pair, value))(input)
}

impl ParseInput<'_, { Day::Day18 }> for AOC2021<{ Day::Day18 }> {
    type Parsed = Vec<Rc<RefCell<SnailFishNumber>>>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input.lines().map(|l| snail_number(l).expect("failed to parse line")).map(|(_, v)| Rc::new(RefCell::new(v))).collect()
    }
}

impl Solution<'_, { Day::Day18 }, { Part::One }> for AOC2021<{ Day::Day18 }> {
    type Input = Vec<Rc<RefCell<SnailFishNumber>>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let sum: SnailFishNumber = input.clone().into_iter().sum();
        sum.magnitude()
    }
}

