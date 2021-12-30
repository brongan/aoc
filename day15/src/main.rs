use std::cmp::Ordering;
use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("invalid digit"))
                .collect()
        })
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct CaveNode {
    pos: (usize, usize),
    path_dist: u32,
}

impl Ord for CaveNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.path_dist.cmp(&self.path_dist)
    }
}

impl PartialOrd for CaveNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn djikstra(map: &[Vec<u32>]) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<(usize, usize), u32> = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            dist.insert((i, j), u32::MAX);
        }
    }

    let get_neighbors = |pos: (usize, usize)| -> Vec<(usize, usize)> {
        let mut ret: Vec<(usize, usize)> = Vec::with_capacity(4);
        for (dx, dy) in [(0i32, 1i32), (1i32, 0i32), (0i32, -1i32), (-1i32, 0i32)] {
            let x = pos.0 as i32 + dx;
            let y = pos.1 as i32 + dy;
            if x >= 0 && x < map.len() as i32 && y >= 0 && y < map.len() as i32 {
                ret.push((x as usize, y as usize));
            }
        }
        ret
    };

    let initial_point = (0, 0);
    let goal = (map.len() - 1, map[0].len() - 1);
    heap.push(CaveNode {
        pos: initial_point,
        path_dist: 0,
    });

    while let Some(CaveNode { pos, path_dist }) = heap.pop() {
        if pos == goal {
            return path_dist;
        }
        if path_dist > dist[&pos] {
            continue;
        }

        for neighbor in get_neighbors(pos) {
            let next = CaveNode {
                pos: neighbor,
                path_dist: path_dist + map[neighbor.0][neighbor.1],
            };
            if next.path_dist < dist[&neighbor] {
                heap.push(next);
                dist.insert(next.pos, next.path_dist);
            }
        }
    }
    0
}

fn part1(map: &[Vec<u32>]) -> u32 {
    djikstra(map)
}

fn part2(map: &mut Vec<Vec<u32>>) -> u32 {
    let initial_size = map.len();
    for row in map.iter_mut().take(initial_size) {
        row.reserve(initial_size * 4);
        for j in 0..initial_size * 4 {
            let old = row[j];
            row.push((old + 1) % 10);
        }
    }

    for i in 0..initial_size * 4 {
        map.push(Vec::with_capacity(initial_size * 5));
        for j in 0..initial_size * 5 {
            let new = (map[i][j] + 1) % 10;
            map[initial_size + i].push(new);
        }
    }
    djikstra(map)
}

fn main() {
    let mut input = parse_input(
        read_to_string("input")
            .expect("failed to read input")
            .trim(),
    );
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&mut input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let mut input = parse_input(input);
        assert_eq!(part1(&input), 40);
        assert_eq!(part2(&mut input), 315);
    }
}
