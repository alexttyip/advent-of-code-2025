#![feature(test)]

use itertools::{multizip, Itertools};
use std::time::Instant;

fn solve(ops_line: &str, nums: Vec<Vec<u64>>) -> u64 {
    ops_line
        .split_whitespace()
        .zip(nums)
        .map(|(op, nums)| match op {
            "+" => nums.iter().sum::<u64>(),
            "*" => nums.iter().product(),
            _ => panic!(),
        })
        .sum()
}

fn part1() -> u64 {
    let file = include_str!("input.txt");

    let nums_in_column: (_, _, _, _) = file
        .lines()
        .take(4)
        .map(|line| line.split_whitespace().flat_map(|s| s.parse::<u64>()))
        .collect_tuple()
        .unwrap();

    let nums = multizip(nums_in_column)
        .map(|nums| -> [_; _] { nums.into() })
        .map(|nums| -> Vec<_> { nums.into() })
        .collect_vec();

    solve(file.lines().nth(4).unwrap(), nums)
}

fn part2() -> u64 {
    let file = include_str!("input.txt");

    let chars_in_column: (_, _, _, _) = file
        .lines()
        .take(4)
        .map(|line| line.chars())
        .collect_tuple()
        .unwrap();

    let nums = multizip(chars_in_column)
        .map(|chars| -> [_; _] { chars.into() })
        .fold(vec![vec![]; 1], |mut acc, curr| {
            match curr.iter().join("").trim().parse::<u64>() {
                Ok(num) => acc.last_mut().unwrap().push(num),
                Err(_) => acc.push(vec![]),
            }

            acc
        });

    solve(file.lines().nth(4).unwrap(), nums)
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 06 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 4387670995909);
    assert_eq!(part2, 9625320374409);
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
