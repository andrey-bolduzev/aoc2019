mod common;
mod day1;

use common::*;

fn main() {
    println!("day one:");
    let input1 = &read_ints_from_file("src/day1/input");
    println!("part one: {}", day1::first(input1));
    println!("part two: {}", day1::second(input1));
}
