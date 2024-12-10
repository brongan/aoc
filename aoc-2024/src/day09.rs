use super::AOC2024;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

#[derive(Debug, PartialEq)]
pub struct Element {
    file_blocks: u32,
    free_blocks: u32,
}

type DiskMap = Vec<Element>;

impl ParseInput<'_, { Day::Day9 }> for AOC2024<{ Day::Day9 }> {
    type Parsed = DiskMap;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let nums: Vec<u32> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        Ok(nums
            .chunks(2)
            .map(|chunk| Element {
                file_blocks: chunk[0],
                free_blocks: *chunk.get(1).unwrap_or(&0),
            })
            .collect())
    }
}

type Block = usize;
type Disk = Vec<Option<Block>>;

fn to_disk(disk_map: &DiskMap) -> Disk {
    let size: u32 = disk_map
        .iter()
        .map(|element| element.file_blocks + element.free_blocks)
        .sum();

    let mut ret = vec![None; size as usize];
    let mut index: usize = 0;
    for (file_id, element) in disk_map.iter().enumerate() {
        for i in 0..element.file_blocks {
            ret[index + i as usize] = Some(file_id);
        }
        index += (element.file_blocks + element.free_blocks) as usize;
    }
    ret
}

fn compact(disk: &mut Disk) {
    let mut l = 0;
    let mut r = disk.len() - 1;

    while l < r {
        while disk[l].is_some() {
            l += 1;
        }
        while disk[r].is_none() {
            r -= 1;
        }
        if l < r {
            disk.swap(l, r);
        }
    }
}

fn defragment(disk: &DiskMap) -> Disk {
    let size: u32 = disk
        .iter()
        .map(|element| element.file_blocks + element.free_blocks)
        .sum();
    let file = disk.last().unwrap();

    // find last file
    // find first empty block we can stuff it in
    // repeat until 
    let r = 
}

fn checksum(disk: Disk) -> usize {
    disk.into_iter()
        .enumerate()
        .map(|(i, file_id)| i * file_id.unwrap_or(0))
        .sum()
}

impl Solution<'_, { Day::Day9 }, { Part::One }> for AOC2024<{ Day::Day9 }> {
    type Input = DiskMap;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut disk = to_disk(input);
        compact(&mut disk);
        Ok(checksum(disk))
    }
}

impl Solution<'_, { Day::Day9 }, { Part::Two }> for AOC2024<{ Day::Day9 }> {
    type Input = DiskMap;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let disk= defragment(input);
        Ok(checksum(disk))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    #[test]
    fn test_parsing() -> Result<()> {
        let problem = super::AOC2024::<{ Day::Day9 }>;
        assert_eq!(
            problem.parse_input("90909")?,
            vec![
                Element {
                    file_blocks: 9,
                    free_blocks: 0
                },
                Element {
                    file_blocks: 9,
                    free_blocks: 0
                },
                Element {
                    file_blocks: 9,
                    free_blocks: 0
                }
            ]
        );
        Ok(())
    }

    #[test]
    fn test_example() -> Result<()> {
        let problem = super::AOC2024::<{ Day::Day9 }>;
        let input = "2333133121414131402";
        problem.test_part1(input, 1928)?;
        problem.test_part2(input, 2858)?;

        Ok(())
    }
}
