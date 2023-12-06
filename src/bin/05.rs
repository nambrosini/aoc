advent_of_code::solution!(5);

use advent_of_code::util::iter::ChunkOps;
use advent_of_code::util::parse::ParseOps;

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    let seeds = input.seeds.clone();
    Some(map_seeds(&input.stages, &seeds))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);
    let seeds: &mut Vec<u64> = &mut input
        .seeds
        .chunks(2)
        .flat_map(|seed| seed[0]..seed[0] + seed[1])
        .collect();

    Some(map_seeds(&input.stages, seeds))
}

pub struct Input {
    seeds: Vec<u64>,
    stages: Vec<Vec<[u64; 3]>>,
}

pub fn parse(input: &str) -> Input {
    let chunks: Vec<_> = input.split("\n\n").collect();
    // Getting the seed, skips the 'seeds:'
    let seeds = chunks[0].iter_unsigned().collect();
    // Getting the stages. Each stage is in its own array, and has multiple ranges.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
