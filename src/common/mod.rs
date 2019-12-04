use std::fs;
use std::iter::Iterator;

pub fn read_ints_from_file(filename: &str) -> Vec<i32> {
    read_file(filename)
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

pub fn read_comma_separated(filename: &str) -> Vec<usize> {
    read_file(filename)
        .split(',')
        .map(|x| x.trim())
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

pub fn read_comma_separated_int(filename: &str) -> Vec<i32> {
    read_file(filename)
        .split(',')
        .map(|x| x.trim())
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

pub fn read_pairs(filename: &str) -> Vec<(String, String)> {
    read_file(filename)
        .lines()
        .map(|l| l.split(')'))
        .map(|mut i| (i.next().unwrap().to_string(), i.next().unwrap().to_string()))
        .collect::<Vec<_>>()
}

pub fn read_file(filename: &str) -> String {
    fs::read_to_string(&filename).unwrap()
}
