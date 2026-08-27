use regex::Regex;

fn parse(input: &str) -> Statue {
    let re =
        Regex::new(r"Disc #\d has (\d+) positions; at time=0, it is at position (\d+)\.").unwrap();

    let mut discs = Vec::new();

    for caps in re.captures_iter(input) {
        let positions = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let current = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();

        discs.push(Disc::new(positions, current));
    }

    Statue::new(discs)
}

fn part1(input: &str) -> usize {
    let statue = parse(input);
    statue.get_passthrough_time()
}

fn part2(input: &str) -> usize {
    let mut statue = parse(input);

    statue.discs.push(Disc::new(11, 0));

    statue.get_passthrough_time()
}

#[derive(Debug, Copy, Clone)]
struct Disc {
    positions: usize,
    at_zero: usize,
}

impl Disc {
    fn new(positions: usize, at_zero: usize) -> Self {
        Self { positions, at_zero }
    }

    fn calc_position_at(&self, time: usize) -> usize {
        (self.at_zero + time) % self.positions
    }
}

#[derive(Debug, Clone)]
struct Statue {
    discs: Vec<Disc>,
}

impl Statue {
    fn new(discs: Vec<Disc>) -> Self {
        Self { discs }
    }

    fn get_passthrough_time(&self) -> usize {
        let mut time = 0;

        'outer: loop {
            let first_disc = self.discs[0].calc_position_at(time + 1);

            for i in 1..self.discs.len() {
                if self.discs[i].calc_position_at(time + i + 1) != first_disc {
                    time += 1;
                    continue 'outer;
                }
            }

            return time;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/15.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = Statue::new(vec![Disc::new(5, 4), Disc::new(2, 1)]);

        assert_eq!(s.get_passthrough_time(), 5);
    }
}
