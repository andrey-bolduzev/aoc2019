#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn first(input: &[i32]) -> i32 {
    solve(input, 1)
}

#[aoc(day5, part2)]
pub fn second(input: &[i32]) -> i32 {
    solve(input, 5)
}

fn solve(input: &[i32], init: i32) -> i32 {
    let mut seq = input.to_vec();

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
                seq[target] = last_output;
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
