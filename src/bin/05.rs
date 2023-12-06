advent_of_code::solution!(5);

use itertools::Itertools;
use advent_of_code::util::iter::ChunkOps;
use advent_of_code::util::parse::ParseOps;

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    let seeds = input.seeds.clone();
    Some(map_seeds(&input.stages, &seeds))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);
    let mut current = &mut Vec::new();
    let mut next = &mut Vec::new();

    // Convert input pairs to ranges.
    for [start, length] in input.seeds.iter().copied().chunk::<2>() {
        current.push([start, start + length]);
    }

    for stage in &input.stages {
        'outer: for &[s1, e1] in current.iter() {
            // Split ranges that overlap into 1, 2 or 3 new ranges.
            // Assumes that seed ranges will only overlap with a single range in each stage.
            for &[dest, s2, e2] in stage {
                // x1 and x2 are the possible overlap.
                let x1 = s1.max(s2);
                let x2 = e1.min(e2);

                if x1 < x2 {
                    if s1 < x1 {
                        next.push([s1, x1]);
                    }
                    if x2 < e1 {
                        next.push([x2, e1]);
                    }
                    // Move overlap to new destination.
                    next.push([x1 - s2 + dest, x2 - s2 + dest]);
                    continue 'outer;
                }
            }
            // No intersection with any range so pass to next stage unchanged.
            next.push([s1, e1]);
        }

        (current, next) = (next, current);
        next.clear();
    }

    Some(current.iter().min_by_key(|r| r[0]).unwrap()[0])
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
