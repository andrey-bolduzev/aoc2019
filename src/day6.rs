use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|l| l.split(')'))
        .map(|mut i| (i.next().unwrap().to_string(), i.next().unwrap().to_string()))
        .collect::<Vec<_>>()
}

#[aoc(day6, part1)]
pub fn first(pairs: &[(String, String)]) -> usize {
    build_extended(pairs).values().map(|x| x.len()).sum()
}

#[aoc(day6, part2)]
pub fn second(pairs: &[(String, String)]) -> usize {
    let map = build_extended(pairs);
    let you_nodes: HashSet<_> = map["YOU"].iter().collect();
    let san_nodes: HashSet<_> = map["SAN"].iter().collect();
    you_nodes.symmetric_difference(&san_nodes).count()
}

fn build_extended(pairs: &[(String, String)]) -> HashMap<String, Vec<String>> {
    let mut links: HashMap<String, Vec<String>> = HashMap::new();
    for (orbit, orbiter) in pairs {
        links
            .entry(orbiter.clone())
            .or_default()
            .push(orbit.clone());
    }
    let mut extended: HashMap<String, Vec<String>> = HashMap::new();
    for key in links.keys() {
        extended.insert(key.clone(), extend_nodes(key.clone(), &links));
    }
    extended
}

fn extend_nodes(node: String, map: &HashMap<String, Vec<String>>) -> Vec<String> {
    map.get(&node)
        .unwrap_or(&vec![])
        .iter()
        .flat_map(|l| {
            let mut vec = extend_nodes(l.to_string(), map);
            vec.push(l.to_string());
            vec
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        assert_eq!(
            42,
            first(&[
                (String::from("COM"), String::from("B")),
                (String::from("B"), String::from("C")),
                (String::from("C"), String::from("D")),
                (String::from("D"), String::from("E")),
                (String::from("E"), String::from("F")),
                (String::from("B"), String::from("G")),
                (String::from("G"), String::from("H")),
                (String::from("D"), String::from("I")),
                (String::from("E"), String::from("J")),
                (String::from("J"), String::from("K")),
                (String::from("K"), String::from("L")),
            ])
        )
    }

    #[test]
    fn second_part() {
        assert_eq!(
            4,
            second(&[
                (String::from("COM"), String::from("B")),
                (String::from("B"), String::from("C")),
                (String::from("C"), String::from("D")),
                (String::from("D"), String::from("E")),
                (String::from("E"), String::from("F")),
                (String::from("B"), String::from("G")),
                (String::from("G"), String::from("H")),
                (String::from("D"), String::from("I")),
                (String::from("E"), String::from("J")),
                (String::from("J"), String::from("K")),
                (String::from("K"), String::from("L")),
                (String::from("K"), String::from("YOU")),
                (String::from("I"), String::from("SAN")),
            ])
        )
    }
}
