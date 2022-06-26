use std::cmp::max;

use crate::point2d::Point2D;

struct Velocity {
    dx: i32,
    dy: i32,
}

#[derive(Clone, Copy)]
struct TargetArea {
    bottom_left: Point2D<i32>,
    top_right: Point2D<i32>,
}

enum ProbeStatus {
    Before,
    Hit,
    Past,
}

fn in_target(p: &Point2D<i32>, target: &TargetArea) -> ProbeStatus {
    match (
        target.bottom_left.x < p.x,
        p.x < target.top_right.x,
        target.bottom_left.y < p.y,
        p.y < target.top_right.y,
    ) {
        (true, true, true, true) => ProbeStatus::Hit,
        (true, false, _, _) => ProbeStatus::Past,
        (_, _, false, true) => ProbeStatus::Past,
        _ => ProbeStatus::Before,
    }
}

fn trajectory_height(mut p: Point2D<i32>, mut v: Velocity, t: &TargetArea) -> i32 {
    let mut max_height = p.y;
    loop {
        p.x += v.dx;
        p.y += v.dy;
        max_height = max(max_height, p.y);
        if v.dx > 0 {
            v.dx -= 1;
        } else if v.dx < 0 {
            v.dx += 1;
        }
        v.dy -= 1;
        match in_target(&p, t) {
            ProbeStatus::Before => (),
            ProbeStatus::Hit => return max_height,
            ProbeStatus::Past => return 0,
        }
    }
}

pub fn solve(input_path: &str) {
    let p = Point2D { x: 0, y: 0 };
    let t = TargetArea {
        bottom_left: Point2D { x: 0, y: 0 },
        top_right: Point2D { x: 10, y: 10 },
    };
    let max_height: i32 = (0..100)
        .map(|dx: i32| (0..100).map(move |dy: i32| trajectory_height(p, Velocity { dx, dy }, &t)))
        .flatten()
        .max()
        .unwrap();
    println!("Max Height: {}", max_height);
}
