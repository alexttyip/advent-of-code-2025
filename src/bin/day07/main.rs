#![feature(test)]
use std::time::Instant;

fn part1() -> u64 {
    let file = include_str!("input.txt");

    let mut prev_row = [false; 150];

    let pos = file.find('S').unwrap();
    prev_row[pos] = true;

    let mut ans = 0;

    for line in file.lines().skip(1) {
        let bytes = line.as_bytes();
        let mut curr_row = [false; 150];

        for i in prev_row
            .iter()
            .enumerate()
            .filter_map(|(i, x)| x.then_some(i))
        {
            if bytes[i] == '^' as u8 {
                ans += 1;
                curr_row[i - 1] = true;
                curr_row[i + 1] = true;

                continue;
            }

            curr_row[i] = true;
        }

        prev_row = curr_row;
    }

    ans
}

fn part2() -> u64 {
    let file = include_str!("input.txt");

    let mut prev_row = [0; 150];

    let pos = file.find('S').unwrap();
    prev_row[pos] = 1;

    for line in file.lines().skip(1) {
        let bytes = line.as_bytes();
        let mut curr_row = [0; 150];

        for (i, x) in prev_row.iter().enumerate().filter(|(_, x)| x > &&0) {
            if bytes[i] == '^' as u8 {
                curr_row[i - 1] += x;
                curr_row[i + 1] += x;

                continue;
            }

            curr_row[i] += x;
        }

        prev_row = curr_row;
    }

    prev_row.iter().sum()
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 07 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 1656);
    assert_eq!(part2, 76624086587804);
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
