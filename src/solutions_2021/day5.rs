use std::cmp::Ordering;

use crate::Solution;

pub fn day5(input: &str) -> Solution {
    // First index is columns, second index is rows
    let mut matrix: Box<[[u16; 1000]; 1000]> = Box::new([[0; 1000]; 1000]);

    for line in input.lines() {
        let mut iter = line.trim().split(',').map(|n| n.parse::<usize>().unwrap());
        let mut col1 = iter.next().unwrap();
        let mut row1 = iter.next().unwrap();
        let mut col2 = iter.next().unwrap();
        let mut row2 = iter.next().unwrap();

        if row1 > row2 {
            std::mem::swap(&mut row1, &mut row2);
        }
        if col1 > col2 {
            std::mem::swap(&mut col1, &mut col2);
        }

        if col1 == col2 {
            for row in row1..=row2 {
                matrix[col1][row] += 1;
            }
        } else if row1 == row2 {
            for col in col1..=col2 {
                matrix[col][row1] += 1;
            }
        }
    }

    let mut sum = 0;
    for col in matrix.iter() {
        for row in col {
            if *row >= 2 {
                sum += 1;
            }
        }
    }
    Solution::Int(sum as i64)
}

pub fn day5_part2(input: &str) -> Solution {
    // First index is columns, second index is rows
    let mut matrix: Box<[[u16; 1000]; 1000]> = Box::new([[0; 1000]; 1000]);

    for line in input.lines() {
        let mut iter = line.trim().split(',').map(|n| n.parse::<usize>().unwrap());
        let mut col1 = iter.next().unwrap();
        let mut row1 = iter.next().unwrap();
        let mut col2 = iter.next().unwrap();
        let mut row2 = iter.next().unwrap();

        if row1 != row2 && col1 != col2 {
            for i in 0..=col1.abs_diff(col2) {
                let i = i as isize;
                let (col_off, row_off) = match (col1.cmp(&col2), row1.cmp(&row2)) {
                    (Ordering::Less, Ordering::Less) => (i, i),
                    (Ordering::Less, Ordering::Greater) => (i, -i),
                    (Ordering::Greater, Ordering::Less) => (-i, i),
                    (Ordering::Greater, Ordering::Greater) => (-i, -i),
                    _ => unreachable!(),
                };

                matrix[(col1 as isize + col_off) as usize][(row1 as isize + row_off) as usize] += 1;
            }
        }

        if row1 > row2 {
            std::mem::swap(&mut row1, &mut row2);
        }
        if col1 > col2 {
            std::mem::swap(&mut col1, &mut col2);
        }

        if col1 == col2 {
            for row in row1..=row2 {
                matrix[col1][row] += 1;
            }
        } else if row1 == row2 {
            for col in col1..=col2 {
                matrix[col][row1] += 1;
            }
        }
    }

    let mut sum = 0;
    for col in matrix.iter() {
        for row in col {
            if *row >= 2 {
                sum += 1;
            }
        }
    }
    Solution::Int(sum)
}

#[test]
#[cfg(test)]
fn test() {
    let input = "0,9,5,9
                8,0,0,8
                9,4,3,4
                2,2,2,1
                7,0,7,4
                6,4,2,0
                0,9,2,9
                3,4,1,4
                0,0,8,8
                5,5,8,2";

    assert_eq!(day5(input), Solution::Int(5));
    assert_eq!(day5_part2(input), Solution::Int(12));
}
