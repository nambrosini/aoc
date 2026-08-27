use fnv::FnvHashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point2 {
    x: i32,
    y: i32,
}

impl Point2 {
    fn new(x: i32, y: i32) -> Self {
        Point2 { x, y }
    }

    fn distance(&self, other: &Self) -> i32 {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }
}

impl FromStr for Point2 {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Point2, Self::Err> {
        let parts: Vec<&str> = input.trim().split(',').collect();
        let x: i32 = parts[0].trim().parse()?;
        let y: i32 = parts[1].trim().parse()?;
        Ok(Point2::new(x, y))
    }
}

fn bounding_box(input: &[Point2]) -> (i32, i32, i32, i32) {
    let first = &input[0];
    input
        .iter()
        .fold((first.x, first.y, first.x, first.y), |acc, i| {
            (
                acc.0.min(i.x),
                acc.1.min(i.y),
                acc.2.max(i.x),
                acc.3.max(i.y),
            )
        })
}

fn parse(input: &str) -> Vec<Point2> {
    input
        .lines()
        .map(|l| l.parse().expect("Failed to parse Point2"))
        .collect()
}

fn part1(input: &str) -> i32 {
    let input = parse(input);
    let bbox = bounding_box(&input);
    let final_bounds = (bbox.0 - 400, bbox.1 - 400, bbox.2 + 400, bbox.3 + 400);

    let input_filtered: Vec<Point2> = input
        .iter()
        .filter_map(|p| {
            if p.x == bbox.0 || p.x == bbox.2 || p.y == bbox.1 || p.y == bbox.3 {
                None
            } else {
                Some(p.clone())
            }
        })
        .collect();

    let mut hash_map: FnvHashMap<Point2, i32> = FnvHashMap::default();
    (final_bounds.0..=final_bounds.2)
        .flat_map(|x| (final_bounds.1..=final_bounds.3).map(move |y| Point2::new(x, y)))
        .for_each(|i| {
            let (point, _, count) = input.iter().map(|p| (p, p.distance(&i))).fold(
                (Point2::new(0, 0), 100000, 0),
                |acc, i| match i.1 {
                    x if x < acc.1 => (i.0.clone(), i.1, 1),
                    x if x == acc.1 => (i.0.clone(), i.1, acc.2 + 1),
                    _ => acc,
                },
            );
            if count == 1 {
                let p_val = hash_map.entry(point).or_insert(0);
                *p_val += 1;
            }
        });

    *hash_map
        .iter()
        .filter_map(|(k, v)| {
            if input_filtered.iter().filter(|p| *p == k).count() == 1 {
                if *v < 10000 { Some(v) } else { None }
            } else {
                None
            }
        })
        .max()
        .expect("Could not find max value")
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let bbox = bounding_box(&input);

    (bbox.0..=bbox.2)
        .flat_map(|x| (bbox.1..=bbox.3).map(move |y| Point2::new(x, y)))
        .filter(|p| input.iter().map(|pp| pp.distance(p)).sum::<i32>() < 10000)
        .count()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/06.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
        assert_eq!(part1(input), 17);
    }
}
