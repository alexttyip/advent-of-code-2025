#![feature(test)]

use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

fn parse_line(line: &str) -> (usize, Vec<Vec<usize>>, Vec<usize>) {
    let mut words = line.split_whitespace();

    let mut diagram = words.next().unwrap();
    diagram = &diagram[1..diagram.len() - 1];

    let target_diagram = diagram
        .chars()
        .rev()
        .fold(0, |acc, curr| acc * 2 + (curr == '#') as usize);

    let raw_joltage = words.next_back().unwrap();
    let joltage = raw_joltage[1..raw_joltage.len() - 1]
        .split(',')
        .flat_map(|raw_int| raw_int.parse::<usize>())
        .collect_vec();

    let buttons = words
        .map(|raw_button| {
            raw_button[1..raw_button.len() - 1]
                .split(',')
                .flat_map(|raw_int| raw_int.parse::<usize>())
                .collect_vec()
        })
        .collect_vec();

    (target_diagram, buttons, joltage)
}
fn part1() -> usize {
    let lines = include_str!("input.txt").lines().map(parse_line);

    let mut ans = 0;
    'line: for (target_diagram, buttons, _) in lines {
        let buttons = buttons
            .iter()
            .map(|button| button.iter().map(|i| 2u32.pow(*i as u32) as usize).sum())
            .collect_vec();

        for i in 1..=buttons.len() {
            for combination in buttons.iter().combinations(i) {
                let mut state = 0;

                for button in combination {
                    state ^= button;
                    // dbg!(state);
                }

                if state == target_diagram {
                    ans += i;
                    continue 'line;
                }
            }
        }
    }

    ans
}

fn solve_joltage(
    buttons: &Vec<Vec<usize>>,
    target_joltage: &Vec<usize>,
    queue: &mut BinaryHeap<(Reverse<usize>, Vec<usize>)>,
    seen: &mut HashSet<Vec<usize>>,
) -> usize {
    while let Some((Reverse(cost), presses)) = queue.pop() {
        if !seen.insert(presses.clone()) {
            continue;
        }

        let curr_joltage =
            presses
                .iter()
                .zip(buttons)
                .fold(vec![0; target_joltage.len()], |mut acc, (press, button)| {
                    for i in button {
                        acc[*i] += press;
                    }

                    acc
                });

        let mut correct = true;
        let mut should_check_next = true;

        for (curr, target) in curr_joltage.iter().zip(target_joltage) {
            if curr == target {
                continue;
            }

            if curr > target {
                correct = false;
                should_check_next = false;
                break;
            }

            if curr < target {
                correct = false;
            }
        }

        if correct {
            return presses.iter().sum();
        }

        if !should_check_next {
            continue;
        }

        let new_cost = cost + 1;
        for i in 0..presses.len() {
            let mut new_presses = presses.clone();
            new_presses[i] += 1;
            queue.push((Reverse(new_cost), new_presses));
        }
    }

    panic!()
}

fn part2() -> usize {
    let lines = include_str!("input.txt").lines().map(parse_line);

    let mut ans = 0;
    for (_, buttons, joltage) in lines {
        let mut counts = vec![0;joltage.len()];

        dbg!(buttons.iter().flatten().counts());



        // dbg!(ans);
        // let mut queue = BinaryHeap::from_iter((0..buttons.len()).map(|i| {
        //     let mut v = vec![0; buttons.len()];
        //     v[i] = 1;
        //     (Reverse(1), v)
        // }));
        // let mut seen = HashSet::new();
        //
        // ans += solve_joltage(&buttons, &joltage, &mut queue, &mut seen);

        /*        dbg!(ans);
                for i in 1.. {
                    'comb: for combination in buttons.iter().combinations_with_replacement(i) {
                        let mut state = vec![0; joltage.len()];

                        for &i in combination.iter().flat_map(|comb| *comb) {
                            let d = state.get_mut(i as usize).unwrap();
                            *d += 1;

                            if *d > joltage[i as usize] {
                                continue 'comb;
                            }
                        }

                        if state == joltage {
                            ans += i;
                            continue 'line;
                        }
                    }
                }
        */
    }

    ans
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 10 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    // assert_eq!(part1, 0);
    // assert_eq!(part2, 0);
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
