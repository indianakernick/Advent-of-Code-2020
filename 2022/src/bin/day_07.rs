use std::ptr::addr_of_mut;

use advent_of_code_2022 as util;

struct Dir {
    files: Vec<Box<File>>,
    size: Option<usize>,
}

enum File {
    Dir(String, Dir),
    File(String, usize),
}

fn get_dir_size(dir: &mut Dir, total: &mut usize) -> usize {
    if let Some(size) = dir.size {
        if size <= 100000 {
            *total += size;
        }
        return size;
    }

    let mut size = 0;

    for file in dir.files.iter_mut() {
        match file.as_mut() {
            File::Dir(_, dir) => {
                size += get_dir_size(dir, total);
            }
            File::File(_, file_size) => {
                size += *file_size;
            }
        }
    }

    if size <= 100000 {
        *total += size;
    }

    dir.size = Some(size);
    size
}

fn main() {
    let mut root = Dir { files: Vec::new(), size: None };
    let mut curr_dir = Vec::<String>::new();

    util::each_line("input/day_07.txt", |line| {
        if line.starts_with("$ cd ") {
            if line.ends_with("/") {
                curr_dir.clear();
            } else if line.ends_with("..") {
                curr_dir.pop();
            } else {
                curr_dir.push(String::from(&line["$ cd ".len()..]));
            }
            return;
        }

        if line == "$ ls" {
            return;
        }

        // Rust was the wrong choice...
        unsafe {
            let mut dir = addr_of_mut!(root);
            'c: for comp in curr_dir.iter() {
                for file in (*dir).files.iter_mut() {
                    match file.as_mut() {
                        File::Dir(name, child_dir) => {
                            if comp == name {
                                dir = child_dir;
                                continue 'c;
                            }
                        }
                        File::File(_, _) => continue,
                    }
                }

                (*dir).files.push(Box::new(File::Dir(
                    comp.clone(),
                    Dir { files: Vec::new(), size: None }
                )));
            }

            if line.starts_with("dir ") {
                for file in (*dir).files.iter() {
                    match file.as_ref() {
                        File::Dir(name, _) => {
                            if name == &line["dir ".len()..] {
                                return;
                            }
                        },
                        File::File(name, _) => {
                            if name == &line["dir ".len()..] {
                                panic!();
                            }
                        },
                    }
                }

                (*dir).files.push(Box::new(File::Dir(
                    line["dir ".len()..].to_owned(),
                    Dir { files: Vec::new(), size: None }
                )));
                return;
            }

            let space = line.bytes().position(|c| c == b' ').unwrap();
            let file_size = line[0..space].parse::<usize>().unwrap();
            let file_name = &line[space + 1..];

            for file in (*dir).files.iter() {
                match file.as_ref() {
                    File::Dir(name, _) => {
                        if name == file_name {
                            panic!();
                        }
                    },
                    File::File(name, size) => {
                        if name == file_name {
                            if *size != file_size {
                                panic!();
                            }
                            return;
                        }
                    },
                }
            }

            (*dir).files.push(Box::new(File::File(
                file_name.to_owned(),
                file_size
            )));
        }
    });

    let mut total = 0;
    get_dir_size(&mut root, &mut total);
    println!("Part 1: {}", total);
}
