use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Node {
    name: String,
    size: usize,
    idx: usize,
    parent_idx: usize,
    children_count: usize,
}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    current: usize,
}

impl Tree {
    fn new() -> Self {
        Tree {
            nodes: vec![Node {
                name: "".into(),
                size: 0,
                idx: 0,
                parent_idx: 0,
                children_count: 0
            }],
            current: 0,
        }
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.nodes.push(Node {
            name: name.to_string(),
            size,
            idx: self.nodes.len(),
            parent_idx: self.current,
            children_count: 0
        });
        self.nodes[self.current].children_count += 1;
    }

    fn add_dir(&mut self, name: &str) {
        self.nodes.push(Node {
            name: name.to_string(),
            size: 0,
            idx: self.nodes.len(),
            parent_idx: self.current,
            children_count: 0
        });
        self.nodes[self.current].children_count += 1;
    }

    fn cd(&mut self, path: &str) {
        if path == ".." {
            self.current = self.nodes[self.current].parent_idx
        } else if path == "/" {
            self.current = 0
        } else {
            self.current = self.nodes.iter()
                .skip(self.current + 1)
                .find(|n| n.name == path && n.parent_idx == self.current)
                .unwrap().idx;
        }
    }

    fn size(&self, idx: usize) -> usize {
        let n = &self.nodes[idx];
        if n.size == 0 { // directory
            self.nodes.iter()
                .enumerate()
                .skip(idx + 1)
                .skip_while(|(_, n)| n.parent_idx != idx)
                .take(n.children_count)
                .map(|(i, _)| self.size(i))
                .sum()
        } else {
            n.size
        }
    }
}


#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Tree {
    let mut tree = Tree::new();
    input.lines().for_each(|l| {
        let s = l.trim_end();
        let split = s.split(" ").collect::<Vec<&str>>();
        if split[0] == "$" {
            if split[1] == "cd" {
                tree.cd(split[2])
            }
        } else if split[0] == "dir" {
            tree.add_dir(split[1])
        } else {
            tree.add_file(split[1].into(), split[0].parse().unwrap())
        }
    });
    tree
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Tree) -> usize {
    input.nodes.iter().enumerate()
        .filter(|(_, n)| n.size == 0)
        .map(|(i, _)| input.size(i))
        .filter(|s| *s <= 100000)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Tree) -> usize {
    let min_size = 30000000 - (70000000 - input.size(0));
    input.nodes.iter().enumerate()
        .filter(|(_, n)| n.size == 0)
        .map(|(i, _)| input.size(i))
        .filter(|s| *s >= min_size)
        .min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day7::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn size() {
        assert_eq!(input_generator(TEST_INPUT).size(5), 584);
        assert_eq!(input_generator(TEST_INPUT).size(1), 94853);
        assert_eq!(input_generator(TEST_INPUT).size(4), 24933642);
        assert_eq!(input_generator(TEST_INPUT).size(0), 48381165);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 95437);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST_INPUT)), 24933642);
    }
}
