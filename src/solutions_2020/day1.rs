use crate::Solution;

pub fn day1(input: &str) -> Solution {
    let input_numbers = input
        .lines()
        .map(|s| s.parse().expect("Can't parse a number"))
        .collect::<Vec<i64>>();

    for i in 0..input_numbers.len() {
        for j in 0..input_numbers.len() {
            if i == j {
                continue;
            }

            if input_numbers[i] + input_numbers[j] == 2020 {
                return Some(input_numbers[i] * input_numbers[j]).into();
            }
        }
    }

    None.into()
}

pub fn day1_part2(input: &str) -> Solution {
    let input_numbers = input
        .lines()
        .map(|s| s.parse().expect("Can't parse a number"))
        .collect::<Vec<i64>>();

    for i in 0..input_numbers.len() {
        for j in 0..input_numbers.len() {
            for k in 0..input_numbers.len() {
                if i == j || j == k {
                    continue;
                }

                if input_numbers[i] + input_numbers[j] + input_numbers[k] == 2020 {
                    return Some(input_numbers[i] * input_numbers[j] * input_numbers[k]).into();
                }
            }
        }
    }

    None.into()
}

#[cfg(test)]
#[test]
fn test() {
    let input = "1721\n979\n366\n299\n675\n1456";

    assert_eq!(day1(input), Some(514579).into());
    assert_eq!(day1_part2(input), Some(241861950).into());
}
