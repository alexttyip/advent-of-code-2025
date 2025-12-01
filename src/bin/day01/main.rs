#![feature(test)]

use num::Integer;
use std::ops::Neg;
use std::time::Instant;

fn read_and_parse_lines() -> impl Iterator<Item = i32> {
    include_str!("input.txt").lines().flat_map(|line| {
        line[1..]
            .parse::<i32>()
            .map(|mag| if &line[0..1] == "L" { mag.neg() } else { mag })
    })
}

fn part1() -> usize {
    let mut pos = 50;

    read_and_parse_lines()
        .filter(|step| {
            pos += step;
            pos %= 100;

            pos == 0
        })
        .count()
}

fn part2() -> i32 {
    let mut pos = 50;

    read_and_parse_lines()
        .map(|step| {
            let (div, rem) = (pos + step).div_rem(&100);

            let mut count = div.abs();

            if step < 0 && rem == 0 {
                count += 1;
            }

            if pos > 0 && rem < 0 {
                count += 1;
            }

            pos = rem + 100;
            pos %= 100;

            count
        })
        .sum()
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 01 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 1165);
    assert_eq!(part2, 6496);
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
