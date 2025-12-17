#![feature(test)]

use std::time::Instant;

fn part1() -> usize {
    let file = include_str!("input.txt");

    file.lines()
        .skip(30)
        .filter(|region| {
            let mut nums = region
                .split(|c: char| !c.is_numeric())
                .filter(|a| !a.is_empty())
                .flat_map(|s| s.parse::<usize>());

            let area = nums.next().unwrap() * nums.next().unwrap();
            let max_needed_area = nums.fold(0, |acc, curr| acc + curr * 9);

            area >= max_needed_area
        })
        .count()
}

pub fn main() {
    let now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    println!("--- Day 12 ---");
    println!("Part 1: {}", part1);
    println!("Part 1 took: {:.2?}", part1_elapsed);

    assert_eq!(part1, 443);
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
}
