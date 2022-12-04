use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn open_file<P>(path: P) -> BufReader<File>
    where P: AsRef<std::path::Path>
{
    BufReader::new(File::open(path).unwrap())
}

pub fn lines_from_file<P, F>(path: P, mut func: F)
    where
        P: AsRef<std::path::Path>,
        F: FnMut(&String)
{
    let mut buf_reader = open_file(path);
    let mut line = String::new();
    while buf_reader.read_line(&mut line).unwrap() > 0 {
        if line.as_bytes()[line.len() - 1] == b'\n' {
            line.pop().unwrap();
        }
        func(&line);
        line.clear();
    }
}

pub fn line_iter_from_file<P>(path: P) -> impl Iterator<Item = String>
    where P: AsRef<std::path::Path>
{
    open_file(path).lines().map(|line| line.unwrap())
}
