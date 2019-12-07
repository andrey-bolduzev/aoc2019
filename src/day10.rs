use itertools::Itertools;
use permutator::Combination;
use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> String {
    input.trim().to_owned()
}

#[aoc(day10, part1)]
pub fn first(input: &String) -> usize {
    let asteroids = collect_asteroids(input.to_owned());

    let combinations = asteroids.combination(3).collect_vec();

    let mut map = HashMap::new();
    for combination in combinations {
        let mut iter = combination.iter();
        let first = iter.next().unwrap();
        let target = iter.next().unwrap();
        let blocker = iter.next().unwrap();
        // println!("{:?}, {:?}, {:?}", first, target, blocker);
        if first.is_blocking(target, blocker) {
            *map.entry(first.clone()).or_insert(asteroids.len()) -= 1;
        }
    }
    *map.values().max().unwrap()
}

fn collect_asteroids(input: String) -> Vec<Asteroid> {
    let lines = input.trim().lines().map(|l| l.trim());
    let mut candidates = vec![];
    for (row, line) in lines.enumerate() {
        for (column, item) in line.chars().enumerate() {
            if item == '#' {
                candidates.push(Asteroid {
                    x: column as i32,
                    y: row as i32,
                })
            }
        }
    }
    candidates
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    fn is_blocking(&self, other: &Asteroid, blocker: &Asteroid) -> bool {
        self.distance_to(blocker) == blocker.distance_to(other)
    }

    fn distance_to(&self, other: &Asteroid) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(33, first(&"......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####".to_owned()));
    }

    #[test]
    #[ignore]
    fn ex2() {
        assert_eq!(35, first(&"#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.".to_owned()));
    }
}
