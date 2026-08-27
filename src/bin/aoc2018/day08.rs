fn parse(input: &str) -> Node {
    let mut input: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    get_node(&mut input)
}

fn part1(input: &str) -> usize {
    count_metadata(&parse(input))
}

fn part2(input: &str) -> usize {
    get_value(&parse(input))
}

fn count_metadata(node: &Node) -> usize {
    let mut metadata = node.metadata.iter().sum();

    for n in &node.children {
        metadata += count_metadata(n);
    }

    metadata
}

fn get_node(input: &mut Vec<usize>) -> Node {
    let header = (input.remove(0), input.remove(0));
    let mut children = vec![];

    for _ in 0..header.0 {
        children.push(get_node(input));
    }

    let mut metadata = vec![];

    for _ in 0..header.1 {
        metadata.push(input.remove(0));
    }

    Node::new(header, children, metadata)
}

fn get_value(node: &Node) -> usize {
    if node.header.0 == 0 {
        return node.metadata.iter().sum();
    }

    let mut sum = 0;

    for i in &node.metadata {
        if *i == 0 || *i > node.children.len() {
            continue;
        }
        sum += get_value(&node.children[*i - 1]);
    }

    sum
}

struct Node {
    header: (usize, usize),
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn new(header: (usize, usize), children: Vec<Node>, metadata: Vec<usize>) -> Node {
        Self {
            header,
            children,
            metadata,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/08.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 138);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 66);
    }
}
