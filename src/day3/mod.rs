use std::collections::HashSet;
use std::iter::Iterator;

pub fn first<S: ToString>(seq1: &[S], seq2: &[S]) -> usize {
    let path1 = get_path(seq1);
    let path2 = get_path(seq2);
    let Point(x, y) = dedup_points(&path1)
        .intersection(&dedup_points(&path2))
        .min_by_key(|p| p.0.abs() + p.1.abs())
        .unwrap();
    (x.abs() + y.abs()) as usize
}

pub fn second<S: ToString>(seq1: &[S], seq2: &[S]) -> usize {
    let path1 = get_path(seq1);
    let path2 = get_path(seq2);

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
        case(&["R75","D30","R83","U83","L12","D49","R71","U7","L72"], &["U62","R66","U55","R34","D71","R55","D58","R83"], 159),
        case(&["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51"], &["U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"], 135),
        )
    ]
    fn part_one_examples(seq1: &[&str], seq2: &[&str], expected: usize) {
        assert_eq!(first(seq1, seq2), expected);
    }

    #[rstest_parametrize(
        seq1,
        seq2,
        expected,
        case(&["R75","D30","R83","U83","L12","D49","R71","U7","L72"], &["U62","R66","U55","R34","D71","R55","D58","R83"], 610),
        case(&["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51"], &["U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"], 410),
        )
    ]
    fn part_two_examples(seq1: &[&str], seq2: &[&str], expected: usize) {
        assert_eq!(second(seq1, seq2), expected);
    }
}
