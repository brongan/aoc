use std::collections::VecDeque;
use aoc_runner::ParseInput;
use aoc_runner::{Day, Solution, Part};
use super::AOC2021;


#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

type Observation = (i32, i32, i32);
type ScanResult = Vec<Observation>;
type Beacon = Point3D<i32>;
type Scanner = Point3D<i32>;

enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

fn to_observation(line: &str) -> Option<Observation> {
    let mut iter = line.split(",");
    let x = iter.next()?.parse().ok()?;
    let y = iter.next()?.parse().ok()?;
    let z = iter.next()?.parse().ok()?;
    Some((x, y, z))
}

fn parse_scanner(lines: &str) -> ScanResult {
    lines
        .split("\n")
        .skip(1)
        .map(|line| to_observation(&line).unwrap())
        .collect()
}

impl ParseInput<'_, {Day::Day19}> for AOC2021<{Day::Day19}> {
    type Parsed = Vec<ScanResult>;
    
    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
    input
        .split("\n\n")
        .map(|lines| parse_scanner(lines))
        .collect()
    }
}

fn match_beacons(scan_result: &Vec<Observation>, beacons: &mut Vec<Beacon>) -> Option<Scanner> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;

    #[test]
    fn test_find_beacons() {
        let (beacons, scanners) = find_positions(input);
        assert_eq!(scanners, vec![Beacon {x: 0, y: 0, z: 0}, Beacon { x: 68, y: -1246, z: -43}, Beacon {x: 1105, y: -1205,z: 1229}, Beacon {x: -92, y: -2380,z: -20}]);
    }

    #[test]
    fn test() -> Result<(), String> {
        let input: &str = "
            --- scanner 0 ---
            404,-588,-901
            528,-643,409
            -838,591,734
            390,-675,-793
            -537,-823,-458
            -485,-357,347
            -345,-311,381
            -661,-816,-575
            -876,649,763
            -618,-824,-621
            553,345,-567
            474,580,667
            -447,-329,318
            -584,868,-557
            544,-627,-890
            564,392,-477
            455,729,728
            -892,524,684
            -689,845,-530
            423,-701,434
            7,-33,-71
            630,319,-379
            443,580,662
            -789,900,-551
            459,-707,401

            --- scanner 1 ---
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            95,138,22
            -476,619,847
            -340,-569,-846
            567,-361,727
            -460,603,-452
            669,-402,600
            729,430,532
            -500,-761,534
            -322,571,750
            -466,-666,-811
            -429,-592,574
            -355,545,-477
            703,-491,-529
            -328,-685,520
            413,935,-424
            -391,539,-444
            586,-435,557
            -364,-763,-893
            807,-499,-711
            755,-354,-619
            553,889,-390

            --- scanner 2 ---
            649,640,665
            682,-795,504
            -784,533,-524
            -644,584,-595
            -588,-843,648
            -30,6,44
            -674,560,763
            500,723,-460
            609,671,-379
            -555,-800,653
            -675,-892,-343
            697,-426,-610
            578,704,681
            493,664,-388
            -671,-858,530
            -667,343,800
            571,-461,-707
            -138,-166,112
            -889,563,-600
            646,-828,498
            640,759,510
            -630,509,768
            -681,-892,-333
            673,-379,-804
            -742,-814,-386
            577,-820,562

            --- scanner 3 ---
            -589,542,597
            605,-692,669
            -500,565,-823
            -660,373,557
            -458,-679,-417
            -488,449,543
            -626,468,-788
            338,-750,-386
            528,-832,-391
            562,-778,733
            -938,-730,414
            543,643,-506
            -524,371,-870
            407,773,750
            -104,29,83
            378,-903,-323
            -778,-728,485
            426,699,580
            -438,-605,-362
            -469,-447,-387
            509,732,623
            647,635,-688
            -868,-804,481
            614,-800,639
            595,780,-596

            --- scanner 4 ---
            727,592,562
            -293,-554,779
            441,611,-461
            -714,465,-776
            -743,427,-804
            -660,-479,-426
            832,-632,460
            927,-485,-438
            408,393,-506
            466,436,-512
            110,16,151
            -258,-428,682
            -393,719,612
            -211,-452,876
            808,-476,-593
            -575,615,604
            -485,667,467
            -680,325,-822
            -627,-443,-432
            872,-547,-609
            833,512,582
            807,604,487
            839,-516,451
            891,-625,532
            -652,-548,-490
            30,-46,-14
            ";
            let problem = super::AOC2021::<{ Day::Day19 }>;
            problem.test_part1(input, 79)
    }
}

fn find_positions(mut input: VecDeque<ScanResult>) -> (Vec<Beacon>, Vec<Scanner>) {
    let mut beacons: Vec<Beacon> = input[0].iter().map(|obs| Point3D { x: obs.0, y: obs.1, z: obs.2 }).collect();
    input.pop_front();
    let mut scanners = vec![Scanner { x: 0, y: 0, z: 0 }];

    for scan_result in input {
        if let Some(scanner) = match_beacons(&scan_result, &mut beacons) {
            scanners.push(scanner)
        }
    }
    (beacons, scanners)
}

impl Solution<'_, { Day::Day19 }, { Part::One }> for AOC2021<{ Day::Day19 }> {
    type Input = ScanResult;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let (beacons, _scanners) = find_positions(input.clone());
        beacons.len()
    }
}

