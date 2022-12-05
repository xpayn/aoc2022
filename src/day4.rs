use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
struct Range {
    pub start: u8,
    pub end: u8,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.start
            || self.start <= other.end && self.end >= other.end
            || self.contains(other)
            || other.contains(self)
    }
}

pub struct Pair(Range, Range);

impl Pair {
    fn has_fully_overlapping_range(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }
    fn has_overlapping_range(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Pair> {
    let mut ret = vec![];
    for l in input.lines() {
        let s = l.trim();

        let ranges: Vec<Range> = s.split(",").map(|r| {
            let bounds: Vec<&str> = r.split("-").collect();
            Range { start: bounds[0].parse().unwrap(), end: bounds[1].parse().unwrap() }
        }).collect();

        ret.push(Pair(ranges[0], ranges[1]));
    }
    return ret;
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Pair]) -> usize {
    input.iter().filter(|p| p.has_fully_overlapping_range()).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Pair]) -> usize {
    input.iter().filter(|p| p.has_overlapping_range()).count()
}


#[cfg(test)]
mod tests {
    use crate::day4::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn contains() {
        assert!(Range { start: 1, end: 3 }.contains(&Range { start: 1, end: 3 }));
        assert!(Range { start: 1, end: 3 }.contains(&Range { start: 1, end: 2 }));
        assert_eq!(Range { start: 1, end: 2 }.contains(&Range { start: 1, end: 3 }), false);
    }

    #[test]
    fn overlaps() {
        assert!(Range { start: 1, end: 3 }.overlaps(&Range { start: 1, end: 3 }));
        assert!(Range { start: 1, end: 3 }.overlaps(&Range { start: 0, end: 2 }));
        assert!(Range { start: 1, end: 3 }.overlaps(&Range { start: 2, end: 4 }));
        assert!(Range { start: 1, end: 3 }.overlaps(&Range { start: 3, end: 4 }));
        assert!(Range { start: 1, end: 1 }.overlaps(&Range { start: 1, end: 1 }));
        assert_eq!(Range { start: 1, end: 2 }.overlaps(&Range { start: 3, end: 4 }), false);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 2)
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST_INPUT)), 4)
    }
}
