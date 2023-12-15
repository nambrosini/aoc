advent_of_code::solution!(15);

type Boxes = Vec<Vec<(String, u64)>>;

struct Input {
    vals: Vec<String>,
}

fn parse(input: &str) -> Input {
    Input {
        vals: input.split(',').map(|s| s.to_string()).collect(),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let vals = parse(input).vals;

    Some(vals.iter().map(|x| hash(x)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let vals = parse(input).vals;

    let mut boxes: Boxes = vec![Vec::new(); 256];

    for op in vals {
        if op.ends_with('-') {
            remove_lens(&mut boxes, &op[..op.len() - 1]);
        } else {
            let split: Vec<&str> = op.split('=').collect();
            let lens = split[0].to_string();
            let val: u64 = split[1].parse().unwrap();
            insert_lens(&mut boxes, &lens, val);
        }
    }

    let mut sum = 0;

    for (i, lenses) in boxes.iter().enumerate() {
        for (j, (_, len)) in lenses.iter().enumerate() {
            sum += (i + 1) * (j + 1) * *len as usize;
        }
    }

    Some(sum)
}

fn insert_lens(map: &mut Boxes, lens: &str, len: u64) {
    let index = hash(lens) as usize;

    if let Some(p) = map[index].iter().position(|(l, _)| lens == l) {
        map[index][p] = (lens.to_string(), len);
    } else {
        map[index].push((lens.to_string(), len));
    }
}

fn remove_lens(map: &mut Boxes, lens: &str) {
    for b in map.iter_mut() {
        if let Some(p) = b.iter().position(|(l, _)| lens == l) {
            b.remove(p);
        }
    }
}

fn hash(s: &str) -> u64 {
    s.chars()
        .fold(0, |acc, c| (acc + (c as u8) as u64) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }

    #[test]
    fn test_hash() {
        let got = hash("HASH");
        let want = 52;

        assert_eq!(want, got)
    }
}
