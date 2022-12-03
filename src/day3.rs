use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Item {
    label: char,
}

impl Item {
    fn priority(&self) -> u32 {
        let c = self.label as u32;
        if c < 91 {
            c - 64 + 26
        } else {
            c - 96
        }
    }
}

#[derive(Debug)]
pub struct Bag {
    compartment1: Vec<Item>,
    compartment2: Vec<Item>,
}

impl Bag {
    fn find_duplicate(&self) -> Item {
        let set: HashSet<Item> = HashSet::from_iter(self.compartment1.iter().cloned());
        self.compartment2.iter().find(|i| set.contains(i)).unwrap().clone()
    }

    fn to_set(&self) -> HashSet<Item> {
        let mut ret = HashSet::with_capacity(self.compartment1.len() * 2 - 1);
        self.compartment1.iter().for_each(|i| {
            ret.insert(i.clone());
        });
        self.compartment2.iter().for_each(|i| {
            ret.insert(i.clone());
        });
        ret
    }
}

impl From<&str> for Bag {
    fn from(input: &str) -> Self {
        let mut ret = Self {
            compartment1: Vec::with_capacity(input.len() / 2),
            compartment2: Vec::with_capacity(input.len() / 2),
        };
        input.chars().take(input.len() / 2).for_each(|c| ret.compartment1.push(Item { label: c }));
        input.chars().skip(input.len() / 2).for_each(|c| ret.compartment2.push(Item { label: c }));
        ret
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Bag> {
    input.lines().map(|l| l.trim().into()).collect::<Vec<Bag>>()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Bag]) -> u32 {
    input.iter().map(|b| b.find_duplicate().priority()).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Bag]) -> u32 {
    input.chunks(3).map(|g| {
        let mut set: HashSet<Item> = g[0].to_set();
        g.iter().skip(1).for_each(|b| {
            set = HashSet::from_iter(
                set.intersection(&b.to_set())
                    .map(|r| r.clone())
            );
            //println!("{} {:?}", set.len(), set)
        });
        set.iter().nth(0).unwrap().priority()
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn priorities() {
        assert_eq!(Item { label: 'a' }.priority(), 1);
        assert_eq!(Item { label: 'A' }.priority(), 27);
    }

    #[test]
    fn bag() {
        let b: Bag = "Aa".into();
        assert_eq!(b.compartment1[0].label, 'A');
        assert_eq!(b.compartment2[0].label, 'a');
    }

    #[test]
    fn set() {
        let b: Bag = "Aa".into();
        let s = b.to_set();
        assert!(s.contains(&Item { label: 'A' }));
        assert!(s.contains(&Item { label: 'a' }));
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 157)
    }

    # [test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST_INPUT)), 70)
    }
}