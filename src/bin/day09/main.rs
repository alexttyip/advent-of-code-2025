#![feature(test)]

use itertools::Itertools;
use std::cmp::{max, min};
use std::time::Instant;

fn parse_line(line: &str) -> Option<(u64, u64)> {
    let (x, y) = line.split_once(',').unwrap();
    Some((x.parse::<u64>().ok()?, y.parse::<u64>().ok()?))
}

fn part1() -> u64 {
    include_str!("input.txt")
        .lines()
        .flat_map(parse_line)
        .combinations(2)
        .fold(0, |acc, curr| {
            let (x1, y1) = curr[0];
            let (x2, y2) = curr[1];

            acc.max((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        })
}

#[derive(Eq, Hash, Debug, PartialEq)]
enum Line {
    Horizontal { x1: u64, x2: u64, y: u64 },
    Vertical { x: u64, y1: u64, y2: u64 },
}

fn normalise_points(p1: &(u64, u64), p2: &(u64, u64)) -> ((u64, u64), (u64, u64)) {
    (
        (min(p1.0, p2.0), min(p1.1, p2.1)),
        (max(p1.0, p2.0), max(p1.1, p2.1)),
    )
}

fn part2() -> u64 {
    let file = include_str!("input.txt");

    let mut points = vec![];
    let mut edges = vec![];

    for p1 in file
        .lines()
        .chain(file.lines().take(1))
        .flat_map(parse_line)
    {
        if let Some(&p2) = points.last() {
            let ((x1, y1), (x2, y2)) = normalise_points(&p1, &p2);

            edges.push(if x1 == x2 {
                Line::Vertical { x: x1, y1, y2 }
            } else {
                Line::Horizontal { y: y1, x1, x2 }
            });
        };

        points.push(p1);
    }

    points
        .iter()
        .combinations(2)
        .map(|points| {
            let (p1, p2) = normalise_points(points[0], points[1]);

            ((p2.0 - p1.0 + 1) * (p2.1 - p1.1 + 1), p1, p2)
        })
        .sorted_unstable()
        .rev()
        .find(|(_, top_left, bottom_right)| {
            edges.iter().all(|edge| match *edge {
                Line::Horizontal { x1, x2, y } => {
                    y <= top_left.1
                        || bottom_right.1 <= y
                        || x2 <= top_left.0
                        || bottom_right.0 <= x1
                }
                Line::Vertical { x, y1, y2 } => {
                    x <= top_left.0
                        || bottom_right.0 <= x
                        || y2 <= top_left.1
                        || bottom_right.1 <= y1
                }
            })
        })
        .unwrap()
        .0
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 09 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 4737096935);
    assert_eq!(part2, 1644094530);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
