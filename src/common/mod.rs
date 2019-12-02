use std::fs;
use std::iter::Iterator;

pub fn read_ints_from_file(filename: &str) -> Vec<i32> {
    fs::read_to_string(&filename)
        .unwrap()
        .lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}
