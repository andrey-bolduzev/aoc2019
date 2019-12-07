use std::collections::HashSet;

pub fn first(input: &[i32]) -> i32 {
    let mut outputs = Vec::new();
    for phase_config in 01234..=43210 {
        let phase_config = phase_config.to_string();
        let phase_config = if phase_config.len() == 4 {
            "0".to_string() + &phase_config
        } else {
            phase_config
        };
        let phases: Vec<i32> = phase_config
            .chars()
            .map(|x| x.to_string().parse().unwrap())
            .collect();

        if phases.iter().any(|x| x >= &5) {
            continue;
        }

        if phases.iter().copied().collect::<HashSet<i32>>().len() < 5 {
            continue;
        }
        outputs.push((phase_config, run_config(input, &phases, false)));
    }
    outputs.iter().max_by_key(|x| x.1).map(|x| x.1).unwrap()
}

pub fn second(input: &[i32]) -> i32 {
    let mut outputs = Vec::new();
    for phase_config in 56789..=98765 {
        let phase_config = phase_config.to_string();
        let phases: Vec<i32> = phase_config
            .chars()
            .map(|x| x.to_string().parse().unwrap())
            .collect();

        if phases.iter().any(|x| x < &5) {
            continue;
        }

        if phases.iter().copied().collect::<HashSet<i32>>().len() < 5 {
            continue;
        }

        let mut result1 = process2(input, 0, phases[0], 0);
        let mut result2 = process2(input, result1.output, phases[1], 0);
        let mut result3 = process2(input, result2.output, phases[2], 0);
        let mut result4 = process2(input, result3.output, phases[3], 0);
        let mut result5 = process2(input, result4.output, phases[4], 0);

        loop {
            result1 = process2(input, result5.output, 0, result1.resume_index);
            result2 = process2(input, result1.output, 0, result2.resume_index);
            result3 = process2(input, result2.output, 0, result3.resume_index);
            result4 = process2(input, result3.output, 0, result4.resume_index);
            result5 = process2(input, result4.output, 0, result5.resume_index);
            if result5.is_final {
                break;
            }
        }
        outputs.push((phase_config, result5.output));
    }
    outputs.iter().max_by_key(|x| x.1).map(|x| x.1).unwrap()
}

fn run_config(input: &[i32], phase_config: &[i32], resuming: bool) -> i32 {
    let mut last_output = 0;
    for phase in phase_config {
        last_output = process(input, last_output, *phase, resuming);
    }
    last_output
}

fn process(input: &[i32], init: i32, phase: i32, resuming: bool) -> i32 {
    let mut seq = input.to_vec();
    let mut is_first = !resuming;

    let mut index = 0;
    let mut last_output = init;

    loop {
        let itype = seq[index] % 100;
        match itype {
            1 => {
                let (op1, op2, target) = read_3_args(&seq, &mut index);
                seq[target] = op1 + op2;
                index += 1;
            }
            2 => {
                let (op1, op2, target) = read_3_args(&seq, &mut index);
                seq[target] = op1 * op2;
                index += 1;
            }
            3 => {
                let target = inc_and_read_u(&seq, &mut index);
                seq[target] = if is_first { phase } else { last_output };
                is_first = false;
                index += 1;
            }
            4 => {
                let source = inc_and_read_u(&seq, &mut index);
                last_output = seq[source];
                index += 1;
            }
            5 => {
                let (op1, op2) = read_2_args(&seq, &mut index);
                if op1 != 0 {
                    index = op2 as usize;
                } else {
                    index += 1;
                }
            }
            6 => {
                let (op1, op2) = read_2_args(&seq, &mut index);
                if op1 == 0 {
                    index = op2 as usize;
                } else {
                    index += 1;
                }
            }
            7 => {
                let (op1, op2, target) = read_3_args(&seq, &mut index);
                if op1 < op2 {
                    seq[target] = 1;
                } else {
                    seq[target] = 0;
                }
                index += 1;
            }
            8 => {
                let (op1, op2, target) = read_3_args(&seq, &mut index);
                if op1 == op2 {
                    seq[target] = 1;
                } else {
                    seq[target] = 0;
                }
                index += 1;
            }
            99 => break,
            _ => panic!(),
        }
    }
    last_output
}

