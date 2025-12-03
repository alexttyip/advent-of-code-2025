#![feature(test)]

use num::{Integer, ToPrimitive};
use std::iter::once;
use std::time::Instant;

// Right-most digit is idx 0
fn remove_digit_at(n: u64, idx: u32) -> u64 {
    let mask = 10u64.pow(idx);
    let (div, rem) = n.div_rem(&mask);

    (div / 10) * mask + rem
}

fn solve(batteries: u32) -> u64 {
    include_str!("input.txt")
        .lines()
        .map(|line| line.split_at(batteries as usize))
        .map(|(left, right)| {
            right
                .chars()
                .flat_map(|c| c.to_digit(10))
                .flat_map(|d| d.to_u64())
                .fold(left.parse::<u64>().unwrap(), |acc, new_digit| {
                    (0..batteries)
                        .map(|i| remove_digit_at(acc, i) * 10 + new_digit)
                        .chain(once(acc))
                        .max()
                        .unwrap_or(acc)
                })
        })
        .sum()
}

fn part1() -> u64 {
    solve(2)
}

fn part2() -> u64 {
    solve(12)
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 03 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 16993);
    assert_eq!(part2, 168617068915447);
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
