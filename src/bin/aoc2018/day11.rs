fn parse(input: &str) -> i64 {
    input.trim().parse().unwrap()
}

fn part1(input: &str) -> String {
    let serial = parse(input);
    let grid = calc_grid(&serial);
    let mut max_power = 0;
    let mut max_cell = (0, 0);

    for x in 1..=300 - 3 {
        for y in 1..=300 - 3 {
            let val = calc_square_power(&x, &y, 3, &grid);

            if val > max_power {
                max_power = val;
                max_cell = (x + 1, y + 1);
            }
        }
    }

    format!("{},{}", max_cell.0, max_cell.1)
}

fn part2(input: &str) -> String {
    let serial = parse(input);
    let grid = calc_grid(&serial);

    let mut max_power = 0;
    let mut max_cell = (0, 0, 0);

    for size in 1..=300 {
        for x in 1..=300 - (size) {
            for y in 1..=300 - (size) {
                let val = calc_square_power(&x, &y, size, &grid);

                if val > max_power {
                    max_power = val;
                    max_cell = (x + 1, y + 1, size);
                }
            }
        }
    }

    format!("{},{},{}", max_cell.0, max_cell.1, max_cell.2)
}

fn calc_square_power(x: &usize, y: &usize, size: usize, grid: &[[i64; 300]; 300]) -> i64 {
    let mut power = 0;

    for xi in 0..size {
        for yi in 0..size {
            power += grid[x + xi][y + yi]
        }
    }

    power
}

fn calc_power_level(x: i64, y: i64, serial: &i64) -> i64 {
    let rack_id = x + 10;
    ((rack_id * y + serial) * rack_id) / 100 % 10 - 5
}

fn calc_grid(serial: &i64) -> [[i64; 300]; 300] {
    let mut grid: [[i64; 300]; 300] = [[0; 300]; 300];

    for (i, r) in grid.iter_mut().enumerate() {
        for (j, e) in r.iter_mut().enumerate() {
            *e = calc_power_level(i as i64 + 1, j as i64 + 1, serial);
        }
    }

    grid
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/11.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(4, calc_power_level(3, 5, &8));
        assert_eq!(-5, calc_power_level(122, 79, &57));
        assert_eq!(0, calc_power_level(217, 196, &39));
        assert_eq!(4, calc_power_level(101, 153, &71));
    }
}
