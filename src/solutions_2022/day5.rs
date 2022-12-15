use crate::Solution;

pub fn day5(input: &str) -> Solution {
    let mut stacks = vec![
        vec!['R', 'P', 'C', 'D', 'B', 'G'],
        vec!['H', 'V', 'G'],
        vec!['N', 'S', 'Q', 'D', 'J', 'P', 'M'],
        vec!['P', 'S', 'L', 'G', 'D', 'C', 'N', 'M'],
        vec!['J', 'B', 'N', 'C', 'P', 'F', 'L', 'S'],
        vec!['Q', 'B', 'D', 'Z', 'V', 'G', 'T', 'S'],
        vec!['B', 'Z', 'M', 'H', 'F', 'T', 'Q'],
        vec!['C', 'M', 'D', 'B', 'F'],
        vec!['F', 'C', 'Q', 'G'],
    ];

    let insts = input.trim().split("\n\n").nth(1).unwrap();
    for inst in insts.lines() {
        let mut iter = inst.split(' ');
        let n = iter.nth(1).unwrap().parse::<usize>().unwrap();
        let from = iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..n {
            let el = stacks[from].pop().unwrap();
            stacks[to].push(el);
        }
    }

    let mut res = String::new();
    for mut stack in stacks.into_iter() {
        res.push(stack.pop().unwrap());
    }
    Solution::String(res)
}

pub fn day5_part2(input: &str) -> Solution {
    let mut stacks = vec![
        vec!['R', 'P', 'C', 'D', 'B', 'G'],
        vec!['H', 'V', 'G'],
        vec!['N', 'S', 'Q', 'D', 'J', 'P', 'M'],
        vec!['P', 'S', 'L', 'G', 'D', 'C', 'N', 'M'],
        vec!['J', 'B', 'N', 'C', 'P', 'F', 'L', 'S'],
        vec!['Q', 'B', 'D', 'Z', 'V', 'G', 'T', 'S'],
        vec!['B', 'Z', 'M', 'H', 'F', 'T', 'Q'],
        vec!['C', 'M', 'D', 'B', 'F'],
        vec!['F', 'C', 'Q', 'G'],
    ];

    let insts = input.trim().split("\n\n").nth(1).unwrap();
    for inst in insts.lines() {
        let mut iter = inst.split(' ');
        let n = iter.nth(1).unwrap().parse::<usize>().unwrap();
        let from = iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        let mut to_place = vec![];
        for _ in 0..n {
            let el = stacks[from].pop().unwrap();
            to_place.push(el);
        }
        to_place.reverse();
        stacks[to].extend_from_slice(&to_place);
    }

    let mut res = String::new();
    for mut stack in stacks.into_iter() {
        res.push(stack.pop().unwrap());
    }
    Solution::String(res)
}
