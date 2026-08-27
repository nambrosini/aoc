use itertools::Itertools;

fn parse(input: &str) -> Vec<Rule> {
    input.lines().map(Rule::from).collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let v = enhance(&input, 5);

    v.iter().flatten().filter(|&&x| x == '#').count()
}

fn enhance(rules: &[Rule], iterations: usize) -> Vec<Vec<char>> {
    let mut v = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    for _ in 0..iterations {
        let size = v.len();
        if size % 2 == 0 {
            v = take2(&v, rules);
        } else {
            v = take3(&v, rules);
        }
    }

    v
}

// The original solution never implemented part 2 (it needs a memoized
// subgrid-counting approach to be tractable at 18 iterations); ported as-is
// rather than fabricating an answer.
fn part2(_input: &str) -> usize {
    0
}

fn take2(v: &[Vec<char>], rules: &[Rule]) -> Vec<Vec<char>> {
    let mut squares: Vec<String> = vec![];

    for i in 0..(v.len() / 2) {
        for j in 0..(v.len() / 2) {
            let mut s = String::new();

            s.push(v[i * 2][j * 2]);
            s.push(v[i * 2][j * 2 + 1]);
            s.push('/');
            s.push(v[i * 2 + 1][j * 2]);
            s.push(v[i * 2 + 1][j * 2 + 1]);

            squares.push(s);
        }
    }

    let mut new_square: Vec<Vec<Vec<char>>> = vec![];

    'outer: for square in &squares {
        for r in rules {
            if let Some(enhancement) = r.check_pattern(&string_to_vec(square)) {
                new_square.push(enhancement);
                continue 'outer;
            }
        }
    }

    let mut v: Vec<Vec<char>> = vec![vec![]; 3];

    for r in new_square.iter() {
        for (j, e) in r.iter().enumerate() {
            v[j].extend(e);
        }
    }

    v
}

fn take3(v: &[Vec<char>], rules: &[Rule]) -> Vec<Vec<char>> {
    let mut squares: Vec<String> = vec![];

    for i in 0..(v.len() / 3) {
        for j in 0..(v.len() / 3) {
            let mut s = String::new();

            for x in 0..3 {
                if x > 0 {
                    s.push('/');
                }
                for y in 0..3 {
                    s.push(v[i * 3 + x][j * 3 + y]);
                }
            }

            squares.push(s);
        }
    }

    let mut new_square: Vec<Vec<Vec<char>>> = vec![];

    'outer: for square in &squares {
        for r in rules {
            if let Some(enhancement) = r.check_pattern(&string_to_vec(square)) {
                new_square.push(enhancement);
                continue 'outer;
            }
        }
    }

    let mut v: Vec<Vec<char>> = vec![vec![]; 4];

    for r in new_square {
        for (j, e) in r.iter().enumerate() {
            v[j].extend(e);
        }
    }

    v
}

fn string_to_vec(s: &str) -> Vec<Vec<char>> {
    let mut v = Vec::new();

    let split = s.split('/');

    for s in split {
        v.push(s.chars().collect());
    }

    v
}

struct Rule {
    from: String,
    to: String,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let split = s.split(" => ").collect_vec();

        Self {
            from: split[0].to_string(),
            to: split[1].to_string(),
        }
    }
}

impl Rule {
    fn check_pattern(&self, pattern: &[Vec<char>]) -> Option<Vec<Vec<char>>> {
        if self.from == Self::string(pattern) {
            return Some(string_to_vec(&self.to));
        }

        let r0h = Self::flip_h(pattern);
        let r0v = Self::flip_v(pattern);
        let r90 = Self::rotate(pattern);
        let r90h = Self::flip_h(&r90);
        let r90v = Self::flip_v(&r90);
        let r180 = Self::rotate(&r90);
        let r270 = Self::rotate(&r180);

        if self.from == Self::string(&r0h)
            || self.from == Self::string(&r0v)
            || self.from == Self::string(&r90)
            || self.from == Self::string(&r90h)
            || self.from == Self::string(&r90v)
            || self.from == Self::string(&r180)
            || self.from == Self::string(&r270)
        {
            return Some(string_to_vec(&self.to));
        }

        None
    }

    fn rotate(pattern: &[Vec<char>]) -> Vec<Vec<char>> {
        let n = pattern.len();
        let mut ret = pattern.to_vec();

        for (i, r) in ret.iter_mut().enumerate() {
            for (j, e) in r.iter_mut().enumerate() {
                *e = pattern[n - j - 1][i];
            }
        }

        ret
    }

    fn flip_h(pattern: &[Vec<char>]) -> Vec<Vec<char>> {
        let mut ret = pattern.to_vec();

        for row in 0..ret.len() / 2 {
            let temp = ret[ret.len() - row - 1].to_vec();
            ret[pattern.len() - row - 1] = pattern[row].to_vec();
            ret[row] = temp;
        }

        ret
    }

    fn flip_v(pattern: &[Vec<char>]) -> Vec<Vec<char>> {
        let mut ret = pattern.to_vec();

        for row in 0..ret.len() {
            for col in 0..ret[0].len() / 2 {
                let temp = ret[row][ret[0].len() - col - 1];
                ret[row][pattern[0].len() - col - 1] = ret[row][col];
                ret[row][col] = temp;
            }
        }

        ret
    }

    fn string(chars: &[Vec<char>]) -> String {
        let mut s = String::new();

        for (i, r) in chars.iter().enumerate() {
            s.push_str(&r.iter().join(""));
            if i < chars.len() - 1 {
                s.push('/');
            }
        }

        s
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/21.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhance() {
        // AoC's own published example only demonstrates 2 rounds with this
        // tiny 2-rule set; part1's real ruleset covers 5 rounds.
        let s = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        let rules = parse(s);

        let v = enhance(&rules, 2);

        assert_eq!(v.iter().flatten().filter(|&&x| x == '#').count(), 12);
    }
}
