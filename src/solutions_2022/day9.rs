use std::{cmp::Ordering, collections::HashSet};

use crate::SolutionType;

pub fn day9(input: &str) -> SolutionType {
    let mut head = (0i32, 0i32);
    let mut tail = (0i32, 0i32);
    let mut visited = HashSet::new();
    visited.insert(tail);

    for line in input.trim().lines() {
        let line = line.trim();
        let amnt = line[2..].parse::<u32>().unwrap();

        for _ in 0..amnt {
            match line.chars().next().unwrap() {
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                'L' => head.0 -= 1,
                'R' => head.0 += 1,
                _ => unreachable!(),
            }

            if head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1 {
                continue;
            }

            match (head.0.cmp(&tail.0), head.1.cmp(&tail.1)) {
                // Same row
                (Ordering::Less, Ordering::Equal) => tail.0 -= 1,
                (Ordering::Greater, Ordering::Equal) => tail.0 += 1,

                // Same column
                (Ordering::Equal, Ordering::Less) => tail.1 -= 1,
                (Ordering::Equal, Ordering::Greater) => tail.1 += 1,

                // Diagonals
                (Ordering::Less, Ordering::Less) => {
                    tail.0 -= 1;
                    tail.1 -= 1;
                }
                (Ordering::Less, Ordering::Greater) => {
                    tail.0 -= 1;
                    tail.1 += 1;
                }
                (Ordering::Greater, Ordering::Less) => {
                    tail.0 += 1;
                    tail.1 -= 1;
                }
                (Ordering::Greater, Ordering::Greater) => {
                    tail.0 += 1;
                    tail.1 += 1;
                }

                // Overlapping
                (Ordering::Equal, Ordering::Equal) => {}
            }

            visited.insert(tail);
        }
    }

    visited.len().into()
}

pub fn day9_part2(input: &str) -> SolutionType {
    let mut knots = [(0i32, 0i32); 10];
    let mut visited = HashSet::new();
    visited.insert(knots[0]);

    for line in input.trim().lines() {
        let line = line.trim();
        let amnt = line[2..].parse::<u32>().unwrap();

        for _ in 0..amnt {
            match line.chars().next().unwrap() {
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                'L' => knots[0].0 -= 1,
                'R' => knots[0].0 += 1,
                _ => unreachable!(),
            }

            for tail_num in 1..10 {
                let head = knots[tail_num - 1];
                let tail = &mut knots[tail_num];

                if head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1 {
                    continue;
                }

                match (head.0.cmp(&tail.0), head.1.cmp(&tail.1)) {
                    // Same row
                    (Ordering::Less, Ordering::Equal) => tail.0 -= 1,
                    (Ordering::Greater, Ordering::Equal) => tail.0 += 1,

                    // Same column
                    (Ordering::Equal, Ordering::Less) => tail.1 -= 1,
                    (Ordering::Equal, Ordering::Greater) => tail.1 += 1,

                    // Diagonals
                    (Ordering::Less, Ordering::Less) => {
                        tail.0 -= 1;
                        tail.1 -= 1;
                    }
                    (Ordering::Less, Ordering::Greater) => {
                        tail.0 -= 1;
                        tail.1 += 1;
                    }
                    (Ordering::Greater, Ordering::Less) => {
                        tail.0 += 1;
                        tail.1 -= 1;
                    }
                    (Ordering::Greater, Ordering::Greater) => {
                        tail.0 += 1;
                        tail.1 += 1;
                    }

                    // Overlapping
                    (Ordering::Equal, Ordering::Equal) => {}
                }
            }

            visited.insert(knots[9]);
        }
    }

    visited.len().into()
}

#[test]
#[cfg(test)]
fn test() {
    let input = "R 4
                U 4
                L 3
                D 1
                R 4
                D 1
                L 5
                R 2";
    let input2 = "R 5
                U 8
                L 8
                D 3
                R 17
                D 10
                L 25
                U 20";

    assert_eq!(day9(input), SolutionType::Int(13));
    assert_eq!(day9_part2(input), SolutionType::Int(1));
    assert_eq!(day9_part2(input2), SolutionType::Int(36));
}
