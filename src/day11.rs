use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

impl Operation {
    fn apply_with_divisor(&self, x: usize, denominator: usize, divisor: usize) -> (usize, bool) {
        let ret = match self {
            Operation::Add(y) => (x + y) / divisor,
            Operation::Mul(y) => (x * y) / divisor,
            Operation::Square => x * x / divisor,
        };
        (ret, ret % denominator == 0)
    }

    fn apply(&self, x: usize, denominator: usize, denominator_product: usize) -> (usize, bool) {
        let x_ = x % denominator_product;
        let ret = match self {
            Operation::Add(y) => x_ + y,
            Operation::Mul(y) => x_ * y,
            Operation::Square => x_ * x_,
        };
        (ret, ret % denominator == 0)
    }
}

#[derive(Clone)]
pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    denominator: usize,
    outcome: [usize; 2],
}

impl Monkey {
    fn inspect_items(&self, items: Vec<usize>, worry_divisor: usize, denominator_product: usize) -> Vec<(usize, usize)> {
        items.iter().map(|item| {
            let (worry_level, outcome) = if worry_divisor != 1 {
                self.operation.apply_with_divisor(*item, self.denominator, worry_divisor)
            } else {
                self.operation.apply(*item, self.denominator, denominator_product)
            };
            let monkey_id = self.outcome[outcome as usize];
            (monkey_id, worry_level)
        }).collect()
    }
}


#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Monkey> {
    input.lines().collect::<Vec<&str>>().chunks(7).map(|bloc| {
        let split: Vec<&str> = bloc[1].split(": ").collect();
        let items = split[1]
            .split(", ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<usize>>();


        let operation = if bloc[2].ends_with("old * old") {
            Operation::Square
        } else {
            let split: Vec<&str> = bloc[2].split(" = ").collect();
            let op: Vec<&str> = split[1].split(" ").collect();
            if op[1] == "*" {
                Operation::Mul(op[2].parse().unwrap())
            } else {
                Operation::Add(op[2].parse().unwrap())
            }
        };

        let denominator = bloc[3].split(" ").collect::<Vec<&str>>().pop().unwrap().parse().unwrap();
        let if_true = bloc[4].split(" ").collect::<Vec<&str>>().pop().unwrap().parse().unwrap();
        let if_false = bloc[5].split(" ").collect::<Vec<&str>>().pop().unwrap().parse().unwrap();
        Monkey {
            items,
            operation,
            denominator,
            outcome: [if_false, if_true],
        }
    }).collect()
}

fn solve(monkeys: &[Monkey], round_count: usize, worry_divisor: usize) -> usize {
    let mut counts = vec![0; monkeys.len()];
    let denominator_product = monkeys.iter().map(|m| m.denominator).product::<usize>();
    let mut items = monkeys.iter().map(|m| m.items.clone()).collect::<Vec<Vec<usize>>>();
    for _ in 0..round_count {
        for (i, monkey) in monkeys.iter().enumerate() {
            let ret = monkey.inspect_items(items[i].clone(), worry_divisor, denominator_product);
            counts[i] += items[i].len();
            items[i] = vec![];
            ret.iter().for_each(|(monkey_id, item)| {
                items[*monkey_id].push(*item)
            })
        }
    }
    counts.sort();
    counts.reverse();
    println!("{:?}", counts);
    counts[0] * counts[1]
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Monkey]) -> usize {
    solve(input, 20, 3)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Monkey]) -> usize {
    solve(input, 10000, 1)
}

#[cfg(test)]
mod tests {
    use crate::day11::*;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 10605);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST_INPUT)), 2713310158);
    }
}
