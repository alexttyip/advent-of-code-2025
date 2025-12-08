#![feature(test)]

use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::mem::swap;
use std::time::Instant;

fn get_distances() -> BinaryHeap<(Reverse<u64>, ((u64, u64, u64), (u64, u64, u64)))> {
    include_str!("input.txt")
        .lines()
        .map(|line| -> (_, _, _) {
            line.split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .combinations(2)
        .map(|comb| {
            let point1 = comb[0];
            let point2 = comb[1];

            let dist = point1.0.abs_diff(point2.0).pow(2)
                + point1.1.abs_diff(point2.1).pow(2)
                + point1.2.abs_diff(point2.2).pow(2);

            (Reverse(dist), (point1, point2))
        })
        .collect()
}

fn make_connection(
    connections: &mut Vec<Vec<(u64, u64, u64)>>,
    point1: (u64, u64, u64),
    point2: (u64, u64, u64),
) {
    let p1_cs = connections.iter().position(|cs| cs.contains(&point1));
    let p2_cs = connections.iter().position(|cs| cs.contains(&point2));

    match (p1_cs, p2_cs) {
        (Some(mut p1_cs), Some(mut p2_cs)) => {
            if p1_cs != p2_cs {
                if p1_cs > p2_cs {
                    swap(&mut p1_cs, &mut p2_cs);
                }

                let p2 = connections.remove(p2_cs);

                connections.get_mut(p1_cs).unwrap().extend(p2);
            }
        }
        (Some(p1_cs), None) => {
            connections.get_mut(p1_cs).unwrap().push(point2);
        }
        (None, Some(p2_cs)) => {
            connections.get_mut(p2_cs).unwrap().push(point1);
        }
        _ => {
            connections.push(vec![point1, point2]);
        }
    }
}

fn part1() -> u64 {
    let mut distances = get_distances();
    let mut connections: Vec<Vec<(_, _, _)>> = vec![];

    for _ in 0..1000 {
        let (_, (point1, point2)) = distances.pop().unwrap();
        make_connection(&mut connections, point1, point2);
    }

    connections
        .iter()
        .map(|v| v.len())
        .sorted_unstable()
        .rev()
        .take(3)
        .product::<usize>() as u64
}

fn part2() -> u64 {
    let mut distances = get_distances();
    let mut connections: Vec<Vec<(_, _, _)>> = vec![];

    while let Some((_, (point1, point2))) = distances.pop() {
        make_connection(&mut connections, point1, point2);

        if connections.iter().map(|v| v.len()).any(|len| len == 1000) {
            return point1.0 * point2.0;
        }
    }

    panic!();
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

    assert_eq!(part1, 32103);
    assert_eq!(part2, 8133642976);
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