struct ProcessResult {
    output: i32,
    resume_index: usize,
    is_final: bool,
}

fn process2(input: &[i32], init: i32, phase: i32, resume_index: usize) -> ProcessResult {
    let mut seq = input.to_vec();
    let mut is_first = resume_index == 0;

    let mut index = resume_index;
    let mut last_output = init;

    loop {
        let itype = seq[index] % 100;
        match itype {
            1 => {
                let (op1, op2, target) = read_3_args(&seq, &mut index);
                seq[target] = op1 + op2;
                index += 1;
            }
            2 => {
                let (op1, op2, target) = read_3_args(&seq, &mut index);
                seq[target] = op1 * op2;
                index += 1;
            }
            3 => {
                let target = inc_and_read_u(&seq, &mut index);
                seq[target] = if is_first { phase } else { last_output };
                is_first = false;
                index += 1;
            }
            4 => {
                let source = inc_and_read_u(&seq, &mut index);
                last_output = seq[source];
                index += 1;
                return ProcessResult {
                    output: last_output,
                    resume_index: index,
                    is_final: false,
                };
            }
            5 => {
                let (op1, op2) = read_2_args(&seq, &mut index);
                if op1 != 0 {
                    index = op2 as usize;
                } else {
                    index += 1;
                }
            }
            6 => {
                let (op1, op2) = read_2_args(&seq, &mut index);
                if op1 == 0 {
                    index = op2 as usize;
                } else {
                    index += 1;
                }
            }
            7 => {
                let (op1, op2, target) = read_3_args(&seq, &mut index);
                if op1 < op2 {
                    seq[target] = 1;
                } else {
                    seq[target] = 0;
                }
                index += 1;
            }
            8 => {
                let (op1, op2, target) = read_3_args(&seq, &mut index);
                if op1 == op2 {
                    seq[target] = 1;
                } else {
                    seq[target] = 0;
                }
                index += 1;
            }
            99 => break,
            _ => panic!(),
        }
    }
    ProcessResult {
        output: last_output,
        resume_index: 0,
        is_final: true,
    }
}

fn read_3_args(seq: &[i32], index: &mut usize) -> (i32, i32, usize) {
    let (op1, op2) = read_2_args(seq, index);
    let target = inc_and_read_u(&seq, index);
    (op1, op2, target)
}

fn read_2_args(seq: &[i32], index: &mut usize) -> (i32, i32) {
    let code = seq[*index];

    let op1_type = code / 100 % 10;
    let op2_type = code / 1000 % 10;

    (
        resolve_operand(&seq, inc_and_read(&seq, index), op1_type),
        resolve_operand(&seq, inc_and_read(&seq, index), op2_type),
    )
}

fn inc_and_read(seq: &[i32], index: &mut usize) -> i32 {
    *index += 1;
    seq[*index]
}

fn inc_and_read_u(seq: &[i32], index: &mut usize) -> usize {
    inc_and_read(seq, index) as usize
}

fn resolve_operand(seq: &[i32], op: i32, op_type: i32) -> i32 {
    if op_type == 0 {
        seq[op as usize]
    } else {
        op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part_1() {
        assert_eq!(
            43210,
            run_config(
                &[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                &[4, 3, 2, 1, 0],
                false
            )
        )
    }

    #[test]
    fn first_part_2() {
        assert_eq!(
            54321,
            run_config(
                &[
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                &[0, 1, 2, 3, 4],
                false
            )
        )
    }

    #[test]
    fn first_part_3() {
        assert_eq!(
            65210,
            run_config(
                &[
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                &[1, 0, 4, 3, 2],
                false
            )
        )
    }

    #[test]
    fn first_part_4() {
        assert_eq!(
            43210,
            first(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],)
        )
    }

    #[test]
    fn first_part_5() {
        assert_eq!(
            54321,
            first(&[
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ],)
        )
    }

    #[test]
    fn first_part_6() {
        assert_eq!(
            65210,
            first(&[
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ],)
        )
    }
}
