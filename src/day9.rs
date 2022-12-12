use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("SHOULD NOT HAPPEN")
        }
    }
}

pub struct Instruction {
    direction: Direction,
    steps: u8,
}

#[derive(Default, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Position {
    fn go(&mut self, d: Direction) {
        match d {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }

    fn follow(&mut self, head: &Self) {
        let d_y = head.y - self.y;
        let d_x = head.x - self.x;
        if self.x == head.x {
            if d_y.abs() > 1 {
                self.y += d_y / d_y.abs()
            }
        } else if self.y == head.y {
            if d_x.abs() > 1 {
                self.x += d_x / d_x.abs()
            }
        } else {
            if d_x.abs() > 1 || d_y.abs() > 1 {
                self.x += d_x / d_x.abs();
                self.y += d_y / d_y.abs()
            }
        }
    }
}

struct Rope {
    nodes: Vec<RefCell<Position>>,
    tracker: HashSet<Position>,
}

impl Rope {
    fn new(size: usize) -> Self {
        Self {
            nodes: vec![RefCell::new(Position::default()); size],
            tracker: HashSet::new(),
        }
    }

    fn apply_instructions(&mut self, intructions: &[Instruction]) {
        for inst in intructions {
            for _ in 0..inst.steps {
                self.nodes.windows(2).enumerate().for_each(|(i, positions)| {
                    if i == 0 {
                        positions[0].borrow_mut().go(inst.direction)
                    }
                    positions[1].borrow_mut().follow(positions[0].borrow().deref());
                });
                self.tracker.insert(self.nodes.iter().last().unwrap().borrow().clone());
            }
        }
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| {
        let split: Vec<&str> = l.trim().split(" ").collect();
        Instruction { direction: split[0].into(), steps: split[1].parse().unwrap() }
    }).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(_input: &[Instruction]) -> usize {
    let mut rope = Rope::new(2);
    rope.apply_instructions(_input);
    rope.tracker.len()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Instruction]) -> usize {
    let mut rope = Rope::new(10);
    rope.apply_instructions(input);
    rope.tracker.len()
}

#[cfg(test)]
mod tests {
    use crate::day9::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST_INPUT)), 1);
        assert_eq!(solve_part2(&input_generator(TEST_INPUT2)), 36);
    }
}
