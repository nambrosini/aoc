use adventofcode::util::iter::ChunkOps;
use adventofcode::util::parse::ParseOps;

struct Input {
    seeds: Vec<u64>,
    stages: Vec<Vec<[u64; 3]>>,
}

fn parse(input: &str) -> Input {
    let chunks: Vec<_> = input.split("\n\n").collect();

    let seeds = chunks[0].iter_unsigned().collect();
    let stages = chunks[1..]
        .iter()
        .map(|chunk| chunk.iter_unsigned().chunk::<3>().collect())
        .collect();

    Input { seeds, stages }
}

fn map_seeds(stages: &[Vec<[u64; 3]>], seeds: &[u64]) -> u64 {
    let mut seeds = seeds.to_vec();

    for stage in stages {
        'outer: for seed in seeds.iter_mut() {
            for [dest, start, count] in stage {
                if *seed >= *start && *seed <= start + count {
                    *seed = *seed - start + dest;
                    continue 'outer;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

fn part1(input: &str) -> u64 {
    let input = parse(input);
    map_seeds(&input.stages, &input.seeds)
}

fn part2(input: &str) -> u64 {
    let input = parse(input);
    let seeds: Vec<u64> = input
        .seeds
        .chunks(2)
        .flat_map(|seed| seed[0]..seed[0] + seed[1])
        .collect();

    map_seeds(&input.stages, &seeds)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2023/05.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 46);
    }
}
