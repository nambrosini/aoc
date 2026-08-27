fn decode(input: &str) -> String {
    let input = input.as_bytes();
    let mut r = String::new();
    let mut i = 1;
    while i < input.len() - 1 {
        if input[i] == b'\\' {
            if input[i + 1] == b'\\' {
                r.push('\\');
                i += 2;
            } else if input[i + 1] == b'"' {
                r.push('"');
                i += 2;
            } else if input[i + 1] == b'x' {
                r.push('?');
                i += 4;
            }
        } else {
            r.push(input[i] as char);
            i += 1;
        }
    }
    r
}

fn encode(input: &str) -> String {
    let input = input.as_bytes();
    let mut r = String::new();
    r.push('"');
    for c in input.iter() {
        match *c {
            b'"' => r.push_str("\\\""),
            b'\\' => r.push_str("\\\\"),
            _ => r.push(*c as char),
        }
    }
    r.push('"');
    r
}

fn part1(input: &str) -> usize {
    let mut delta = 0;
    for s in input.lines() {
        let literal_length = s.len();
        let decoded_length = decode(s).len();
        delta += literal_length - decoded_length;
    }
    delta
}

fn part2(input: &str) -> usize {
    let mut delta = 0;
    for s in input.lines() {
        let literal_length = s.len();
        let encoded_length = encode(s).len();
        delta += encoded_length - literal_length;
    }
    delta
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/08.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
