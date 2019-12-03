use std::fs;
use std::iter::Iterator;

pub fn read_ints_from_file(filename: &str) -> Vec<i32> {
    read_file(filename)
        .lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}

pub fn read_comma_separated(filename: &str) -> Vec<usize> {
    read_file(filename)
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

pub fn read_file(filename: &str) -> String {
    fs::read_to_string(&filename).unwrap()
}
