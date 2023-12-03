use std::{cmp::max, collections::HashSet};

use crate::{Array2D, Solution};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Number {
    n: i64,
    x: usize,
    y: usize,
    n_digits: usize,
}
impl Number {
    fn contains(&self, x: isize, y: isize) -> bool {
        let x = max(0, x);
        let y = max(0, y);
        self.y == y as usize && x as usize >= self.x && (x as usize) < self.x + self.n_digits
    }

    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        let x = x as isize;
        let y = y as isize;

        self.contains(x - 1, y - 1)
            || self.contains(x, y - 1)
            || self.contains(x + 1, y - 1)
            || self.contains(x - 1, y)
            || self.contains(x, y)
            || self.contains(x + 1, y)
            || self.contains(x - 1, y + 1)
            || self.contains(x, y + 1)
            || self.contains(x + 1, y + 1)
    }
}

pub fn day3(input: &str) -> Solution {
    let chars = Array2D::from_map(input);
    let mut numbers = vec![];
    let mut solution = 0;
    let mut already_added = HashSet::new();

    for y in 0..chars.y() {
        let mut x = 0;
        while x < chars.y() {
            let mut end = x;
            while end < chars.y() && chars[(end, y)].is_ascii_digit() {
                end += 1;
            }

            let mut n = 0;
            for i in x..end {
                let digit = chars[(i, y)].to_digit(10).unwrap();
                let power = (end - 1) - i;
                n += digit as i64 * 10i64.pow(power as u32);
            }

            if n > 0 {
                let n_digits = end - x;
                numbers.push(Number { n, x, y, n_digits });
            }
            x += max(1, end - x);
        }
    }

    for x in 0..chars.x() {
        for y in 0..chars.y() {
            if !chars[(x, y)].is_ascii_digit() && chars[(x, y)] != '.' {
                for n in numbers.iter().filter(|n| n.is_adjacent(x, y)) {
                    if already_added.insert((n.x, n.y)) {
                        solution += n.n;
                    }
                }
            }
        }
    }

    Solution::Int(solution)
}

pub fn day3_part2(input: &str) -> Solution {
    let chars = Array2D::from_map(input);
    let mut numbers = vec![];
    let mut solution = 0;

    for y in 0..chars.y() {
        let mut x = 0;
        while x < chars.y() {
            let mut end = x;
            while end < chars.y() && chars[(end, y)].is_ascii_digit() {
                end += 1;
            }

            let mut n = 0;
            for i in x..end {
                let digit = chars[(i, y)].to_digit(10).unwrap();
                let power = (end - 1) - i;
                n += digit as i64 * 10i64.pow(power as u32);
            }

            if n > 0 {
                let n_digits = end - x;
                numbers.push(Number { n, x, y, n_digits });
            }
            x += max(1, end - x);
        }
    }

    for (x, y) in chars.positions('*') {
        let mut adjacents = HashSet::new();
        for n in &numbers {
            if n.is_adjacent(x, y) {
                adjacents.insert(n);
            }
        }
        if adjacents.len() == 2 {
            let mut iter = adjacents.into_iter();
            let a1 = iter.next().unwrap();
            let a2 = iter.next().unwrap();
            solution += a1.n * a2.n;
        }
    }

    Solution::Int(solution)
}

#[test]
#[cfg(test)]
fn test() {
    let input = "
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    ";

    assert_eq!(day3(input), Solution::Int(4361));
    assert_eq!(day3_part2(input), Solution::Int(467835));
}
