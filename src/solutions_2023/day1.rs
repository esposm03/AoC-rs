use crate::Solution;

pub fn day1(input: &str) -> Solution {
    let mut sum = 0;

    for line in input.trim().split('\n') {
        let line = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();
        sum += 10 * line[0] + line[line.len() - 1];
    }

    Solution::Int(sum as i64)
}

pub fn day1_part2(input: &str) -> Solution {
    let mut sum = 0;

    for line in input.trim().split('\n') {
        let line = line.trim().to_string();
        let mut replaced = vec![];

        for i in 0..line.len() {
            replaced.push(if line[i..].strip_prefix("one").is_some() {
                '1'
            } else if line[i..].strip_prefix("two").is_some() {
                '2'
            } else if line[i..].strip_prefix("three").is_some() {
                '3'
            } else if line[i..].strip_prefix("four").is_some() {
                '4'
            } else if line[i..].strip_prefix("five").is_some() {
                '5'
            } else if line[i..].strip_prefix("six").is_some() {
                '6'
            } else if line[i..].strip_prefix("seven").is_some() {
                '7'
            } else if line[i..].strip_prefix("eight").is_some() {
                '8'
            } else if line[i..].strip_prefix("nine").is_some() {
                '9'
            } else {
                line.chars().nth(i).unwrap()
            });
        }

        let nums = replaced
            .iter()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();
        sum += 10 * nums[0] + nums[nums.len() - 1];
    }

    Solution::Int(sum as i64)
}

#[test]
#[cfg(test)]
fn test() {
    let input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
    ";
    assert_eq!(day1(input), Solution::Int(142));

    let input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    ";
    assert_eq!(day1_part2(input), Solution::Int(281));
}
