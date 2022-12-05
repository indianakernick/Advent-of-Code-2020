use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

pub fn open_file<P: AsRef<Path>>(path: P) -> BufReader<File> {
    BufReader::new(File::open(path).unwrap())
}

pub fn each_line<P: AsRef<Path>, F: FnMut(&str)>(path: P, mut f: F) {
    let mut reader = open_file(path);
    let mut line = String::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        if line.as_bytes()[line.len() - 1] == b'\n' {
            line.pop();
        }
        f(&line);
        line.clear();
    }
}
