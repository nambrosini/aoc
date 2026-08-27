use std::collections::HashMap;

type Filesystem = HashMap<String, Vec<(usize, String)>>;

fn build_absolute_path(path: &[&str], filename: Option<&str>) -> String {
    let mut s = String::from("/");
    for p in path.iter().skip(1) {
        s.push_str(p);
        s.push('/');
    }

    if let Some(name) = filename {
        s.push_str(name);
        if !name.contains('.') {
            s.push('/')
        }
    }

    s
}

fn parse(input: &str) -> HashMap<String, usize> {
    let mut map: Filesystem = HashMap::new();
    let mut current_folder: Vec<&str> = Vec::new();
    let mut files: Vec<(usize, String)> = Vec::new();

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        if line[0] == "$" {
            if line[1] == "cd" {
                let folder_absolute_path = build_absolute_path(&current_folder, None);
                let entry = map.entry(folder_absolute_path).or_default();
                entry.append(&mut files);
                if line[2] == ".." {
                    current_folder.pop();
                } else {
                    current_folder.push(line[2]);
                }
            }
        } else if line[0] == "dir" {
            let absolute_path = build_absolute_path(&current_folder, Some(line[1]));
            files.push((0, absolute_path));
        } else {
            let absolute_path = build_absolute_path(&current_folder, Some(line[1]));
            files.push((line[0].parse().unwrap(), absolute_path));
        }
    }

    let folder_absolute_path = build_absolute_path(&current_folder, None);
    let entry = map.entry(folder_absolute_path).or_default();
    entry.append(&mut files);

    let mut queue: Vec<String> = vec!["/".to_string()];
    let mut folder_sizes: HashMap<String, usize> = HashMap::new();

    while let Some(current) = queue.pop() {
        let content = &map[&current];

        let mut folders: Vec<String> = content
            .iter()
            .filter(|(size, name)| size == &0 && !folder_sizes.keys().any(|f| f == name))
            .map(|(_, name)| name.clone())
            .collect();
        if folders.is_empty() {
            let mut size = 0;
            for (s, n) in content {
                size += if s == &0 { folder_sizes[n] } else { *s }
            }
            folder_sizes.insert(current.to_string(), size);
        } else {
            queue.push(current);
            queue.append(&mut folders);
        }
    }

    folder_sizes
}

fn part1(input: &str) -> usize {
    parse(input).values().filter(|&&v| v <= 100_000).sum()
}

fn part2(input: &str) -> usize {
    let sizes = parse(input);
    let needed = 30_000_000 - (70_000_000 - sizes["/"]);

    sizes
        .values()
        .copied()
        .filter(|&v| v >= needed)
        .min()
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/07.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 24933642);
    }

    #[test]
    fn test_build_absolute_path_empty() {
        assert_eq!("/", build_absolute_path(&[], None));
    }

    #[test]
    fn test_build_absolute_path_root() {
        assert_eq!("/", build_absolute_path(&["/"], None));
    }

    #[test]
    fn test_build_absolute_path_dir() {
        assert_eq!("/a/b/c/", build_absolute_path(&["/", "a", "b"], Some("c")));
    }

    #[test]
    fn test_build_absolute_path_file() {
        assert_eq!("/a/b.tmp", build_absolute_path(&["/", "a"], Some("b.tmp")));
    }
}
