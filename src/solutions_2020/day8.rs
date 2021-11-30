use crate::SolutionType;
use std::collections::HashSet;

pub fn day8_part2(input: &str) -> SolutionType {
    let mut program = input.lines().map(parse_line).collect::<Vec<_>>();
    let jmp_indices = program
        .iter()
        .enumerate()
        .filter(|(_, (op, _))| *op == "jmp")
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let nop_indices = program
        .iter()
        .enumerate()
        .filter(|(_, (op, _))| *op == "nom")
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    for i in jmp_indices {
        program[i].0 = "nop";
        if let Ok(acc) = run_prog(&program) {
            return acc.into();
        }
        program[i].0 = "jmp";
    }
    for i in nop_indices {
        program[i].0 = "jmp";
        if let Ok(acc) = run_prog(&program) {
            return acc.into();
        }
        program[i].0 = "nop";
    }

    unreachable!();
}

pub fn day8(input: &str) -> SolutionType {
    let program = input.lines().map(parse_line).collect::<Vec<_>>();
    run_prog(&program).unwrap_err().into()
}

fn run_prog(program: &[(&str, i64)]) -> Result<i64, i64> {
    let mut acc: i64 = 0;
    let mut ins_ptr: i64 = 0;

    let mut run = HashSet::new();

    while !run.contains(&ins_ptr) && ins_ptr as usize != program.len() {
        run.insert(ins_ptr);

        match program[ins_ptr as usize].0 {
            "nop" => ins_ptr += 1,
            "acc" => {
                acc += program[ins_ptr as usize].1;
                ins_ptr += 1;
            }
            "jmp" => ins_ptr += program[ins_ptr as usize].1,
            _ => unreachable!(),
        }
    }

    if ins_ptr as usize == program.len() {
        Ok(acc)
    } else {
        Err(acc)
    }
}

fn parse_line(i: &str) -> (&str, i64) {
    let mut a = i.split(' ');
    (a.next().unwrap(), a.next().unwrap().parse().unwrap())
}
