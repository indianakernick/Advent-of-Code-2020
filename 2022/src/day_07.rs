use std::collections::HashMap;

#[derive(Clone, Copy)]
struct File {
    dir: bool,
    size: usize,
}

fn path_join(parent: &str, child: &str) -> String {
    let mut path = String::with_capacity(parent.len() + 1 + child.len());
    path.push_str(parent);
    path.push('/');
    path.push_str(child);
    path
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut current = "/".to_owned();
    let mut tree = HashMap::<String, File>::new();

    for line in input.lines() {
        if let Some(path) = line.strip_prefix("$ cd ") {
            if path == "/" {
                current.clear();
            } else if path == ".." {
                current.truncate(current.rfind('/').unwrap());
            } else {
                current.push('/');
                current.push_str(path);
            }
            continue;
        }

        if line == "$ ls" {
            continue;
        }

        if let Some(name) = line.strip_prefix("dir ") {
            tree.insert(path_join(&current, name), File { dir: true, size: 0 });
            continue;
        }

        if let Some((size, name)) = line.split_once(' ') {
            let size = size.parse().unwrap();
            tree.insert(path_join(&current, name), File { dir: false, size });
        }
    }

    let mut paths = tree.iter()
        .map(|pair| pair.0.clone())
        .collect::<Vec<_>>();
    paths.sort_unstable_by(|a, b| b.cmp(a));

    let mut root_size = 0;

    for path in paths.iter() {
        if let Some((parent_path, _)) = path.rsplit_once('/') {
            if let Some(file) = tree.get(path).copied() {
                if parent_path.is_empty() {
                    root_size += file.size;
                } else {
                    if let Some(parent_file) = tree.get_mut(parent_path) {
                        parent_file.size += file.size;
                    }
                }
            }
        }
    }

    let part_1 = tree.iter()
        .filter(|(_, f)| f.dir && f.size <= 100000)
        .map(|(_, f)| f.size)
        .sum::<usize>();

    let minimum = 30000000 - (70000000 - root_size);

    let part_2 = tree.iter()
        .filter(|(_, f)| f.dir && f.size >= minimum)
        .map(|(_, f)| f.size)
        .min().unwrap();

    (part_1, part_2)
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"$ cd /
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
    let output = solve(input);
    assert_eq!(output.0, 95437);
    assert_eq!(output.1, 24933642);
}
