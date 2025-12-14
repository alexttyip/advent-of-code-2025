#![feature(test)]
extern crate core;

use std::collections::HashMap;
use std::time::Instant;

fn parse<'a>() -> HashMap<String, Vec<&'a str>> {
    let mut devices: HashMap<_, _> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut words = line
                .split(|c: char| !c.is_alphabetic())
                .filter(|a| !a.is_empty());

            let from = words.next().unwrap().to_string();
            let to = words.collect();

            (from, to)
        })
        .collect();
    devices.insert("out".to_owned(), vec![]);

    devices
}

fn dfs(
    devices: &HashMap<String, Vec<&str>>,
    curr: String,
    dest: String,
    seen: &mut HashMap<(String, String), u64>,
) -> u64 {
    if curr == dest {
        return 1;
    }

    if let Some(count) = seen.get(&(curr.clone(), dest.clone())) {
        return *count;
    }

    let count = devices
        .get(&curr)
        .unwrap()
        .iter()
        .map(|device| dfs(devices, (*device).to_owned(), dest.clone(), seen))
        .sum();

    seen.insert((curr, dest), count);

    count
}

fn part1() -> u64 {
    let devices = parse();

    dfs(
        &devices,
        "you".to_owned(),
        "out".to_owned(),
        &mut HashMap::new(),
    )
}

fn part2() -> u64 {
    let devices = parse();

    let mut seen = HashMap::new();
    let svr_to_fft = dfs(&devices, "svr".to_owned(), "fft".to_owned(), &mut seen);
    let svr_to_dac = dfs(&devices, "svr".to_owned(), "dac".to_owned(), &mut seen);
    let fft_to_dac = dfs(&devices, "fft".to_owned(), "dac".to_owned(), &mut seen);
    let dac_to_fft = dfs(&devices, "dac".to_owned(), "fft".to_owned(), &mut seen);
    let dac_to_out = dfs(&devices, "dac".to_owned(), "out".to_owned(), &mut seen);
    let fft_to_out = dfs(&devices, "dac".to_owned(), "out".to_owned(), &mut seen);

    svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 11 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 494);
    assert_eq!(part2, 296006754704850);
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
