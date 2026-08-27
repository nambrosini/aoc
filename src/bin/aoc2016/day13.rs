use itertools::Itertools;

fn parse(input: &str) -> i32 {
    input.trim().parse().unwrap()
}

fn part1(input: &str) -> usize {
    let favorite = parse(input);
    astar(&(1, 1), &(31, 39), &favorite).len() - 1
}

fn part2(input: &str) -> usize {
    let favorite = parse(input);
    check_nodes(&favorite)
}

fn check_nodes(favorite: &i32) -> usize {
    let mut open_list: Vec<(Point, usize)> = Vec::new();
    let mut closed_list: Vec<(Point, usize)> = Vec::new();

    let start = (1, 1);

    open_list.push((start, 0));

    while !open_list.is_empty() {
        let mut current_node = open_list[0];
        let mut current_index = 0;

        for (index, &item) in open_list.iter().enumerate() {
            if item.1 < current_node.1 {
                current_node = item;
                current_index = index;
            }
        }

        open_list.remove(current_index);

        if current_node.1 > 50 {
            continue;
        }
        closed_list.push(current_node);

        let children = get_point_neighbours(&current_node.0, favorite);

        for child in children {
            if closed_list.iter().any(|(c, _)| c == &child) {
                continue;
            }

            if let Some((i, c)) = open_list.iter().enumerate().find(|(_, c)| c.0 == child) {
                if c.1 > current_node.1 {
                    open_list.remove(i);
                } else {
                    continue;
                }
            }
            open_list.push((child, current_node.1 + 1));
        }
    }

    closed_list.len()
}

type Point = (i32, i32);

fn astar(start: &Point, end: &Point, favorite: &i32) -> Vec<Point> {
    let mut open_list: Vec<Node> = Vec::new();
    let mut closed_list: Vec<Node> = Vec::new();

    let start = Node::new(*start, None);
    let end = Node::new(*end, None);

    open_list.push(start);

    while !open_list.is_empty() {
        let mut current_node = open_list[0];
        let mut current_index = 0;

        for (index, &item) in open_list.iter().enumerate() {
            if item.f < current_node.f {
                current_node = item;
                current_index = index;
            }
        }

        open_list.remove(current_index);
        closed_list.push(current_node);

        if current_node == end {
            let mut path = vec![];
            let mut current = Some(&current_node);
            while let Some(c) = current {
                current = closed_list.iter().find(|x| Some(x.position) == c.parent);
                path.push(c.position);
            }
            return path.iter().rev().cloned().collect::<Vec<Point>>();
        }

        let children = get_neighbours(&current_node, favorite);

        for child in children {
            if closed_list.contains(&child) {
                continue;
            }

            let mut child = child;
            child.g = current_node.g + 1;
            child.h = (child.position.0 - end.position.0).abs()
                + (child.position.1 - end.position.1).abs();
            child.f = child.g + child.h;

            if open_list.iter().any(|&x| x == child && child.g > x.g) {
                continue;
            }

            open_list.push(child);
        }
    }

    vec![]
}

#[derive(Debug, Clone, Copy, Eq)]
struct Node {
    parent: Option<Point>,
    position: Point,
    f: i32,
    g: i32,
    h: i32,
}

impl Node {
    fn new(position: Point, parent: Option<Point>) -> Self {
        Self {
            parent,
            position,
            g: 0,
            h: 0,
            f: 0,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

fn get_point_neighbours(current: &Point, favorite: &i32) -> Vec<Point> {
    let neighbours = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    neighbours
        .iter()
        .map(|&p| (current.0 + p.0, current.1 + p.1))
        .filter(|&x| is_open(&x, favorite) && x.0 >= 0 && x.1 >= 0)
        .collect_vec()
}

fn get_neighbours(current: &Node, favorite: &i32) -> Vec<Node> {
    let neighbours = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    neighbours
        .iter()
        .map(|&p| {
            Node::new(
                (current.position.0 + p.0, current.position.1 + p.1),
                Some(current.position),
            )
        })
        .filter(|&x| is_open(&x.position, favorite))
        .collect_vec()
}

fn is_open(point: &Point, favorite: &i32) -> bool {
    let x = point.0;
    let y = point.1;

    let res = x * x + 3 * x + 2 * x * y + y + y * y + favorite;

    res.count_ones().is_multiple_of(2)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/13.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let path = astar(&(1, 1), &(7, 4), &10);
        assert_eq!(path.len() - 1, 11);
    }
}
