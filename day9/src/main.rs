use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).expect("Expected Digit"))
                .collect()
        })
        .collect()
}

fn read_input() -> Vec<Vec<u32>> {
    parse_input(read_to_string("input").expect("Failed to read file").trim())
}

fn get_low_points(height_map: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    let num_rows = height_map.len() as i32;
    let num_col = height_map[0].len() as i32;
    for i in 0..num_rows {
        for j in 0..num_col {
            let neighbors = get_neighbors(i, j, num_rows, num_col);
            if neighbors
                .iter()
                .all(|(x, y)| height_map[*x][*y] > height_map[i as usize][j as usize])
            {
                ret.push((i as usize, j as usize));
            }
        }
    }
    ret
}

fn get_neighbors(i: i32, j: i32, num_rows: i32, num_col: i32) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::with_capacity(4);
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let new_x = i + dx;
        let new_y = j + dy;
        if new_x >= 0 && new_x < num_rows && new_y >= 0 && new_y < num_col {
            neighbors.push((new_x as usize, new_y as usize))
        }
    }
    neighbors
}

fn calculate_basin_size(height_map: &[Vec<u32>], low_point: (usize, usize)) -> u32 {
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::from([low_point]);
    let mut visited: HashSet<(usize, usize)> = HashSet::from([low_point]);
    let mut ret = 0;
    while !frontier.is_empty() {
        let node = frontier.pop_front().unwrap();
        ret += 1;
        for neighbor in get_neighbors(
            node.0 as i32,
            node.1 as i32,
            height_map.len() as i32,
            height_map[0].len() as i32,
        ) {
            if height_map[neighbor.0][neighbor.1] > height_map[node.0][node.1]
                && height_map[neighbor.0][neighbor.1] != 9
                && !visited.contains(&neighbor)
            {
                visited.insert(neighbor);
                frontier.push_back(neighbor);
            }
        }
    }
    ret
}

fn part1(height_map: &[Vec<u32>]) -> u32 {
    get_low_points(height_map)
        .iter()
        .map(|(x, y)| height_map[*x][*y] + 1)
        .sum()
}

fn part2(height_map: &[Vec<u32>]) -> u32 {
    let mut basin_sizes: Vec<u32> = get_low_points(height_map)
        .iter()
        .filter(|(x, y)| height_map[*x][*y] != 9)
        .map(|low_point| calculate_basin_size(height_map, *low_point))
        .collect();
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn main() {
    let height_map = read_input();
    println!("Part 1: {}", part1(&height_map));
    println!("Part 2: {}", part2(&height_map));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pt_1() {
        let height_map = parse_input(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
        );
        assert_eq!(part1(&height_map), 15);
    }

    #[test]
    fn test_pt_2() {
        let height_map = parse_input(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
        );
        assert_eq!(part2(&height_map), 1134);
    }
}
