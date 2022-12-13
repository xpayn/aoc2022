use std::cmp::Ordering;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;
use nom::combinator::{all_consuming, map, map_res};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::{Finish, IResult};
use nom::multi::separated_list0;
use nom::sequence::delimited;

#[derive(Clone, Eq, PartialEq)]
pub enum Data {
    Integer(u8),
    List(Vec<Data>),
}

impl PartialOrd<Self> for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Data::Integer(a) => {
                match other {
                    Data::Integer(b) => { a.cmp(b) }
                    Data::List(_) => { Data::List(vec![self.clone()]).cmp(other) }
                }
            }
            Data::List(a) => {
                match other {
                    Data::Integer(_) => { self.cmp(&Data::List(vec![other.clone()])) }
                    Data::List(b) => {
                        for (i, j) in a.iter().zip(b.iter()) {
                            let ret = i.cmp(j);
                            if ret != Ordering::Equal {
                                return ret;
                            }
                        }
                        if a.len() < b.len() {
                            Ordering::Less
                        } else if a.len() > b.len() {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    }
                }
            }
        }
    }
}

fn parse_number(i: &str) -> IResult<&str, Data> {
    map_res(digit1, |s: &str| s.parse::<u8>().map(|i| Data::Integer(i)))(i)
}

fn parse_list(i: &str) -> IResult<&str, Data> {
    delimited(
        tag("["),
        map(separated_list0(
            tag(","),
            alt((
                parse_number,
                parse_list
            ))), |s| Data::List(s)),
        tag("]"),
    )(i)
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<[Data; 2]> {
    input.lines().collect::<Vec<&str>>().chunks(3).map(|bloc| {
        let (_, fst) = all_consuming(parse_list)(bloc[0]).finish().unwrap();
        let (_, sec) = all_consuming(parse_list)(bloc[1]).finish().unwrap();
        [fst, sec]
    }).collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[[Data; 2]]) -> usize {
    input.iter()
        .enumerate()
        .filter(|(_, a)| a[0] <= a[1])
        .map(|(i, _)| i + 1).sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[[Data; 2]]) -> usize {
    let dividers_str = "[[2]]
[[6]]
";
    let dividers = &input_generator(dividers_str)[0];
    let mut flattened = input.iter()
        .flatten()
        .collect::<Vec<&Data>>();
    flattened.extend(dividers.iter());
    flattened.sort();
    flattened.iter().enumerate()
        .filter(|(_, p)| ***p == dividers[0] || ***p == dividers[1])
        .fold(1, |acc, (i, _)| acc * (i + 1))
}

#[cfg(test)]
mod tests {
    use crate::day13::*;

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST_INPUT)), 140);
    }
}
