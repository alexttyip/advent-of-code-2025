#![feature(test)]

use num::Integer;
use std::time::Instant;

fn read_and_parse() -> impl Iterator<Item = u64> {
    include_str!("input.txt")
        .trim()
        .split(",")
        .flat_map(|range| {
            range
                .split_once("-")
                .and_then(|(from, to)| Some((from.parse::<u64>().ok()?, to.parse::<u64>().ok()?)))
        })
        .flat_map(|(from, to)| from..=to)
}

fn part1() -> u64 {
    read_and_parse()
        .filter(|n| {
            let digits = n.ilog10() + 1;

            let (div, rem) = n.div_rem(&10_u64.pow(digits / 2));

            div == rem
        })
        .sum()
}

fn part2() -> u64 {
    read_and_parse()
        .filter(|n| {
            let digits = n.ilog10() + 1;

            for chunk_width in 1..=(digits / 2) {
                if digits % chunk_width != 0 {
                    continue;
                }

                let mut repeats = true;

                let divisor = &10_u64.pow(chunk_width);

                let (mut div, rem) = n.div_rem(divisor);

                while div > 0 {
                    if rem != div % divisor {
                        repeats = false;
                        break;
                    }

                    div /= divisor;
                }

                if repeats {
                    return true;
                }
            }

            false
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

    println!("--- Day 02 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 52316131093);
    assert_eq!(part2, 69564213293);
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
