use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> String {
    input.to_owned()
}

fn all_different(chars: &[u8]) -> bool {
    for (i, x) in chars.iter().enumerate() {
        for y in chars.iter().skip(i+1) {
            if x == y {
                return false
            }
        }
    }
    true
}

fn solve(size: usize, input: &str) -> usize {
    input.as_bytes()
        .windows(size)
        .enumerate()
        .find(|(_, bytes)|
            all_different(bytes)
        ).unwrap().0 + size
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    solve(4, input)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    solve(14, input)
}

#[cfg(test)]
mod tests {
    use crate::day6::*;

    #[test]
    fn all_diff() {
        assert_eq!(all_different(&vec![1, 2, 3, 4]), true);
        assert_eq!(all_different(&vec![1, 2, 4, 4]), false);
        assert_eq!(all_different(&vec![1, 3, 3, 4]), false);
        assert_eq!(all_different(&vec![1, 2, 3, 1]), false);
        assert_eq!(all_different(&vec![1, 4, 3, 4]), false);
    }
    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 7);
        assert_eq!(solve_part1(&input_generator("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(solve_part1(&input_generator("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(solve_part1(&input_generator("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
        assert_eq!(solve_part1(&input_generator("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(solve_part2(&input_generator("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(solve_part2(&input_generator("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(solve_part2(&input_generator("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
        assert_eq!(solve_part2(&input_generator("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
    }
}
