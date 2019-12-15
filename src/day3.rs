use itertools::Itertools;
use std::collections::HashSet;
use std::iter::Iterator;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.trim().lines();
    (
        lines
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.to_string())
            .collect_vec(),
        lines
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.to_string())
            .collect_vec(),
    )
}

#[aoc(day3, part1)]
pub fn first(input: &(Vec<String>, Vec<String>)) -> usize {
    let path1 = get_path(&input.0);
    let path2 = get_path(&input.1);
    let Point(x, y) = dedup_points(&path1)
        .intersection(&dedup_points(&path2))
        .min_by_key(|p| p.0.abs() + p.1.abs())
        .unwrap();
    (x.abs() + y.abs()) as usize
}

#[aoc(day3, part2)]
pub fn second(input: &(Vec<String>, Vec<String>)) -> usize {
    let path1 = get_path(&input.0);
    let path2 = get_path(&input.1);

    dedup_points(&path1)
        .intersection(&dedup_points(&path2))
        .map(|p| (steps_to_point(&path1, &p), steps_to_point(&path2, &p)))
        .map(|(steps1, steps2)| steps1 + steps2)
        .min()
        .unwrap()
}

fn dedup_points(path: &[Point]) -> HashSet<&Point> {
    path.iter().collect::<HashSet<_>>()
}

fn get_path<S: ToString>(seq: &[S]) -> Vec<Point> {
    seq.iter()
        .map(ToString::to_string)
        .fold(vec![], |mut acc, command| {
            let (t, mv) = command.split_at(1);
            let mv = mv.parse::<usize>().unwrap();
            for _ in 0..mv {
                acc.push(acc.last().unwrap_or(&Point(0, 0)).apply_move(t));
            }
            acc
        })
}

fn steps_to_point(path: &[Point], target: &Point) -> usize {
    path.iter().take_while(|&p| p != target).count() + 1
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point(i32, i32);

impl Point {
    fn apply_move(&self, t: &str) -> Self {
        match t {
            "R" => Self(self.0 + 1, self.1),
            "L" => Self(self.0 - 1, self.1),
            "D" => Self(self.0, self.1 - 1),
            "U" => Self(self.0, self.1 + 1),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(
        seq1,
        seq2,
        expected,
        case(vec!["R75","D30","R83","U83","L12","D49","R71","U7","L72"], vec!["U62","R66","U55","R34","D71","R55","D58","R83"], 159),
        case(vec!["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51"], vec!["U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"], 135),
        )
    ]
    fn part_one_examples(seq1: Vec<&str>, seq2: Vec<&str>, expected: usize) {
        let input = (
            seq1.iter().map(|&s| s.to_owned()).collect_vec(),
            seq2.iter().map(|&s| s.to_owned()).collect_vec(),
        );
        assert_eq!(first(&input), expected);
    }

    #[rstest_parametrize(
        seq1,
        seq2,
        expected,
        case(vec!["R75","D30","R83","U83","L12","D49","R71","U7","L72"], vec!["U62","R66","U55","R34","D71","R55","D58","R83"], 610),
        case(vec!["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51"], vec!["U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"], 410),
        )
    ]
    fn part_two_examples(seq1: Vec<&str>, seq2: Vec<&str>, expected: usize) {
        let input = (
            seq1.iter().map(|&s| s.to_owned()).collect_vec(),
            seq2.iter().map(|&s| s.to_owned()).collect_vec(),
        );
        assert_eq!(second(&input), expected);
    }
}
