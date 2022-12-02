use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Eq, PartialEq, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Clone, Debug)]
enum Outcome {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl Hand {
    fn fight(&self, o: &Self) -> Outcome {
        if *o == self.draw_against() {
            Outcome::Draw
        } else if *o == self.win_against() {
            Outcome::Win
        } else {
            Outcome::Loose
        }
    }

    fn win_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissor,
            Hand::Scissor => Hand::Paper,
            Hand::Paper => Hand::Rock
        }
    }

    fn loose_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Scissor => Hand::Rock,
            Hand::Paper => Hand::Scissor
        }
    }

    fn draw_against(&self) -> Hand {
        self.clone()
    }

    fn score(&self) -> u32 {
        self.clone() as u32
    }
}

impl From<char> for Hand {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissor,
            _ => unreachable!()
        }
    }
}

impl Outcome {
    fn score(&self) -> u32 {
        self.clone() as u32
    }

    fn complementary_hand(&self, h: &Hand) -> Hand {
        match self {
            Outcome::Draw => h.draw_against(),
            Outcome::Win => h.loose_against(),
            _ => h.win_against()
        }
    }
}

impl From<char> for Outcome {
    fn from(desired_outcome: char) -> Self {
        match desired_outcome {
            'X' => Outcome::Loose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!()
        }
    }
}

pub struct Game {
    elf: Hand,
    // part1
    me: Hand,
    // part2
    outcome: Outcome,
}

impl Game {
    fn score(&self) -> u32 {
        self.me.fight(&self.elf).score() + self.me.score()
    }

    fn needed_hand(&self) -> Hand {
        self.outcome.complementary_hand(&self.elf)
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(|l| {
        let elf = l.chars().nth(0).unwrap().into();
        let c2 = l.chars().nth(2).unwrap();
        let outcome = Outcome::from(c2);
        Game {
            elf,
            me: c2.into(),
            outcome
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u32 {
    input.iter().map(|g| g.score()).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> u32 {
    input.iter().map(|g| g.outcome.score() + g.needed_hand().score()).sum()
}
