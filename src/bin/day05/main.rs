#![feature(test)]

use std::cmp::Ordering;
use std::time::Instant;

fn combine_ranges(range_strings: &str) -> Vec<(u64, u64)> {
    let mut ranges = vec![];

    for line in range_strings.lines() {
        let (from, to) = line.split_once('-').unwrap();

        ranges.push((from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap()));
    }

    ranges.sort_unstable();

    let mut pointer = 1;

    while pointer < ranges.len() {
        if ranges[pointer].0 <= ranges[pointer - 1].1 {
            let popped = ranges.remove(pointer);

            if popped.1 > ranges[pointer - 1].1 {
                ranges[pointer - 1].1 = popped.1
            }
        } else {
            pointer += 1;
        }
    }

    ranges
}

fn part1() -> u32 {
    let (range_strings, ids) = include_str!("input.txt").split_once("\n\n").unwrap();

    let ranges = combine_ranges(range_strings);

    let mut ans = 0;

    for id in ids.lines() {
        let id = id.parse::<u64>().unwrap();

        if ranges
            .binary_search_by(|(from, to)| {
                if from <= &id && &id <= to {
                    return Ordering::Equal;
                }

                if &id < from {
                    return Ordering::Greater;
                }

                return Ordering::Less;
            })
            .is_ok()
        {
            ans += 1;
        }
    }

    ans
}

fn part2() -> u64 {
    let (range_strings, _) = include_str!("input.txt").split_once("\n\n").unwrap();

    combine_ranges(range_strings)
        .iter()
        .fold(0, |acc, (from, to)| acc + to - from + 1)
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 05 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 601);
    assert_eq!(part2, 367899984917516);
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
