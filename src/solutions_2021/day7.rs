use crate::Solution;

fn sum_factorial(n: i64) -> i64 {
    (1..=n).sum()
}

pub fn day7(input: &str) -> Solution {
    let positions = input
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i64>>();

    let range = positions.iter().max().unwrap() - positions.iter().min().unwrap();
    (0..range)
        .map(|pos| positions.iter().map(|i| (pos - i).abs()).sum::<i64>())
        .min()
        .unwrap()
        .into()
}

pub fn day7_part2(input: &str) -> Solution {
    let positions = input
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i64>>();

    let range = positions.iter().max().unwrap() - positions.iter().min().unwrap();
    (0..range)
        .map(|pos| {
            positions
                .iter()
                .map(|i| (pos - i).abs())
                .map(sum_factorial)
                .sum::<i64>()
        })
        .min()
        .unwrap()
        .into()
}

#[test]
#[cfg(test)]
fn test() {
    assert_eq!(day7("16,1,2,0,4,2,7,1,2,14"), 37_i64.into());
    assert_eq!(day7_part2("16,1,2,0,4,2,7,1,2,14"), 168_i64.into());
}
