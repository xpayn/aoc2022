use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default, Clone)]
pub struct Forest {
    trees: Vec<Vec<i8>>,
}

impl Forest {
    fn count_visible(&self) -> usize {
        let dim = self.trees.len();
        let mut grid = vec![vec![false; dim]; dim];
        for i in 0..dim {
            let mut left_max = -1;
            self.trees[i].iter().enumerate().for_each(|(j, tree)| {
                if *tree > left_max {
                    left_max = self.trees[i][j];
                    grid[i][j] = true;
                }
            });

            let mut right_max = -1;
            self.trees[i].iter().enumerate().rev().for_each(|(j, tree)| {
                if *tree > right_max {
                    right_max = self.trees[i][j];
                    grid[i][j] = true;
                }
            });


            let mut top_max = -1;
            self.trees.iter().enumerate().for_each(|(j, line)| {
                if line[i] > top_max {
                    top_max = line[i];
                    grid[j][i] = true;
                }
            });

            let mut bottom_max = -1;
            self.trees.iter().enumerate().rev().for_each(|(j, line)| {
                if line[i] > bottom_max {
                    bottom_max = line[i];
                    grid[j][i] = true;
                }
            });
        }
        grid.iter().map(|line| line.iter().filter(|&v| *v).count()).sum()
    }

    fn scenic_score(&self) -> Vec<Vec<usize>> {
        let dim = self.trees.len();
        let mut grid = vec![vec![1usize; dim]; dim];
        for i in 0..dim {
            let mut left_max = -1;
            let mut left_max_idx = 0;
            self.trees[i].iter().enumerate().for_each(|(col, &tree)| {
                let row = i;
                let mut coeff = col - left_max_idx;
                if row == 2 && col == 3 {
                    println!("{} {} {}", tree, left_max, left_max_idx);
                }
                if tree > left_max {
                    left_max = tree;
                    left_max_idx = col;
                    coeff = col;
                } else if tree == left_max {
                    left_max_idx = col;
                }
                grid[row][col] *= coeff;
                println!("{},{} {} left", row, col, coeff);
            });

            let mut right_max = -1;
            let mut right_max_idx = dim - 1;
            self.trees[i].iter().enumerate().rev().for_each(|(col, &tree)| {
                let row = i;
                let mut coeff = right_max_idx - col;
                if tree > right_max {
                    right_max = tree;
                    right_max_idx = col;
                    coeff = dim - 1 - col;
                } else if tree == right_max {
                    right_max_idx = col;
                }
                grid[row][col] *= coeff;
                println!("{},{} {} right", row, col, coeff);
            });


            let mut top_max = -1;
            let mut top_max_idx = 0;
            self.trees.iter().enumerate().for_each(|(row, line)| {
                let col = i;
                let tree = line[col];
                let mut coeff = row - top_max_idx;
                if tree > top_max {
                    top_max = tree;
                    top_max_idx = row;
                    coeff = row;
                } else if tree == top_max {
                    top_max_idx = row;
                }
                grid[row][col] *= coeff;
                println!("{},{} {} up", row, col, coeff);
            });

            let mut bottom_max = -1;
            let mut bottom_max_idx = dim - 1;
            self.trees.iter().enumerate().rev().for_each(|(row, line)| {
                let col = i;
                let tree = line[col];
                let mut coeff = bottom_max_idx - row;
                if tree > bottom_max {
                    bottom_max = tree;
                    bottom_max_idx = row;
                    coeff = dim - 1 - row;
                } else if tree == bottom_max {
                    bottom_max_idx = row;
                }
                grid[row][col] *= coeff;
                println!("{},{} {} down", row, col, coeff);
            });
        }
        println!("AAAAAAA {:?}", self.trees);
        println!("AAAAAAA {:?}", grid);
        grid
    }
}


#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Forest {
    let mut forest = Forest::default();
    input.lines().for_each(|l| {
        forest.trees.push(l.trim()
            .chars()
            .map(|c| {
                c as i8 - '0' as i8
            }).collect())
    });
    forest
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Forest) -> usize {
    input.count_visible()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Forest) -> usize {
    let grid = input.scenic_score();
    *grid.iter().map(|line| line.iter().max().unwrap()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day8::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 21);
    }

    #[test]
    fn grid() {
        let grid = &input_generator(TEST_INPUT).clone().scenic_score();
        let dim = grid[0].len();
        for i in 0..dim {
            assert_eq!(grid[i][0], 0);
            assert_eq!(grid[0][i], 0);
            assert_eq!(grid[i][dim - 1], 0);
            assert_eq!(grid[dim - 1][i], 0);
        }
        assert_eq!(grid[1][1], 1);
        assert_eq!(grid[1][2], 4);
        assert_eq!(grid[1][2], 4);
        assert_eq!(grid[3][2], 8);
        assert_eq!(grid[2][3], 2);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST_INPUT)), 8);
    }
}
