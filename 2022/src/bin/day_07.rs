use std::{ptr::NonNull, collections::HashMap};
use advent_of_code_2022 as util;

struct Directory {
    files: HashMap<String, File>,
    size: Option<usize>,
}

impl Directory {
    fn new() -> Self {
        Self { files: HashMap::new(), size: None }
    }

    fn add_directory(&mut self, name: &str) {
        self.files.entry(name.to_owned())
            .or_insert(File::Directory(Directory::new()));
    }

    fn add_regular(&mut self, name: &str, size: usize) {
        self.files.entry(name.to_owned())
            .or_insert(File::Regular(Regular { size }));
    }
}

struct Regular {
    size: usize,
}

enum File {
    Directory(Directory),
    Regular(Regular),
}

fn get_dir_size(dir: &mut Directory, small_dir_total: &mut usize) -> usize {
    if let Some(size) = dir.size {
        if size <= 100000 {
            *small_dir_total += size;
        }
        return size;
    }

    let mut size = 0;

    for (_, file) in dir.files.iter_mut() {
        match file {
            File::Directory(dir) => {
                size += get_dir_size(dir, small_dir_total);
            }
            File::Regular(reg) => {
                size += reg.size;
            }
        }
    }

    if size <= 100000 {
        *small_dir_total += size;
    }

    dir.size = Some(size);
    size
}

fn find_smallest_dir_larger_than(dir: &Directory, minimum: usize, mut current: usize) -> usize {
    let dir_size = dir.size.unwrap();
    if dir_size >= minimum && dir_size < current {
        current = dir_size;
    }

    for (_, file) in dir.files.iter() {
        match file {
            File::Directory(dir) => {
                current = find_smallest_dir_larger_than(&dir, minimum, current);
            }
            File::Regular(_) => continue,
        }
    }

    current
}

fn resolve_current(dir: &mut Directory, current_path: &[String]) -> NonNull<Directory> {
    if current_path.len() == 0 {
        NonNull::from(dir)
    } else {
        match dir.files.get_mut(&current_path[0]) {
            Some(File::Directory(next_dir)) => {
                resolve_current(next_dir, &current_path[1..])
            }
            Some(File::Regular(_)) => {
                panic!("Cannot enter regular file '{}'", &current_path[0]);
            }
            None => panic!("Trying to enter a directory that you don't know exists")
        }
    }
}

fn main() {
    let mut root = Directory::new();
    let mut current_path = Vec::<String>::new();
    let mut current_dir = NonNull::from(&mut root);

    util::each_line("input/day_07.txt", |line| {
        if line.starts_with("$ cd ") {
            if line.ends_with("/") {
                current_path.clear();
            } else if line.ends_with("..") {
                current_path.pop().unwrap();
            } else {
                current_path.push(line["$ cd ".len()..].to_owned());
            }
            current_dir = resolve_current(&mut root, &current_path);
            return;
        }

        if line == "$ ls" {
            return;
        }

        // Rust was the wrong choice...
        let current_dir = unsafe { current_dir.as_mut() };

        if line.starts_with("dir ") {
            current_dir.add_directory(&line["dir ".len()..]);
            return;
        }

        let space = line.bytes().position(|c| c == b' ').unwrap();
        let file_size = line[0..space].parse::<usize>().unwrap();
        let file_name = &line[space + 1..];

        current_dir.add_regular(file_name, file_size);
    });

    let mut small_dir_total = 0;
    let root_size = get_dir_size(&mut root, &mut small_dir_total);
    println!("Part 1: {}", small_dir_total);

    let minimum = 30000000 - (70000000 - root_size);
    println!("Part 2: {}", find_smallest_dir_larger_than(&root, minimum, usize::MAX));
}
