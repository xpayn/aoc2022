use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default, Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Default, Debug)]
pub struct HeightMap {
    start: Position,
    end: Position,
    heights: Vec<Vec<u8>>,
}

impl HeightMap {
    fn get_height(&self, pos: &Position) -> u8 {
        self.heights[pos.x][pos.y]
    }

    fn find_candidates(&self, current: &Position) -> Vec<Position> {
        let mut ret = vec![];
        let dim_x = self.heights.len() - 1;
        let dim_y = self.heights[0].len() - 1;
        let cur_height = self.get_height(current);

        if 0 < current.x {
            let candidate = Position { x: current.x - 1, y: current.y };
            let c_height = self.get_height(&candidate);
            if c_height <= cur_height + 1 {
                ret.push(candidate)
            }
        }
        if current.x < dim_x {
            let candidate = Position { x: current.x + 1, y: current.y };
            let c_height = self.get_height(&candidate);
            if c_height <= cur_height + 1 {
                ret.push(candidate)
            }
        }
        if 0 < current.y {
            let candidate = Position { x: current.x, y: current.y - 1 };
            let c_height = self.get_height(&candidate);
            if c_height <= cur_height + 1 {
                ret.push(candidate)
            }
        }
        if current.y < dim_y {
            let candidate = Position { x: current.x, y: current.y + 1 };
            let c_height = self.get_height(&candidate);
            if c_height <= cur_height + 1 {
                ret.push(candidate)
            }
        }
        ret
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> HeightMap {
    let mut height_map = HeightMap::default();
    input.lines().enumerate().for_each(|(x, l)| {
        height_map.heights.push(l.trim()
            .chars()
            .enumerate()
            .map(|(y, c)| {
                match c {
                    'S' => {
                        height_map.start = Position { x, y };
                        0
                    }
                    'E' => {
                        height_map.end = Position { x, y };
                        25
                    }
                    _ => c as u8 - 'a' as u8
                }
            }).collect())
    });
    height_map
}

fn a_star(height_map: &HeightMap, current: &Position, end: &Position, cost: usize, acc: &mut [Vec<usize>]) -> usize {
    if cost >= acc[current.x][current.y] {
        return usize::MAX;
    }

    acc[current.x][current.y] = cost;

    if *current == *end {
        return cost;
    }

    height_map.find_candidates(current).iter().map(|c| {
        a_star(height_map, &c, end, cost + 1, acc)
    }).min().unwrap_or(usize::MAX)
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &HeightMap) -> usize {
    let mut acc = vec![vec![usize::MAX; input.heights[0].len()]; input.heights.len()];
    a_star(input, &input.start, &input.end, 0, &mut acc)
}


#[aoc(day12, part2)]
pub fn solve_part2(input: &HeightMap) -> usize {
    input.heights.iter().enumerate().map(|(x, line)| {
        let mut acc = vec![vec![usize::MAX; input.heights[0].len()]; input.heights.len()];
        line.iter().enumerate()
            .filter(|(_, h)| **h == 0)
            .map(|(y, _)| a_star(input, &Position{x, y}, &input.end, 0, &mut acc)).min().unwrap()
    }).min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day12::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 31);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST_INPUT)), 29);
    }
}
