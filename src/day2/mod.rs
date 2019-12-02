pub fn first(input: &[usize], replacement1: usize, replacement2: usize) -> usize {
    let mut seq = input.to_vec();
    seq[1] = replacement1;
    seq[2] = replacement2;

    let mut code = input[0];
    let mut index = 0;

    while code != 99 {
        let operand1 = seq[seq[index + 1]];
        let operand2 = seq[seq[index + 2]];
        let result_index = seq[index + 3];
        match code {
            1 => seq[result_index] = operand1 + operand2,
            2 => seq[result_index] = operand1 * operand2,
            _ => panic!(),
        }
        index += 4;
        code = seq[index];
    }
    seq[0]
}

pub fn second(input: &[usize]) -> usize {
    for i in 0..=99 {
        for j in 0..=99 {
            if first(input, i, j) == 19690720 {
                return 100 * i + j;
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(
            input,
            expected,
            case(vec![1,0,0,0,99], 2),
            case(vec![2,3,0,3,99], 2),
            case(vec![2,4,4,5,99,0], 2),
            case(vec![1,1,1,4,99,5,6,0,99], 30)
        )
    ]
    fn part_one_examples(input: Vec<usize>, expected: usize) {
        assert_eq!(first(&input, 0, 0), expected);
    }
}
