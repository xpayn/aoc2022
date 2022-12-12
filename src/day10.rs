use aoc_runner_derive::{aoc, aoc_generator};

pub enum Instruction {
    Addx(i8),
    Noop,
}

#[derive(Clone)]
pub enum Pixel {
    On,
    Off,
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        match self {
            Pixel::On => "#",
            Pixel::Off => "."
        }.to_owned()
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| {
        let split = l.trim().split(" ").collect::<Vec<&str>>();
        match split[0] {
            "addx" => Instruction::Addx(split[1].parse().unwrap()),
            "noop" => Instruction::Noop,
            _ => unreachable!()
        }
    }).collect()
}

fn build_cycles(program: &[Instruction]) -> Vec<isize> {
    let mut cycles = vec![];
    let mut x = 1;
    program.iter().for_each(|inst| {
        cycles.push(x);
        match *inst {
            Instruction::Noop => (),
            Instruction::Addx(i) => {
                cycles.push(x);
                x += i as isize;
            }
        }
    });
    cycles
}

fn print_crt_line(line: &[Pixel]) -> String {
    line.iter()
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn print_crt(crt: &[Vec<Pixel>]) -> String {
    crt.iter()
        .map(|line| print_crt_line(line))
        .collect::<Vec<String>>().join("\n")
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Instruction]) -> isize {
    let cycles = build_cycles(input);
    [20usize, 60, 100, 140, 180, 220].iter().map(|&i| (i as isize) * cycles[i - 1]).sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Instruction]) -> String {
    let mut crt = vec![vec![Pixel::Off; 40]; 6];
    let cycles = build_cycles(input);

    for (i, register) in cycles.iter().enumerate() {
        let sprite_pos = *register as usize % 40;
        let y = i / 40;
        if sprite_pos.abs_diff(i % 40) <= 1 {
            crt[y as usize][i % 40] = Pixel::On
        }
    }

    let ret = print_crt(&crt);
    ret
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    const TEST_INPUT: &str = "noop
addx 3
addx -5
noop";

    const TEST_INPUT2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn cycles() {
        let cycles = build_cycles(&input_generator(TEST_INPUT));
        println!("{:?}", cycles);
        assert_eq!(cycles[0], 1);
        assert_eq!(cycles[1], 1);
        assert_eq!(cycles[2], 1);
        assert_eq!(cycles[3], 4);
        assert_eq!(cycles[4], 4);
        assert_eq!(cycles[5], -1);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST_INPUT2)), 13140);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST_INPUT2)), "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }
}
