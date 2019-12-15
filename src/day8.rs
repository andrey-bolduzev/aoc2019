use itertools::Itertools;

const WIDTH: usize = 25;
const LAYER_LEN: usize = WIDTH * 6;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> String {
    input.trim().to_owned()
}

#[aoc(day8, part1)]
pub fn first(input: &String) -> usize {
    let vec: Vec<_> = input.chars().collect();
    let layers: Vec<_> = vec.chunks(LAYER_LEN).collect();
    let min_layer = layers
        .iter()
        .min_by_key(|x| x.iter().filter(|&&c| c == '0').count())
        .unwrap();
    let ones = min_layer.iter().filter(|&&c| c == '1').count();
    let twos = min_layer.iter().filter(|&&c| c == '2').count();
    ones * twos
}

#[aoc(day8, part2)]
pub fn second(input: &String) -> usize {
    let vec: Vec<_> = input.chars().collect();
    let layers: Vec<_> = vec.chunks(LAYER_LEN).collect();
    let mut output: Vec<_> = Vec::new();

    for i in 0..layers[0].len() {
        for layer in &layers {
            match layer[i] {
                '0' => {
                    output.push(' ');
                    break;
                }
                '1' => {
                    output.push('#');
                    break;
                }
                _ => {}
            }
        }
    }

    output
        .chunks(WIDTH)
        .map(|chunk| chunk.iter().join(""))
        .for_each(|row| println!("{}", row));
    0
}
