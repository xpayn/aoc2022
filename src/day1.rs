use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let mut ret = vec![vec![]];
    for l in input.lines() {
        let s = l.trim();
        match s {
            "" => ret.push(vec![]),
            _ => ret.last_mut().unwrap().push(s.parse().unwrap())
        }
    }
    return ret;
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|e| e.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> u32 {
    let mut a = input.iter().map(|e| e.iter().sum()).collect::<Vec<u32>>();
    a.sort();
    a[a.len()-3..].iter().sum()
}