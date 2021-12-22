#![feature(int_abs_diff)]
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::vec;

#[derive(Clone, Copy, Default, Debug)]
struct Point2D<T> {
    x: T,
    y: T,
}

type VentLine = (Point2D<i32>, Point2D<i32>);
type Seafloor = Vec<Vec<i32>>;

fn to_vent_line(line: &str) -> Option<VentLine> {
    let (p1, p2) = line.split_once(" -> ")?;
    let p1 = p1.split_once(',')?;
    let p2 = p2.split_once(',')?;
    let y1 = p1.0.parse().ok()?;
    let y2 = p2.0.parse().ok()?;
    let x1 = p1.1.parse().ok()?;
    let x2 = p2.1.parse().ok()?;
    Some((Point2D { x: x1, y: y1 }, Point2D { x: x2, y: y2 }))
}
fn parse_lines(lines: Lines<BufReader<File>>) -> Vec<VentLine> {
    lines
        .filter_map(|l| l.ok())
        .flat_map(|l| to_vent_line(l.as_str()))
        .collect()
}

fn seafloor_from_lines(
    lines: &[VentLine],
    filter: fn(VentLine) -> bool,
    max_index: usize,
) -> Seafloor {
    let mut seafloor = vec![vec![0i32; max_index]; max_index];

    for line in lines.iter().filter(|line| filter(**line)) {
        for row in &seafloor {
            println!("{:?}", row);
        }
        println!();
        if line.1.y - line.0.y == 0 {
            for x in min(line.0.x, line.1.x)..max(line.0.x, line.1.x) + 1 {
                seafloor[x as usize][line.0.y as usize] += 1;
            }
        } else if line.1.x - line.0.x == 0 {
            for y in min(line.0.y, line.1.y)..max(line.0.y, line.1.y) + 1 {
                seafloor[line.0.x as usize][y as usize] += 1;
            }
        } else {
            let (start, end) = if line.0.x < line.1.x {
                (line.0, line.1)
            } else {
                (line.1, line.0)
            };

            let delta_y = if end.y > start.y { 1 } else { -1 };

            for i in start.x..(end.x - start.x) {
                seafloor[(start.x + i) as usize][(start.y + delta_y) as usize] += 1
            }
        }
    }

    println!("printing seafloor");
    for row in &seafloor {
        println!("{:?}", row);
    }

    seafloor
}

fn part1(lines: &[VentLine], max: usize) -> usize {
    let seafloor = seafloor_from_lines(lines, part1_filter, max);

    seafloor.iter().flatten().filter(|x| **x >= 2).count()
}

fn part1_filter(line: VentLine) -> bool {
    line.0.x == line.1.x || line.0.y == line.1.y
}

fn part2(lines: &[VentLine], max: usize) -> usize {
    fn part2_filter(line: VentLine) -> bool {
        part1_filter(line)
            || (i32::abs_diff(line.0.x, line.1.x) == 1 && i32::abs_diff(line.0.y, line.1.y) == 1)
    }
    let seafloor = seafloor_from_lines(lines, part2_filter, max);
    seafloor.iter().flatten().filter(|x| **x >= 2).count()
}

fn read_input(path: &str) -> Lines<BufReader<File>> {
    let f = File::open(path).expect("Failed to open input file");
    let f = BufReader::new(f);
    f.lines()
}

fn main() {
    let lines: Vec<VentLine> = parse_lines(read_input("input"));
    println!("Part 1: {}", part1(&lines, 1000));
    println!("Part 2: {}", part2(&lines, 1000));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_overflow() {
        println!("what is code");
    }

    #[test]
    fn test_pt_1() {
        let lines: Vec<VentLine> = parse_lines(read_input("test"));
        assert_eq!(part1(&lines, 10), 5);
    }

    #[test]
    fn test_pt_2() {
        let lines: Vec<VentLine> = parse_lines(read_input("test"));
        assert_eq!(part2(&lines, 10), 12);
    }
}
