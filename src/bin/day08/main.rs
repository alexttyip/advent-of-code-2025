#![feature(test)]

use itertools::Itertools;
use std::array::from_fn;
use std::iter::FlatMap;
use std::time::Instant;

const N: usize = 1000;

fn find(roots: &mut [usize], x: usize) -> usize {
    if roots[x] == x {
        return x;
    }

    roots[x] = find(roots, roots[x]);
    roots[x]
}

fn union(roots: &mut [usize], x: usize, y: usize) {
    let x_root = find(roots, x);
    let y_root = find(roots, y);
    roots[x_root] = y_root;
}

fn part1() -> u64 {
    let mut tuples = include_str!("input.txt").lines().flat_map(|line| {
        line.split(',')
            .flat_map(|s| s.parse::<u64>())
            .collect_tuple::<(_, _, _)>()
    });

    let points: [(_, _, _); N] = from_fn(|_| tuples.next().unwrap());

    let mut roots: [usize; N] = from_fn(|i| i);

    points
        .iter()
        .enumerate()
        .combinations(2)
        .map(|points| {
            let (i1, (x1, y1, z1)) = points[0];
            let (i2, (x2, y2, z2)) = points[1];

            let d = x1.abs_diff(*x2).pow(2) + y1.abs_diff(*y2).pow(2) + z1.abs_diff(*z2).pow(2);

            (d, i1, i2)
        })
        .sorted_unstable()
        .take(1000)
        .for_each(|(_, x, y)| union(&mut roots, x, y));

    let mut sizes = [0; N];

    for x in 0..N {
        sizes[find(&mut roots, x)] += 1;
    }

    sizes.iter().sorted_unstable().rev().take(3).product()
}

fn part2() -> u64 {
    let lines = include_str!("input.txt").lines();

    0
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 08 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    // assert_eq!(part1, 0);
    // assert_eq!(part2, 0);
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
