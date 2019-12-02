mod common;
mod day1;
mod day2;

use common::*;

fn main() {
    println!("day one:");
    let input1 = &read_ints_from_file("src/day1/input");
    println!("part one: {}", day1::first(input1));
    println!("part two: {}", day1::second(input1));

    println!();

    println!("day two:");
    let input2 = &read_comma_separated("src/day2/input");
    println!("part one: {}", day2::first(input2, 12, 2));
    println!("part two: {}", day2::second(input2));
}
