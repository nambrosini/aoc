use adventofcode::util::intcode::{Intcode, parse};
use std::collections::HashMap;

fn beam(mem: &[i64], x: i64, y: i64) -> i64 {
    let mut pc = Intcode::new(mem);
    pc.push_input(x);
    pc.run_to_output(y).unwrap()
}

fn part1(input: &str) -> i64 {
    let mem = parse(input);
    let mut count = 0;

    for x in 0..50 {
        for y in 0..50 {
            count += beam(&mem, x, y);
        }
    }

    count
}

fn part2(input: &str) -> i64 {
    let mem = parse(input);
    let mut cache: HashMap<(i64, i64), i64> = HashMap::new();
    let get = |cache: &mut HashMap<(i64, i64), i64>, x: i64, y: i64| {
        *cache.entry((x, y)).or_insert_with(|| beam(&mem, x, y))
    };

    for x in 500..10000 {
        let mut was_one = false;
        for y in x..10000 {
            let current = get(&mut cache, x, y);

            if current == 1 {
                was_one = true;
                let fits = (1..100)
                    .all(|k| get(&mut cache, x + k, y) == 1 && get(&mut cache, x, y + k) == 1);
                if fits {
                    return x * 10000 + y;
                }
            } else if was_one {
                break;
            }
        }
    }

    unreachable!()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/19.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
