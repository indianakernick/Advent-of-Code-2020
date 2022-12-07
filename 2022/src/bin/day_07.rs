use std::collections::HashMap;
use advent_of_code_2022 as util;

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

fn main() {
    let mut current = "/".to_owned();
    let mut tree = HashMap::<String, File>::new();

    util::each_line("input/day_07.txt", |line| {
        if let Some(path) = line.strip_prefix("$ cd ") {
            if path == "/" {
                current.clear();
            } else if path == ".." {
                current.truncate(current.rfind('/').unwrap());
            } else {
                current.push('/');
                current.push_str(path);
            }
            return;
        }

        if line == "$ ls" {
            return;
        }

        if let Some(name) = line.strip_prefix("dir ") {
            tree.insert(path_join(&current, name), File { dir: true, size: 0 });
            return;
        }

        if let Some((size, name)) = line.split_once(' ') {
            let size = size.parse().unwrap();
            tree.insert(path_join(&current, name), File { dir: false, size });
        }
    });

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

    println!("Part 1: {}", tree.iter()
        .filter(|(_, f)| f.dir && f.size <= 100000)
        .map(|(_, f)| f.size)
        .sum::<usize>());

    let minimum = 30000000 - (70000000 - root_size);

    println!("Part 2: {}", tree.iter()
        .filter(|(_, f)| f.dir && f.size >= minimum)
        .map(|(_, f)| f.size)
        .min().unwrap());
}
