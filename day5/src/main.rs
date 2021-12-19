use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::vec;

struct Point2D<T> {
    x: T,
    y: T,
}

type VentLine = (Point2D<u32>, Point2D<u32>);
type Seafloor = Vec<Vec<u32>>;

fn to_vent_line(line: &str) -> Option<VentLine> {
    let (p1, p2) = line.split_once(" -> ")?;
    let p1 = p1.split_once(',')?;
    let p2 = p2.split_once(',')?;
    let x1 = p1.0.parse().ok()?;
    let x2 = p2.0.parse().ok()?;
    let y1 = p1.1.parse().ok()?;
    let y2 = p2.1.parse().ok()?;
    Some((Point2D { x: x1, y: y1 }, Point2D { x: x2, y: y2 }))
}
fn parse_lines(lines: Lines<BufReader<File>>) -> Vec<VentLine> {
    lines
        .filter_map(|l| l.ok())
        .flat_map(|l| to_vent_line(l.as_str()))
        .collect()
}

fn seafloor_from_lines(lines: Vec<VentLine>, filter: fn(VentLine) -> bool, max: usize) -> Seafloor {
    let mut seafloor = vec![vec![0u32; max]; max];
    for line in lines.iter().filter(filter) {
        if y2 - y1 == 0 {
            for x in min(x1, x2)..max(x1..x2) + 1 {
                seafloor[x][y1] += 1;
            }
        } else if x2 - x1 == 0 {
            for y in min(y1, y2)..max(y1..y2) + 1 {
                seafloor[x1][y] += 1;
            }
        }
    }
    seafloor
}

fn part1(lines: Vec<VentLine>, max: usize) -> usize {
    fn part1_filter(line: VentLine) -> bool {
        line.0.x == line.1.x || line.0.y == line.1.y
    }
    let seafloor = seafloor_from_lines(lines, part1_filter, max);
    seafloor.iter().flatten().filter(|x| **x >= 2).count()
}

fn read_input(path: &str) -> Lines<BufReader<File>> {
    let f = File::open(path).expect("Failed to open input file");
    let f = BufReader::new(f);
    f.lines()
}

fn main() {
    let lines: Vec<VentLine> = parse_lines(read_input("input"));
    println!("Part 1: {}", part1(lines, 1000));
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
        assert_eq!(part1(lines, 10), 5);
    }
}
