use aoc_runner_derive::{aoc, aoc_generator};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct Crate(char);

#[derive(Debug)]
pub struct Movement {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Clone)]
pub struct Ship {
    stacks: Vec<Vec<Crate>>,
}

impl Ship {
    fn new() -> Self {
        Ship { stacks: vec![] }
    }

    fn add(&mut self, stack_idx: usize, crate_: Crate) {
        //println!("{} {:?}", stack_idx, crate_);
        if self.stacks.len() <= stack_idx {
            self.stacks.resize(stack_idx + 1, vec![])
        }
        self.stacks[stack_idx].insert(0, crate_);
    }

    fn move_crates(&mut self, movement: &Movement) {
        //println!("{:?} {:?}", self.stacks, movement);
        for _ in 0..movement.quantity {
            let v;
            {
                let src = &mut self.stacks[movement.from];
                v = src.pop().unwrap();
            }
            let dst = &mut self.stacks[movement.to];
            dst.push(v)
        }
    }

    fn move_crates_by_block(&mut self, movement: &Movement) {
        //println!("{:?} {:?}", self.stacks, movement);
        let mut v;
        {
            let src = &mut self.stacks[movement.from];
            v = src.drain(src.len() - movement.quantity..).collect::<Vec<Crate>>();
        }
        let dst = &mut self.stacks[movement.to];
        dst.append(&mut v)
    }

    fn apply_procedure(&mut self, procedure: &[Movement], is_9001: bool) {
        if is_9001 {
            procedure.iter().for_each(|m|
                self.move_crates_by_block(m)
            )
        } else {
            procedure.iter().for_each(|m|
                self.move_crates(m)
            )
        }
    }
}


#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Ship, Vec<Movement>) {
    let mut ship = Ship::new();
    let mut procedure: Vec<Movement> = vec![];
    for l in input.lines() {
        let s = l.trim_end();
        if s.starts_with("[") || s.starts_with("  ") {
            s.chars()
                .enumerate()
                .filter(|(j, c)| j % 4 == 1 && *c != ' ')
                .for_each(|(j, c)| ship.add(j / 4, Crate(c)))
        } else if s.starts_with("m") {
            let split: Vec<&str> = s.split(" ").collect();
            procedure.push(Movement {
                quantity: split[1].parse().unwrap(),
                from: split[3].parse::<usize>().unwrap() - 1,
                to: split[5].parse::<usize>().unwrap() - 1,
            });
        }
    }
    return (ship, procedure);
}

pub fn solve(input: &Ship, procedure: &[Movement], is_9001: bool) -> String {
    let mut ret: String = "".into();
    let mut ship = input.clone();
    ship.apply_procedure(procedure, is_9001);
    ship.stacks.iter().for_each(|s| ret.push(s.iter().last().unwrap().0));
    ret
}

#[aoc(day5, part1)]
pub fn solve_part1((ship, procedure): &(Ship, Vec<Movement>)) -> String {
    solve(ship, procedure, false)
}

#[aoc(day5, part2)]
pub fn solve_part2((ship, procedure): &(Ship, Vec<Movement>)) -> String {
    solve(ship, procedure, true)
}


#[cfg(test)]
mod tests {
    use crate::day5::*;

    const TEST_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST_INPUT)), "CMZ")
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST_INPUT)), "MCD")
    }
}
