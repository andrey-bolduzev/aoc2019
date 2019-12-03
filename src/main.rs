mod common;
mod day1;
mod day2;
mod day3;

use common::*;

fn main() {
    print_day_one();
    print_day_two();
    print_day_three();
}

fn print_day_one() {
    println!("day one:");
    let input1 = &read_ints_from_file("src/day1/input");
    println!("part one: {}", day1::first(input1));
    println!("part two: {}", day1::second(input1));
    println!();
}

fn print_day_two() {
    println!("day two:");
    let input2 = &read_comma_separated("src/day2/input");
    println!("part one: {}", day2::first(input2, 12, 2));
    println!("part two: {}", day2::second(input2));
    println!();
}

fn print_day_three() {
    println!("day three:");
    let input3 = &read_file("src/day3/input");
    let mut lines = input3.lines();
    let seq1: Vec<_> = lines.next().unwrap().split(',').collect();
    let seq2: Vec<_> = lines.next().unwrap().split(',').collect();
    println!("part one: {}", day3::first(&seq1, &seq2));
    println!("part two: {}", day3::second(&seq1, &seq2));
    println!();
}
