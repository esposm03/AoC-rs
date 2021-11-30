use crate::SolutionType;
use std::collections::HashMap;

pub fn day4(input: &str) -> SolutionType {
    let mut iter = input.trim().split("-");
    let n1 = iter.next().unwrap().parse().unwrap();
    let n2 = iter.next().unwrap().parse().unwrap();
    let mut count = 0;

    for i0 in 0..10 {
        for i1 in 0..10 {
            for i2 in 0..10 {
                for i3 in 0..10 {
                    for i4 in 0..10 {
                        for i5 in 0..10 {
                            if pw_valid_part1(n1..=n2, [i0, i1, i2, i3, i4, i5]) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    SolutionType::Int(count)
}

pub fn day4_part2(input: &str) -> SolutionType {
    let mut iter = input.trim().split('-');
    let n1 = iter.next().unwrap().parse().unwrap();
    let n2 = iter.next().unwrap().parse().unwrap();
    let mut count = 0;

    for i0 in 0..10 {
        for i1 in 0..10 {
            for i2 in 0..10 {
                for i3 in 0..10 {
                    for i4 in 0..10 {
                        for i5 in 0..10 {
                            if pw_valid_part2(n1..=n2, [i0, i1, i2, i3, i4, i5]) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    SolutionType::Int(count)
}

fn pw_valid_part1(range: std::ops::RangeInclusive<u32>, from: [u8; 6]) -> bool {
    if !(from[0] <= from[1]
        && from[1] <= from[2]
        && from[2] <= from[3]
        && from[3] <= from[4]
        && from[4] <= from[5])
    {
        return false;
    }

    if !(from[0] == from[1]
        || from[1] == from[2]
        || from[2] == from[3]
        || from[3] == from[4]
        || from[4] == from[5])
    {
        return false;
    }

    let number = from[5] as u32 * 10u32.pow(0)
        + from[4] as u32 * 10u32.pow(1)
        + from[3] as u32 * 10u32.pow(2)
        + from[2] as u32 * 10u32.pow(3)
        + from[1] as u32 * 10u32.pow(4)
        + from[0] as u32 * 10u32.pow(5);

    range.contains(&number)
}

fn pw_valid_part2(range: std::ops::RangeInclusive<u32>, from: [u8; 6]) -> bool {
    // Here we check that each digit is greater or equal to the next one.
    if !(from[0] <= from[1]
        && from[1] <= from[2]
        && from[2] <= from[3]
        && from[3] <= from[4]
        && from[4] <= from[5])
    {
        return false;
    }

    // Here we check that there are two equal digits next to each other.
    let mut digits = HashMap::<u8, u8>::new();
    for i in &from {
        digits.insert(*i, *digits.get(i).unwrap_or(&0) + 1);
    }
    if !digits.values().any(|val| *val == 2) {
        return false;
    }

    // Here we check that the number is within the range
    let number = from[5] as u32 * 10u32.pow(0)
        + from[4] as u32 * 10u32.pow(1)
        + from[3] as u32 * 10u32.pow(2)
        + from[2] as u32 * 10u32.pow(3)
        + from[1] as u32 * 10u32.pow(4)
        + from[0] as u32 * 10u32.pow(5);
    range.contains(&number)
}
