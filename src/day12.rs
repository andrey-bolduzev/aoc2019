use itertools::Itertools;
use permutator::Combination;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;
use std::ops::AddAssign;

#[aoc_generator(day12)]
pub fn input_generator(_input: &str) -> (HashSet<Moon>, usize) {
    (
        hashset! { Moon {
            position: Position(-15, 1, 4),
            velocity: Default::default(),
        },
        Moon {
            position: Position(1, -10, -8),
            velocity: Default::default(),
        },
        Moon {
            position: Position(-5, 4, 9),
            velocity: Default::default(),
        },
        Moon {
            position: Position(4, 6, -2),
            velocity: Default::default(),
        },
        },
        1000,
    )
}

#[aoc(day12, part1)]
pub fn first(input: &(HashSet<Moon>, usize)) -> usize {
    let mut moons = input.0.clone();
    for _ in 0..input.1 {
        moons = perform_step(moons);
    }
    moons.iter().map(|m| m.energy()).sum()
}

fn perform_step(moons: HashSet<Moon>) -> HashSet<Moon> {
    println!("before step {:?}", moons);
    println!();

    let mut before_velocity: HashMap<Position, Velocity> = HashMap::new();
    let before_vel_set = moons
        .into_iter()
        .collect_vec()
        .combination(2)
        .map(|vec| {
            let mut iter = vec.iter();
            let first = *iter.next().unwrap();
            let second = *iter.next().unwrap();
            MoonPair(first.clone(), second.clone())
        })
        .map(apply_gravity)
        .flat_map(|pair| vec![pair.0.clone(), pair.1.clone()])
        .collect_vec();

    for moon in before_vel_set {
        // println!("adding to map: {:?}", moon);
        *before_velocity.entry(moon.position).or_default() += moon.velocity;
    }

    // println!("map {:?}", before_velocity);

    before_velocity
        .into_iter()
        .map(|(position, velocity)| Moon { position, velocity })
        .map(|moon| {
            // println!("before velocity {:?}", moon);
            moon
        })
        .map(Moon::apply_velocity)
        .collect()
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
pub struct Moon {
    position: Position,
    velocity: Velocity,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position(i32, i32, i32);

#[derive(Debug, Hash, Clone, PartialEq, Eq, Default)]
struct Velocity(i32, i32, i32);

#[derive(Debug, Hash)]
struct MoonPair(Moon, Moon);

impl Moon {
    fn energy(&self) -> usize {
        self.position.energy() + self.velocity.energy()
    }

    fn apply_velocity(self) -> Self {
        Self {
            position: &self.position + &self.velocity,
            ..self
        }
    }
}

impl Position {
    fn energy(&self) -> usize {
        (self.0.abs() + self.1.abs() + self.2.abs()) as usize
    }
}

impl Velocity {
    fn energy(&self) -> usize {
        (self.0.abs() + self.1.abs() + self.2.abs()) as usize
    }
}

fn apply_gravity(pair: MoonPair) -> MoonPair {
    // println!();
    // println!("before gravity: {:?}", pair);
    let xdiff = (pair.0.position.0 - pair.1.position.0).signum();
    let ydiff = (pair.0.position.1 - pair.1.position.1).signum();
    let zdiff = (pair.0.position.2 - pair.1.position.2).signum();

    let new_pair = MoonPair(
        Moon {
            velocity: pair.0.velocity + Velocity(-xdiff, -ydiff, -zdiff),
            ..pair.0
        },
        Moon {
            velocity: pair.1.velocity + Velocity(xdiff, ydiff, zdiff),
            ..pair.1
        },
    );
    // println!("after gravity: {:?}", new_pair);
    // println!();
    new_pair
}

impl Add<&Velocity> for &Position {
    type Output = Position;

    fn add(self, other: &Velocity) -> Position {
        Position(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add for Velocity {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Velocity {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other
    }
}

impl PartialEq for MoonPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.1 == other.0 && self.0 == other.1
    }
}

impl Eq for MoonPair {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            51,
            first(&(
                hashset! {
                    Moon {
                        position: Position(-1, 0, 2),
                        velocity: Default::default(),
                    },
                    Moon {
                        position: Position(2, -10, -7),
                        velocity: Default::default(),
                    },
                    Moon {
                        position: Position(4, -8, 8),
                        velocity: Default::default(),
                    },
                    Moon {
                        position: Position(3, 5, -1),
                        velocity: Default::default(),
                    },
                },
                0,
            ))
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            59,
            first(&(
                hashset! {
                    Moon {
                        position: Position(-1, 0, 2),
                        velocity: Default::default(),
                    },
                    Moon {
                        position: Position(2, -10, -7),
                        velocity: Default::default(),
                    },
                    Moon {
                        position: Position(4, -8, 8),
                        velocity: Default::default(),
                    },
                    Moon {
                        position: Position(3, 5, -1),
                        velocity: Default::default(),
                    },
                },
                1,
            ))
        )
    }

    #[test]
    fn ex3() {
        assert_eq!(
            65,
            first(&(
                hashset! {
                    Moon {
                        position: Position(-1, 0, 2),
                        velocity: Default::default(),
                    },
                    Moon {
                        position: Position(2, -10, -7),
                        velocity: Default::default(),
                    },
                    Moon {
                        position: Position(4, -8, 8),
                        velocity: Default::default(),
                    },
                    Moon {
                        position: Position(3, 5, -1),
                        velocity: Default::default(),
                    },
                },
                2,
            ))
        )
    }

    #[test]
    fn ex_per_st() {
        assert_eq!(
            hashset! {
                Moon {
                    position: Position(2, -1, 1),
                    velocity: Velocity(3, -1, -1)
                },
                Moon {
                    position: Position(3, -7, -4),
                    velocity: Velocity(1, 3, 3)
                },
                Moon {
                    position: Position(1, -7, 5),
                    velocity: Velocity(-3, 1, -3),
                },
                Moon {
                    position: Position(2, 2, 0),
                    velocity: Velocity(-1, -3, 1),
                },
            },
            perform_step(hashset! {
                Moon {
                    position: Position(-1, 0, 2),
                    velocity: Default::default(),
                },
                Moon {
                    position: Position(2, -10, -7),
                    velocity: Default::default(),
                },
                Moon {
                    position: Position(4, -8, 8),
                    velocity: Default::default(),
                },
                Moon {
                    position: Position(3, 5, -1),
                    velocity: Default::default(),
                },
            },)
        )
    }

    #[test]
    fn ex_per_2_st() {
        assert_eq!(
            hashset! {
                Moon {
                    position: Position(5,-3,-1),
                    velocity: Velocity(3,-2,-2)
                },
                Moon {
                    position: Position(1, -2, 2),
                    velocity: Velocity(-2, 5, 6)
                },
                Moon {
                    position: Position(1, -4, -1),
                    velocity: Velocity(0, 3, -6),
                },
                Moon {
                    position: Position(1, -4, 2),
                    velocity: Velocity(-1, -6, 2),
                },
            },
            perform_step(perform_step(hashset! {
                Moon {
                    position: Position(-1, 0, 2),
                    velocity: Default::default(),
                },
                Moon {
                    position: Position(2, -10, -7),
                    velocity: Default::default(),
                },
                Moon {
                    position: Position(4, -8, 8),
                    velocity: Default::default(),
                },
                Moon {
                    position: Position(3, 5, -1),
                    velocity: Default::default(),
                },
            },))
        )
    }

    #[test]
    fn test_apply_gravity() {
        let new_pair = apply_gravity(MoonPair(
            Moon {
                position: Position(1, -1, 0),
                velocity: Default::default(),
            },
            Moon {
                position: Position(0, 0, 0),
                velocity: Default::default(),
            },
        ));

        assert_eq!(
            MoonPair(
                Moon {
                    position: Position(1, -1, 0),
                    velocity: Velocity(-1, 1, 0)
                },
                Moon {
                    position: Position(0, 0, 0),
                    velocity: Velocity(1, -1, 0)
                },
            ),
            new_pair
        )
    }
}
