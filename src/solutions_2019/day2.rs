use crate::SolutionType;

pub fn day2(input: &str) -> SolutionType {
    get_index_at(&vec![0], 0);

    let mut memory = input
        .split(',')
        .filter(|i| *i != "\n")
        .map(|i| i.replace("\n", ""))
        .map(|i| i.parse::<usize>().expect(&format!("Can't parse {}", i)))
        .collect::<Vec<_>>();
    let mut position = 0usize;

    memory[1] = 12;
    memory[2] = 2;

    loop {
        match *memory.get(position).unwrap() {
            1 => add(&mut memory, position).unwrap(),
            2 => mul(&mut memory, position).unwrap(),
            99 => {
                return memory[0].into();
            }
            _ => unreachable!(),
        }

        position += 4;
    }

    fn add(mem: &mut Vec<usize>, index: usize) -> Option<()> {
        let sum = get_index_at(&mem, index + 1) + get_index_at(&mem, index + 2);
        let pos: usize = *mem.get(index + 3).unwrap();
        mem[pos] = sum;

        Some(())
    }

    fn mul(mem: &mut Vec<usize>, index: usize) -> Option<()> {
        let mul = get_index_at(&mem, index + 1) * get_index_at(&mem, index + 2);
        let pos: usize = *mem.get(index + 3).unwrap();
        mem[pos] = mul;

        Some(())
    }

    fn get_index_at(mem: &Vec<usize>, position: usize) -> usize {
        let index = *mem.get(position).unwrap();
        *mem.get(index).unwrap()
    }
}

pub fn day2_part2(input: &str) -> SolutionType {
    get_index_at(&vec![0], 0);

    let memory = input
        .split(',')
        .filter(|i| *i != "\n")
        .map(|i| i.replace("\n", ""))
        .map(|i| i.parse::<usize>().expect(&format!("Can't parse {}", i)))
        .collect::<Vec<_>>();

    for i1 in 0..100 {
        for i2 in 0..100 {
            let mut program_memory = memory.clone();
            program_memory[1] = i1;
            program_memory[2] = i2;
            if run_program(program_memory) == 19690720 {
                return (100 * i1 + i2).into();
            }
        }
    }

    unreachable!();
}

fn run_program(mut memory: Vec<usize>) -> usize {
    let mut instruction_pointer = 0;

    loop {
        match *memory.get(instruction_pointer).unwrap() {
            1 => add(&mut memory, instruction_pointer).unwrap(),
            2 => mul(&mut memory, instruction_pointer).unwrap(),
            99 => {
                return *memory.get(0).unwrap();
            }
            _ => unreachable!(),
        }

        instruction_pointer += 4;
    }
}

fn add(mem: &mut Vec<usize>, index: usize) -> Option<()> {
    let sum = get_index_at(&mem, index + 1) + get_index_at(&mem, index + 2);
    let pos: usize = *mem.get(index + 3).unwrap();
    mem[pos] = sum;

    Some(())
}

fn mul(mem: &mut Vec<usize>, index: usize) -> Option<()> {
    let mul = get_index_at(&mem, index + 1) * get_index_at(&mem, index + 2);
    let pos: usize = *mem.get(index + 3).unwrap();
    mem[pos] = mul;

    Some(())
}

fn get_index_at(mem: &Vec<usize>, position: usize) -> usize {
    let index = *mem.get(position).unwrap();
    *mem.get(index).unwrap()
}
