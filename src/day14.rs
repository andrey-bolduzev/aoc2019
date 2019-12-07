use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Mul;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Reaction> {
    input.trim().lines().map(Reaction::new).collect_vec()
}

#[aoc(day14, part1)]
pub fn first(input: &Vec<Reaction>) -> usize {
    reduce_to_basic(input)
}

fn reduce_to_basic(factory: &Vec<Reaction>) -> usize {
    // todo decompose one level at a time
    let mut inputs: HashMap<Chemical, usize> = hashmap! { Chemical::new("FUEL") => 1 };
    while let Some(entry) = remove_non_basic(&mut inputs, &factory) {
        let (reaction, runs) = find_reaction_for(
            ReactionComponent {
                chemical: entry.0,
                quantity: entry.1,
            },
            &factory,
        );

        let ins = reaction.to_inputs(runs);
        for input in ins {
            *inputs.entry(input.chemical).or_insert(0) += input.quantity;
        }
    }

    let mut basic_inputs = inputs
        .iter()
        .map(|(k, v)| ReactionComponent {
            chemical: k.clone(),
            quantity: v.clone(),
        })
        .collect_vec();
    reduce_to_ore(&mut basic_inputs, factory)[0].quantity
}

fn remove_non_basic(
    inputs: &mut HashMap<Chemical, usize>,
    factory: &Vec<Reaction>,
) -> Option<(Chemical, usize)> {
    let not_basic = {
        let not = &inputs.iter().find(|(k, _)| !is_basic(k, factory));
        not.map(|(k, _)| k.clone())
    };
    if let Some(b) = not_basic {
        return inputs.remove_entry(&b);
    } else {
        return None;
    }
}

fn is_basic(chemical: &Chemical, factory: &Vec<Reaction>) -> bool {
    factory
        .iter()
        .find(|reaction| {
            &reaction.output.chemical == chemical
                && reaction.inputs.len() == 1
                && reaction.inputs[0].is_ore()
        })
        .is_some()
}

fn reduce_to_ore(
    inputs: &mut Vec<ReactionComponent>,
    factory: &Vec<Reaction>,
) -> Vec<ReactionComponent> {
    let inputs = collapse_groups(inputs.to_vec());
    if inputs.len() == 1 && inputs[0].is_ore() {
        return vec![ReactionComponent {
            chemical: Chemical::ore(),
            quantity: inputs[0].quantity,
        }];
    } else {
        let (mut ore, not_ore): (Vec<ReactionComponent>, Vec<ReactionComponent>) =
            inputs.into_iter().partition(|comp| comp.is_ore());
        let mut not_ore = not_ore
            .into_iter()
            .flat_map(|comp| {
                let (reaction, runs) = find_reaction_for(comp, factory);
                reaction.to_inputs(runs)
            })
            .collect_vec();
        let mut not_ore = reduce_to_ore(&mut not_ore, factory);
        not_ore.append(&mut ore);
        return not_ore;
    }
}

