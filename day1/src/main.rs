use std::fs;
use std::iter::Iterator;
use std::ops::FnMut;

fn main() {
    first();
    second();
}

fn second() {
    println!("part one: {}", process(fuel_rec))
}

fn first() {
    println!("part two: {}", process(|x| x))
}

fn process(transform: impl FnMut(i32) -> i32) -> i32 {
    fs::read_to_string("input")
        .unwrap()
        .lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .map(|x| x / 3 - 2)
        .map(transform)
        .sum()
}

fn fuel_rec(fuel_mass: i32) -> i32 {
    match fuel_mass {
        x if x <= 0 => 0,
        _ => fuel_mass + fuel_rec(fuel_mass / 3 - 2),
    }
}
