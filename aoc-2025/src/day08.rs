use std::str::FromStr;

use super::AOC2025;
use anyhow::{Context, Result};
use aoc_runner::graph::{Edge, Graph, kruskal, kruskal_with_limit};
use aoc_runner::point3d::{Point3D, euclidean_distance_squared};
use aoc_runner::{Day, ParseInput, Part, Solution};

impl ParseInput<'_, { Day::Day8 }> for AOC2025<{ Day::Day8 }> {
    type Parsed = Graph3D;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let points: Vec<Point3D<i64>> = input
            .trim()
            .lines()
            .map(Point3D::<i64>::from_str)
            .map(|thing| thing.unwrap())
            .collect();
        let mut edges = Vec::new();
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                edges.push(Edge3D::new(i, j, points[i], points[j]));
            }
        }
        Ok(Graph3D {
            edges,
            vertices: points.len(),
        })
    }
}

#[derive(Copy, Clone)]
pub struct Edge3D {
    first: usize,
    second: usize,
    dist_squared: i64,
    coord1: Point3D<i64>,
    coord2: Point3D<i64>,
}

impl Edge3D {
    fn new(first: usize, second: usize, coord1: Point3D<i64>, coord2: Point3D<i64>) -> Self {
        let dist_squared = euclidean_distance_squared(coord1, coord2);

        Self {
            first,
            second,
            dist_squared,
            coord1,
            coord2,
        }
    }
}

impl Edge for Edge3D {
    fn first_vertex(&self) -> usize {
        self.first
    }

    fn second_vertex(&self) -> usize {
        self.second
    }
}

pub struct Graph3D {
    edges: Vec<Edge3D>,
    vertices: usize,
}

impl Graph for Graph3D {
    type E = Edge3D;

    fn edges_ordered_by_weight(&self) -> Vec<Self::E> {
        let mut edges = self.edges.clone();
        edges.sort_unstable_by(|l, r| l.dist_squared.cmp(&r.dist_squared));
        edges
    }

    fn number_vertices(&self) -> usize {
        self.vertices
    }
}

impl Solution<'_, { Day::Day8 }, { Part::One }> for AOC2025<{ Day::Day8 }> {
    type Input = Graph3D;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut connected_components = kruskal_with_limit(input, 1000).sets();
        connected_components.sort_by_key(|set| set.len());
        let ret = connected_components
            .iter()
            .rev()
            .take(3)
            .map(|set| set.len())
            .product();

        Ok(ret)
    }
}

impl Solution<'_, { Day::Day8 }, { Part::Two }> for AOC2025<{ Day::Day8 }> {
    type Input = Graph3D;
    type Output = i64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(kruskal(input)
            .last()
            .map(|edge| edge.coord1.x * edge.coord2.x)
            .context("Empty Kruskal??")?)
    }
}
