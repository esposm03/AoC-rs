use crate::Solution;

pub fn day9(input: &str) -> Solution {
    let preamble = 25;
    let input = input
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for i in preamble + 1..input.len() {
        if !any_sum(input[i], &input[i - preamble..i]) {
            return input[i].into();
        }
    }

    unreachable!();
}

pub fn day9_part2(input: &str) -> Solution {
    let invalid: Solution = day9(input);

    let input = input
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for start in 0..input.len() {
        for end in start + 1..input.len() {
            let slice = &input[start..end];
            let sum: Solution = slice.iter().sum::<i64>().into();
            if sum == invalid {
                return (slice.iter().max().unwrap() + slice.iter().min().unwrap()).into();
            }
        }
    }

    unreachable!();
}

fn any_sum(number: i64, input: &[i64]) -> bool {
    for i in input {
        for j in input {
            let res = i + j == number;
            if res && i != j {
                return true;
            }
        }
    }

    false
}
