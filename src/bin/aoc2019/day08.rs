const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .chunks(LAYER_SIZE)
        .map(|c| c.to_vec())
        .collect()
}

fn part1(input: &str) -> usize {
    let layers = parse(input);
    let layer = layers
        .iter()
        .min_by_key(|l| l.iter().filter(|&&d| d == 0).count())
        .unwrap();

    let ones = layer.iter().filter(|&&d| d == 1).count();
    let twos = layer.iter().filter(|&&d| d == 2).count();

    ones * twos
}

fn render(input: &str) -> String {
    let layers = parse(input);
    let mut image = [2u32; LAYER_SIZE];

    for layer in &layers {
        for (px, &d) in image.iter_mut().zip(layer) {
            if *px == 2 {
                *px = d;
            }
        }
    }

    let mut s = String::new();
    for row in image.chunks(WIDTH) {
        for &px in row {
            s.push(if px == 1 { '#' } else { ' ' });
        }
        s.push('\n');
    }
    s
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/08.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2:\n{}", render(&input));
}
