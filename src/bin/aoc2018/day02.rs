use itertools::Itertools;

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let checked = input.iter().map(|x| check_letters(x)).collect_vec();

    let two = checked.iter().fold(0, |sum, x| sum + x.0);
    let three = checked.iter().fold(0, |sum, x| sum + x.1);

    two * three
}

fn check_letters(id: &str) -> (usize, usize) {
    let mut vec = vec![];

    let mut two = 0;
    let mut three = 0;

    let v = id.chars().collect_vec();

    for c in v.iter() {
        if !vec.contains(c) {
            let count = v.iter().filter(|&x| x == c).count();

            if count == 2 && two == 0 {
                two += 1;
            } else if count == 3 && three == 0 {
                three += 1;
            }

            vec.push(*c);
        }
    }

    (two, three)
}

fn part2(input: &str) -> String {
    let id = parse(input);
    let (i1, i2) = find_similar(&id);

    let mut vec = vec![];

    let id1 = id[i1].chars();
    let mut id2 = id[i2].chars();

    for c in id1 {
        if c == id2.next().unwrap() {
            vec.push(c);
        }
    }

    vec.iter().join("")
}

fn find_similar(id: &[String]) -> (usize, usize) {
    for i in 0..id.len() {
        for j in 0..id.len() {
            let b1 = id[i].chars().collect_vec();
            let b2 = id[j].chars().collect_vec();

            let mut diff = 0;

            'inner: for k in 0..b1.len() {
                if b1[k] != b2[k] {
                    if diff == 0 {
                        diff = 1;
                    } else {
                        diff = 2;
                        break 'inner;
                    }
                }
            }

            if diff == 1 {
                return (i, j);
            }
        }
    }

    unreachable!();
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/02.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        assert_eq!(part1(s), 12);
    }

    #[test]
    fn test_part2() {
        let s = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        assert_eq!(part2(s), "fgij");
    }
}
