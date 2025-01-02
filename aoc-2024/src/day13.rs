use super::AOC2024;
use anyhow::Result;
use aoc_runner::{point2d::Point2D, Day, ParseInput, Part, Solution};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    sequence::{preceded, tuple},
    IResult,
};
use rayon::prelude::*;

type Num = i64;
type Point = Point2D<Num>;

#[derive(Debug, PartialEq)]
pub struct Claw {
    a: Point,
    b: Point,
    prize: Point,
}

type IR = Vec<Claw>;

fn parse_num(input: &str) -> IResult<&str, Num> {
    map_res(digit1, |num| Num::from_str_radix(num, 10))(input)
}

// Button A: X+24, Y+90
fn parse_a(input: &str) -> IResult<&str, Point> {
    let parse_x = |input| preceded(tag("Button A: X+"), parse_num)(input);
    let parse_y = |input| preceded(tag(", Y+"), parse_num)(input);
    map(tuple((parse_x, parse_y)), |(x, y)| Point { x, y })(input)
}

// Button B: X+85, Y+62
fn parse_b(input: &str) -> IResult<&str, Point> {
    let parse_x = |input| preceded(tag("Button B: X+"), parse_num)(input);
    let parse_y = |input| preceded(tag(", Y+"), parse_num)(input);
    map(tuple((parse_x, parse_y)), |(x, y)| Point { x, y })(input)
}

// Prize: X=6844, Y=6152
fn parse_prize(input: &str) -> IResult<&str, Point> {
    let parse_x = |input| preceded(tag("Prize: X="), parse_num)(input);
    let parse_y = |input| preceded(tag(", Y="), parse_num)(input);
    map(tuple((parse_x, parse_y)), |(x, y)| Point { x, y })(input)
}

fn parse_claw(input: &str) -> IResult<&str, Claw> {
    map(
        tuple((parse_a, newline, parse_b, newline, parse_prize)),
        |(a, _, b, _, prize)| Claw { a, b, prize },
    )(input)
}

impl ParseInput<'_, { Day::Day13 }> for AOC2024<{ Day::Day13 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .split("\n\n")
            .map(|claw| {
                let (_, claw) = parse_claw(claw).map_err(|e| e.to_owned())?;
                Ok(claw)
            })
            .collect::<Result<_>>()
    }
}

impl Claw {
    fn solve(&self) -> Option<(Num, Num)> {
        for a_presses in 0..100 {
            for b_presses in 0..100 {
                if self.a * a_presses + self.b * b_presses == self.prize {
                    return Some((a_presses, b_presses));
                }
            }
        }
        None
    }

    fn cramers(&self) -> Option<(Num, Num)> {
        let &Self { a, b, prize } = self;
        let determinant = a.x * b.y - a.y * b.x;
        let new_a = prize.x * b.y - b.x * prize.y;
        let new_b = a.x * prize.y - prize.x * a.y;
        if new_a % determinant != 0 || new_b % determinant != 0 {
            return None;
        }
        Some((new_a / determinant, new_b / determinant))
    }
}

impl Solution<'_, { Day::Day13 }, { Part::One }> for AOC2024<{ Day::Day13 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .par_iter()
            .map(|claw| claw.solve().map(|(a, b)| 3 * a + b).unwrap_or(0))
            .sum())
    }
}

impl Solution<'_, { Day::Day13 }, { Part::Two }> for AOC2024<{ Day::Day13 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .par_iter()
            .map(|claw| Claw {
                prize: claw.prize + 10000000000000,
                ..*claw
            })
            .map(|claw| claw.cramers().map(|(a, b)| 3 * a + b).unwrap_or(0))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner::PartOneVerifier;

    use super::*;

    #[test]
    fn test_parse_a() -> Result<()> {
        let input = "Button A: X+94, Y+34";
        let (rem, a) = parse_a(input)?;
        assert_eq!(rem, "");
        assert_eq!(a, Point { x: 94, y: 34 });
        Ok(())
    }

    #[test]
    fn test_parse_b() -> Result<()> {
        let input = "Button B: X+22, Y+67";
        let (rem, b) = parse_b(input)?;
        assert_eq!(rem, "");
        assert_eq!(b, Point { x: 22, y: 67 });
        Ok(())
    }

    #[test]
    fn test_parse_prize() -> Result<()> {
        let input = "Prize: X=8400, Y=5400";
        let (rem, point) = parse_prize(input)?;
        assert_eq!(rem, "");
        assert_eq!(point, Point { x: 8400, y: 5400 });
        Ok(())
    }

    #[test]
    fn test_parse_claw() -> Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        let (rem, claw) = parse_claw(input)?;
        assert_eq!(rem, "");
        assert_eq!(
            claw,
            Claw {
                a: Point { x: 94, y: 34 },
                b: Point { x: 22, y: 67 },
                prize: Point { x: 8400, y: 5400 },
            }
        );
        Ok(())
    }

    #[test]
    fn test() -> Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let problem = super::AOC2024::<{ Day::Day13 }>;
        assert_eq!(
            problem.parse_input(input)?,
            vec![
                Claw {
                    a: Point { x: 94, y: 34 },
                    b: Point { x: 22, y: 67 },
                    prize: Point { x: 8400, y: 5400 },
                },
                Claw {
                    a: Point { x: 26, y: 66 },
                    b: Point { x: 67, y: 21 },
                    prize: Point { x: 12748, y: 12176 },
                },
                Claw {
                    a: Point { x: 17, y: 86 },
                    b: Point { x: 84, y: 37 },
                    prize: Point { x: 7870, y: 6450 },
                },
                Claw {
                    a: Point { x: 69, y: 23 },
                    b: Point { x: 27, y: 71 },
                    prize: Point { x: 18641, y: 10279 },
                },
            ]
        );
        problem.test_part1(input, 480)?;
        Ok(())
    }
}
