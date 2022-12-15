use crate::Solution;

pub fn day1(input: &str) -> Solution {
    let vec = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>();
    let vec = vec.split(|line| line.is_empty());

    let mut calories = vec![];
    for group in vec {
        calories.push(
            group
                .iter()
                .map(|line| line.parse::<i64>().unwrap())
                .sum::<i64>(),
        );
    }

    Solution::Int(*calories.iter().max().unwrap())
}

pub fn day1_part2(input: &str) -> Solution {
    let vec = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>();
    let vec = vec.split(|line| line.is_empty());

    let mut calories = vec![];
    for group in vec {
        calories.push(
            group
                .iter()
                .map(|line| line.parse::<i64>().unwrap())
                .sum::<i64>(),
        );
    }

    calories.sort_unstable();
    calories.reverse();
    Solution::Int(calories[0] + calories[1] + calories[2])
}
