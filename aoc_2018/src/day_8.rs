fn build_tree(tree: &[usize]) -> (Node, usize) {
    if tree[0] == 0 {
        return (
            Node {
                children: Vec::new(),
                metadata: tree[2..tree[1] + 2].iter().cloned().collect(),
            },
            tree[1] + 2,
        );
    }

    let mut node = Node {
        children: Vec::with_capacity(tree[0]),
        metadata: Vec::with_capacity(tree[1]),
    };

    let mut consumed = 2;
    for _ in 0..tree[0] {
        let (child, n) = build_tree(&tree[consumed..]);
        node.children.push(child);
        consumed += n;
    }

    for _ in 0..tree[1] {
        node.metadata.push(tree[consumed]);
        consumed += 1;
    }

    (node, consumed)
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Box<Node> {
    let data: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (tree, n) = build_tree(&data);
    assert_eq!(n, data.len());
    Box::new(tree)
}

#[aoc(day8, part1)]
pub fn part1(tree: &Node) -> usize {
    tree.metadata_sum()
}

#[aoc(day8, part2)]
pub fn part2(tree: &Node) -> usize {
    tree.value()
}

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn metadata_sum(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata.iter().sum::<usize>()
                + self
                    .children
                    .iter()
                    .map(|x| x.metadata_sum())
                    .sum::<usize>()
        }
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata_sum()
        } else {
            self.metadata
                .iter()
                .map(|entry| {
                    self.children
                        .get(*entry - 1)
                        .map_or(0, |child| child.value())
                })
                .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn d8p1() {
        let v = input();
        let (tree, _) = super::build_tree(&v);
        assert_eq!(138, super::part1(&tree));
    }

    fn input() -> Vec<usize> {
        vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]
    }

    #[test]
    fn d8p2() {
        let v = input();
        let (tree, _) = super::build_tree(&v);
        assert_eq!(66, super::part2(&tree));
    }
}
