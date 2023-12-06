use std::collections::HashSet;
advent_of_code::solution!(3);

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let mut sum = 0;
    let mut last_num = 0;
    for (i, l) in input.iter().enumerate() {
        for (j, e) in l.iter().enumerate() {
            if e.is_ascii_digit() {
                if check_adjacent_symbol(i, j, &input) {
                    let v = get_value(j, l);
                    if v == last_num {
                        continue;
                    }
                    sum += v;
                    last_num = v;
                }
            } else {
                last_num = 0;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let mut res = 0;
    for (i, l) in input.iter().enumerate() {
        for (j, e) in l.iter().enumerate() {
            if e == &'*' {
                res += get_gears(i, j, &input);
            }
        }
    }
    Some(res)
}

fn check_adjacent_symbol(x: usize, y: usize, map: &[Vec<char>]) -> bool {
    let x = x as i32;
    let y = y as i32;
    for i in -1..=1 {
        let new_x = x + i;
        if !(0..map.len() as i32).contains(&new_x) {
            continue;
        }
        for j in -1..=1 {
            let new_y = y + j;
            if i == 0 && j == 0 {
                continue;
            }
            if !(0..map[0].len() as i32).contains(&new_y) {
                continue;
            }
            let e = map[new_x as usize][new_y as usize];
            if !e.is_ascii_digit() && e != '.' {
                return true;
            }
        }
    }

    false
}

fn get_value(x: usize, row: &[char]) -> u32 {
    let mut start = x;
    let mut end = x;
    let mut found_start = false;
    let mut found_end = false;
    let mut i = 1;
    while !found_end || !found_start {
        if !found_end {
            if x + i < row.len() {
                if !row[x + i].is_ascii_digit() {
                    found_end = true;
                } else if row[x + i].is_ascii_digit() {
                    end = x + i;
                }
            } else {
                found_end = true;
            }
        }
        if !found_start {
            if x.checked_sub(i).is_some() {
                if !row[x - i].is_ascii_digit() {
                    found_start = true;
                } else {
                    start = x - i;
                }
            } else {
                found_start = true;
            }
        }
        i += 1;
    }

    row[start..=end].iter().collect::<String>().parse().unwrap()
}

fn get_gears(x: usize, y: usize, map: &[Vec<char>]) -> u32 {
    let mut gears = HashSet::new();
    let x = x as i32;
    let y = y as i32;
    for i in -1..=1 {
        let new_x = x + i;
        if !(0..map.len() as i32).contains(&new_x) {
            continue;
        }
        for j in -1..=1 {
            let new_y = y + j;
            if i == 0 && j == 0 {
                continue;
            }
            if !(0..map[0].len() as i32).contains(&new_y) {
                continue;
            }
            let e = map[new_x as usize][new_y as usize];
            if e.is_ascii_digit() {
                gears.insert(get_value(new_y as usize, &map[new_x as usize]));
                if gears.len() == 2 {
                    return gears.iter().product();
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
