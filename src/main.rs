mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use common::*;

fn main() {
    print_day_one();
    print_day_two();
    print_day_three();
    print_day_four();
    print_day_five();
}

fn print_day_one() {
    println!("day one:");
    let input = &read_ints_from_file("src/day1/input");
    println!("part one: {}", day1::first(input));
    println!("part two: {}", day1::second(input));
    println!();
}

fn print_day_two() {
    println!("day two:");
    let input = &read_comma_separated("src/day2/input");
    println!("part one: {}", day2::first(input, 12, 2));
    println!("part two: {}", day2::second(input));
    println!();
}

fn print_day_three() {
    println!("day three:");
    let input = &read_file("src/day3/input");
    let mut lines = input.lines();
    let seq1: Vec<_> = lines.next().unwrap().split(',').collect();
    let seq2: Vec<_> = lines.next().unwrap().split(',').collect();
    println!("part one: {}", day3::first(&seq1, &seq2));
    println!("part two: {}", day3::second(&seq1, &seq2));
    println!();
}

fn print_day_four() {
    println!("day four:");
    println!("part one: {}", day4::first());
    println!("part two: {}", day4::second());
    println!();
}

fn print_day_five() {
    println!("day five:");
    let input = &read_comma_separated_int("src/day5/input");
    println!("part one: {}", day5::second(input, 1));
    println!("part two: {}", day5::second(input, 5));
    println!();
}
