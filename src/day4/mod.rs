use std::collections::HashMap;

pub fn first() -> usize {
    (367479..=893698_usize).filter(matches_first).count()
}

pub fn second() -> usize {
    (367479..=893698_usize).filter(matches_second).count()
}

fn matches_first(candidate: &usize) -> bool {
    let string = candidate.to_string();
    let mut iter = string.chars();
    let c0 = iter.next();
    let c1 = iter.next();
    let c2 = iter.next();
    let c3 = iter.next();
    let c4 = iter.next();
    let c5 = iter.next();

    c0 <= c1
        && c1 <= c2
        && c2 <= c3
        && c3 <= c4
        && c4 <= c5
        && contains_group_of_at_least_two(string)
}

fn matches_second(candidate: &usize) -> bool {
    matches_first(candidate) && contains_group_of_two(candidate.to_string())
}

fn contains_group_of_two(candidate: String) -> bool {
    check_contains_group(candidate, |&v| v == 2)
}

fn contains_group_of_at_least_two(candidate: String) -> bool {
    check_contains_group(candidate, |&v| v >= 2)
}

fn check_contains_group(candidate: String, check: impl FnMut(&usize) -> bool) -> bool {
    let mut map: HashMap<char, usize> = HashMap::new();
    for c in candidate.chars() {
        *map.entry(c).or_default() += 1;
    }
    map.values().any(check)
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(
        password,
        matches,
        case(111111, true),
        case(223450, false),
        case(123789, false)
    )]
    fn part_one_examples(password: usize, matches: bool) {
        assert!(matches_first(&password) == matches)
    }

    #[rstest_parametrize(
        password,
        matches,
        case(112233, true),
        case(123444, false),
        case(445556, true),
        case(111122, true)
    )]
    fn part_two_examples(password: usize, matches: bool) {
        assert!(matches_second(&password) == matches)
    }
}