fn collapse_groups(input: Vec<ReactionComponent>) -> Vec<ReactionComponent> {
    let mut result = vec![];
    for (chem, quantities) in input
        .into_iter()
        .map(|comp| (comp.chemical, comp.quantity))
        .into_group_map()
    {
        result.push(ReactionComponent {
            chemical: chem,
            quantity: quantities.into_iter().sum(),
        })
    }

    result
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Chemical {
    name: String,
}

#[derive(Debug, Clone)]
pub struct Reaction {
    inputs: Vec<ReactionComponent>,
    output: ReactionComponent,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ReactionComponent {
    chemical: Chemical,
    quantity: usize,
}

impl Chemical {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    fn ore() -> Self {
        Self::new("ORE")
    }
}

impl ReactionComponent {
    fn new(component: &str) -> Self {
        let mut parts = component.trim().split_whitespace();
        let quantity: usize = parts.next().unwrap().parse().unwrap();
        let name = parts.next().unwrap();
        Self {
            chemical: Chemical::new(name),
            quantity,
        }
    }

    fn is_ore(&self) -> bool {
        self.chemical == Chemical::ore()
    }
}

impl Mul<usize> for ReactionComponent {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Self {
            quantity: self.quantity * rhs,
            ..self
        }
    }
}

impl Reaction {
    pub fn new(line: &str) -> Self {
        let mut split = line.split("=>");
        let inputs = collection_of_components(split.next().unwrap());
        let output = ReactionComponent::new(split.next().unwrap());
        Self { inputs, output }
    }
    fn is_producing(&self, target: &Chemical) -> bool {
        &self.output.chemical == target
    }

    fn runs_needed(&self, target_quantity: usize) -> usize {
        (target_quantity - 1) / self.output.quantity + 1
    }

    fn to_inputs(&self, runs: usize) -> Vec<ReactionComponent> {
        self.inputs
            .iter()
            .map(|input| input.clone() * runs)
            .collect()
    }
}

fn collection_of_components(components: &str) -> Vec<ReactionComponent> {
    components
        .trim()
        .split(",")
        .map(|s| ReactionComponent::new(s))
        .collect()
}

fn find_reaction_for<'a>(
    target: ReactionComponent,
    factory: &'a Vec<Reaction>,
) -> (&'a Reaction, usize) {
    let reaction = factory
        .iter()
        .find(|&reaction| reaction.is_producing(&target.chemical))
        .unwrap();
    (reaction, reaction.runs_needed(target.quantity))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runs_needed_with_producing_less_than_target() {
        let reaction = Reaction {
            inputs: Default::default(),
            output: ReactionComponent::new("1 A"),
        };
        assert_eq!(2, reaction.runs_needed(2));
    }

    #[test]
    fn test_runs_needed_with_producing_more_than_target() {
        let reaction = Reaction {
            inputs: Default::default(),
            output: ReactionComponent::new("2 A"),
        };
        assert_eq!(1, reaction.runs_needed(1));
    }

    #[test]
    fn test_runs_needed_with_producing_exactly_as_much_as_needed_target() {
        let reaction = Reaction {
            inputs: Default::default(),
            output: ReactionComponent::new("2 A"),
        };
        assert_eq!(1, reaction.runs_needed(2));
    }

    #[test]
    fn test_runs_needed_with_producing_with_remainder() {
        let reaction = Reaction {
            inputs: Default::default(),
            output: ReactionComponent::new("5 A"),
        };
        assert_eq!(1, reaction.runs_needed(2));
    }

    #[test]
    fn test_runs_needed_with_producing_with_remainder_opposite() {
        let reaction = Reaction {
            inputs: Default::default(),
            output: ReactionComponent::new("2 A"),
        };
        assert_eq!(3, reaction.runs_needed(5));
    }

    #[test]
    fn test_reduce_to_ore_for_single_ore_reaction() {
        let result = reduce_to_ore(
            &mut vec![ReactionComponent::new("1 ORE")],
            &vec![Reaction::new("1 ORE => 1 ORE")],
        );
        assert_eq!(vec![ReactionComponent::new("1 ORE")], result);
    }

    #[test]
    fn test_reduce_to_ore_for_single_more_complex_ore_reaction() {
        let result = reduce_to_ore(
            &mut vec![ReactionComponent::new("1 FUEL")],
            &vec![Reaction::new("2 ORE => 1 FUEL")],
        );
        assert_eq!(vec![ReactionComponent::new("2 ORE")], result);
    }

    #[test]
    fn test_reduce_to_ore_for_two_reactions() {
        let result = reduce_to_ore(
            &mut vec![ReactionComponent::new("1 FUEL")],
            &vec![
                Reaction::new("2 B => 1 FUEL"),
                Reaction::new("5 ORE => 2 B"),
            ],
        );
        assert_eq!(vec![ReactionComponent::new("5 ORE")], result);
    }

    #[test]
    fn collapse_groups_groups_ore_items() {
        let result = collapse_groups(vec![
            ReactionComponent::new("1 ORE"),
            ReactionComponent::new("2 ORE"),
        ]);
        assert_eq!(vec![ReactionComponent::new("3 ORE")], result);
    }

    #[test]
    fn collapse_groups_collapses_everything() {
        let result = collapse_groups(vec![
            ReactionComponent::new("1 ORE"),
            ReactionComponent::new("2 ORE"),
            ReactionComponent::new("1 A"),
            ReactionComponent::new("1 A"),
            ReactionComponent::new("1 B"),
        ]);
        assert!(result
            .iter()
            .find(|&comp| comp == &ReactionComponent::new("3 ORE"))
            .is_some());
        assert!(result
            .iter()
            .find(|&comp| comp == &ReactionComponent::new("2 A"))
            .is_some());
        assert!(result
            .iter()
            .find(|&comp| comp == &ReactionComponent::new("1 B"))
            .is_some());
    }

    #[test]
    fn ex0() {
        assert_eq!(
            31,
            first(&vec![
                Reaction::new("10 ORE => 10 A"),
                Reaction::new("1 ORE => 1 B"),
                Reaction::new("7 A, 1 B => 1 C"),
                Reaction::new("7 A, 1 C => 1 D"),
                Reaction::new("7 A, 1 D => 1 E"),
                Reaction::new("7 A, 1 E => 1 FUEL"),
            ])
        );
    }

    #[test]
    fn ex1() {
        assert_eq!(
            165,
            first(&vec![
                Reaction::new("9 ORE => 2 A"),
                Reaction::new("8 ORE => 3 B"),
                Reaction::new("7 ORE => 5 C"),
                Reaction::new("3 A, 4 B => 1 AB"),
                Reaction::new("5 B, 7 C => 1 BC"),
                Reaction::new("4 C, 1 A => 1 CA"),
                Reaction::new("2 AB, 3 BC, 4 CA => 1 FUEL"),
            ])
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            13312,
            first(&vec![
                Reaction::new("157 ORE => 5 NZVS"),
                Reaction::new("165 ORE => 6 DCFZ"),
                Reaction::new("44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL"),
                Reaction::new("12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ"),
                Reaction::new("179 ORE => 7 PSHF"),
                Reaction::new("177 ORE => 5 HKGWZ"),
                Reaction::new("7 DCFZ, 7 PSHF => 2 XJWVT"),
                Reaction::new("165 ORE => 2 GPVTF"),
                Reaction::new("3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"),
            ])
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            180697,
            first(&vec![
                Reaction::new("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG"),
                Reaction::new("17 NVRVD, 3 JNWZP => 8 VPVL"),
                Reaction::new("53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL"),
                Reaction::new("22 VJHF, 37 MNCFX => 5 FWMGM"),
                Reaction::new("139 ORE => 4 NVRVD"),
                Reaction::new("144 ORE => 7 JNWZP"),
                Reaction::new("5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC"),
                Reaction::new("5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV"),
                Reaction::new("145 ORE => 6 MNCFX"),
                Reaction::new("1 NVRVD => 8 CXFTF"),
                Reaction::new("1 VJHF, 6 MNCFX => 4 RFSQX"),
                Reaction::new("176 ORE => 6 VJHF"),
            ])
        );
    }

    #[test]
    #[ignore]
    fn ex4() {
        assert_eq!(
            2210736,
            first(&vec![
                Reaction::new("171 ORE => 8 CNZTR"),
                Reaction::new(
                    "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL"
                ),
                Reaction::new("114 ORE => 4 BHXH"),
                Reaction::new("14 VRPVC => 6 BMBT"),
                Reaction::new("6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL"),
                Reaction::new(
                    "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT"
                ),
                Reaction::new("15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW"),
                Reaction::new("13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW"),
                Reaction::new("5 BMBT => 4 WPTQ"),
                Reaction::new("189 ORE => 9 KTJDG"),
                Reaction::new("1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP"),
                Reaction::new("12 VRPVC, 27 CNZTR => 2 XDBXC"),
                Reaction::new("15 KTJDG, 12 BHXH => 5 XCVML"),
                Reaction::new("3 BHXH, 2 VRPVC => 7 MZWV"),
                Reaction::new("121 ORE => 7 VRPVC"),
                Reaction::new("7 XCVML => 6 RJRHP"),
                Reaction::new("5 BHXH, 4 VRPVC => 5 LTCX"),
            ])
        );
    }
}
