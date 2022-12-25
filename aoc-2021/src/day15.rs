use super::AOC2021;
use anyhow::{Context, Result};
use aoc_runner::{Day, ParseInput, Part, Solution};
use core::panic;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::{BinaryHeap, HashMap};

use colored::Colorize;

type Point = (usize, usize);

impl ParseInput<'_, { Day::Day15 }> for AOC2021<{ Day::Day15 }> {
    type Parsed = Vec<Vec<u32>>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).context("invalid digit"))
                    .collect()
            })
            .collect::<Result<Vec<Vec<u32>>>>()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct CaveNode {
    pos: Point,
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

fn calculate_final_path(
    initial_point: Point,
    goal_point: Point,
    prev: &HashMap<Point, Point>,
) -> HashSet<Point> {
    let mut ret = HashSet::from([initial_point, goal_point]);
    let mut curr = goal_point;
    while curr != initial_point {
        ret.insert(curr);
        curr = prev[&curr];
    }
    ret
}

fn djikstra(map: &[Vec<u32>]) -> Option<(HashSet<Point>, u32)> {
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<Point, u32> = HashMap::new();
    let mut prev: HashMap<Point, Point> = HashMap::new();
    let size = map.len();
    for i in 0..size {
        for j in 0..size {
            dist.insert((i, j), u32::MAX);
        }
    }

    let get_neighbors = |pos: Point| -> Vec<Point> {
        let mut ret: Vec<Point> = Vec::with_capacity(4);
        for (dx, dy) in [(0i32, 1i32), (1i32, 0i32), (0i32, -1i32), (-1i32, 0i32)] {
            let x = pos.0 as i32 + dx;
            let y = pos.1 as i32 + dy;
            if x >= 0 && x < size as i32 && y >= 0 && y < size as i32 {
                ret.push((x as usize, y as usize));
            }
        }
        ret
    };

    let initial_point = (0, 0);
    let goal_point = (size - 1, size - 1);
    heap.push(CaveNode {
        pos: initial_point,
        path_dist: 0,
    });

    while let Some(CaveNode { pos, path_dist }) = heap.pop() {
        if pos == goal_point {
            return Some((
                calculate_final_path(initial_point, goal_point, &prev),
                path_dist,
            ));
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
                prev.insert(neighbor, pos);
                heap.push(next);
                dist.insert(next.pos, next.path_dist);
            }
        }
    }
    None
}

#[allow(dead_code)]
fn print_map(map: &[Vec<u32>], prev: &HashSet<Point>) {
    let size = map.len();
    for (i, row) in map.iter().enumerate().take(size) {
        for (j, elem) in row.iter().enumerate().take(size) {
            if prev.contains(&(i, j)) {
                print!("{}", elem.to_string().bright_red());
            } else {
                print!("{elem}");
            }
        }
        println!();
    }
}

impl Solution<'_, { Day::Day15 }, { Part::One }> for AOC2021<{ Day::Day15 }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let (_prev, path_cost) = djikstra(input).context("Did not find solution")?;
        //print_map(input, &prev);
        Ok(path_cost)
    }
}

fn part2_extend(map: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let initial_size = map.len();
    let extended_size = initial_size * 5;
    let mut ret = vec![vec![0; extended_size]; extended_size];
    for (i, row) in ret.iter_mut().enumerate().take(extended_size) {
        for (j, elem) in row.iter_mut().enumerate().take(extended_size) {
            let di = i % initial_size;
            let dj = j % initial_size;
            let delta = (i / initial_size + j / initial_size) as u32;
            let val = match map[di][dj] + delta {
                1 | 10 => 1,
                2 | 11 => 2,
                3 | 12 => 3,
                4 | 13 => 4,
                5 | 14 => 5,
                6 | 15 => 6,
                7 | 16 => 7,
                8 | 17 => 8,
                9 | 18 => 9,
                _ => panic!("bakana"),
            };
            *elem = val;
        }
    }
    ret
}

impl Solution<'_, { Day::Day15 }, { Part::Two }> for AOC2021<{ Day::Day15 }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let input = part2_extend(input.to_vec());
        let (_prev, path_cost) = djikstra(&input).context("Did not find solution")?;
        //print_map(&input, &prev);
        Ok(path_cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    #[test]
    fn test() -> Result<()> {
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
        let expected_extended_input = "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479";
        let problem = super::AOC2021::<{ Day::Day15 }>;
        let parsed_input = problem.parse_input(input)?;
        problem.test_part1(input, 40)?;

        let expected_extended_input = problem.parse_input(expected_extended_input)?;
        let extended_input = part2_extend(parsed_input);
        assert_eq!(extended_input, expected_extended_input);
        problem.test_part2(input, 315)
    }
}
