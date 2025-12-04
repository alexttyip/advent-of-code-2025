#![feature(test)]

use std::collections::VecDeque;
use std::mem::swap;
use std::time::Instant;

const N: usize = 136;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn solve(part2: bool) -> u16 {
    let file = include_str!("input.txt").lines();

    let mut grid = [[false; N + 1]; N + 1];
    let mut queue = VecDeque::new();

    for (y, line) in file.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                grid[x][y] = true;
                queue.push_back((x, y));
            }
        }
    }

    let mut ans = 0;

    let mut changed = true;
    let mut temp_q = VecDeque::new();

    while changed {
        changed = false;

        while let Some((x, y)) = queue.pop_front() {
            let mut count = 0;

            for (dx, dy) in DIRECTIONS {
                let (Some(xx), Some(yy)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
                else {
                    continue;
                };

                if grid[xx][yy] {
                    count += 1;

                    if count >= 4 {
                        break;
                    }
                }
            }

            if count < 4 {
                ans += 1;
                if part2 {
                    grid[x][y] = false;
                    changed = true;
                }
            } else {
                temp_q.push_back((x, y))
            }
        }

        swap(&mut queue, &mut temp_q);
    }

    ans
}

fn part1() -> u16 {
    solve(false)
}

fn part2() -> u16 {
    solve(true)
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 04 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed); // 10,223,350.00
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 1393);
    assert_eq!(part2, 8643);
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
