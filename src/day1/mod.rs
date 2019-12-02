use std::convert::identity;
use std::ops::FnMut;

pub fn first(input: &[i32]) -> i32 {
    process(input, identity)
}

pub fn second(input: &[i32]) -> i32 {
    process(input, fuel_rec)
}

fn process(input: &[i32], transform: impl FnMut(i32) -> i32) -> i32 {
    input.iter().map(|x| x / 3 - 2).map(transform).sum()
}

fn fuel_rec(fuel_mass: i32) -> i32 {
    match fuel_mass {
        x if x <= 0 => 0,
        _ => fuel_mass + fuel_rec(fuel_mass / 3 - 2),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(
        input,
        expected,
        case(12, 2),
        case(14, 2),
        case(1969, 654),
        case(100756, 33583)
    )]
    fn part_one_examples(input: i32, expected: i32) {
        assert_eq!(first(&[input]), expected);
    }

    #[rstest_parametrize(input, expected, case(14, 2), case(1969, 966), case(100756, 50346))]
    fn part_two_examples(input: i32, expected: i32) {
        assert_eq!(second(&[input]), expected);
    }
}
