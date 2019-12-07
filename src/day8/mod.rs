use itertools::Itertools;

const WIDTH: usize = 25;
const LAYER_LEN: usize = WIDTH * 6;

pub fn first(input: String) -> usize {
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

pub fn second(input: String) {
    let vec: Vec<_> = input.chars().collect();
    let layers: Vec<_> = vec.chunks(LAYER_LEN).collect();
    let mut output: Vec<_> = Vec::new();

    for i in 0..layers[0].len() {
        for layer in &layers {
            match layer[i] {
                '0' => {
                    output.push('_');
                    break;
                }
                '1' => {
                    output.push('1');
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
}
