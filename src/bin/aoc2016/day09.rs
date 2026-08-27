use itertools::Itertools;

fn part1(input: &str) -> usize {
    let input = input.trim().chars().collect_vec();
    let mut decompressed_file = String::new();

    let mut in_brackets = false;
    let mut before_x = false;
    let mut i = 0;

    let mut len = 0;
    let mut repetitions = 0;

    while i < input.len() {
        if input[i] == '(' {
            in_brackets = true;
            before_x = true;
        } else if input[i] == ')' {
            repeat(
                &mut decompressed_file,
                &input[i + 1..i + 1 + len].iter().join(""),
                repetitions,
            );
            i += len + 1;
            len = 0;
            repetitions = 0;
            in_brackets = false;
            continue;
        } else if input[i] == 'x' {
            before_x = false;
        } else if in_brackets {
            if before_x {
                len = len * 10 + input[i].to_digit(10).unwrap() as usize;
            } else {
                repetitions = repetitions * 10 + input[i].to_digit(10).unwrap() as usize;
            }
        } else {
            decompressed_file.push(input[i]);
        }

        i += 1;
    }

    decompressed_file.len()
}

fn part2(input: &str) -> usize {
    decompress(&input.trim().chars().collect_vec(), 1)
}

fn decompress(file: &[char], repeat: usize) -> usize {
    let mut count = 0;

    let mut new_part = Vec::new();

    for _ in 0..repeat {
        new_part.append(&mut file.to_vec());
    }

    let mut in_brackets = false;
    let mut before_x = false;
    let mut i = 0;

    let mut len = 0;
    let mut repetitions = 0;

    while i < new_part.len() {
        if new_part[i] == '(' {
            in_brackets = true;
            before_x = true;
        } else if new_part[i] == ')' {
            count += decompress(&new_part[i + 1..i + 1 + len], repetitions);
            i += len + 1;
            len = 0;
            repetitions = 0;
            in_brackets = false;
            continue;
        } else if new_part[i] == 'x' {
            before_x = false;
        } else if in_brackets {
            if before_x {
                len = len * 10 + new_part[i].to_digit(10).unwrap() as usize;
            } else {
                repetitions = repetitions * 10 + new_part[i].to_digit(10).unwrap() as usize;
            }
        } else {
            count += 1;
        }

        i += 1;
    }

    count
}

fn repeat(decompressed_file: &mut String, chars: &str, repetitions: usize) {
    for _ in 0..repetitions {
        decompressed_file.push_str(chars);
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/09.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("ADVENT"), 6);
        assert_eq!(part1("A(1x5)BC"), 7);
        assert_eq!(part1("(3x3)XYZ"), 9);
        assert_eq!(part1("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(part1("(6x1)(1x3)A"), 6);
        assert_eq!(part1("X(8x2)(3x3)ABCY"), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("(3x3)XYZ"), 9);
        assert_eq!(part2("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(
            part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
