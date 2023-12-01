use std::{
    cmp::{max, Ordering},
    collections::HashSet,
};

use crate::Solution;

fn day13_part(input: &str, num_steps: usize) -> HashSet<(u32, u32)> {
    let input = input.lines().map(|line| line.trim()).collect::<Vec<_>>();
    let mut iter = input.split(|line| line.is_empty());
    let dots_input = iter.next().unwrap();
    let folds = iter.next().unwrap();

    let mut dots = HashSet::new();
    for dot in dots_input {
        let mut iter = dot.trim().split(',');
        let x: u32 = iter.next().unwrap().parse().unwrap();
        let y: u32 = iter.next().unwrap().parse().unwrap();
        dots.insert((x, y));
    }

    for fold in folds.iter().take(num_steps) {
        let mut iter = fold.trim().split('=');
        let along_x = fold.chars().nth(11).unwrap() == 'x';
        let coord: u32 = iter.nth(1).unwrap().parse().unwrap();

        for (x, y) in std::mem::take(&mut dots) {
            assert!(!(along_x && coord == x), "Fold on line containing dots");
            assert!(along_x || coord != y, "Fold on line containing dots");

            let (x, y) = match (along_x, if along_x { x } else { y }.cmp(&coord)) {
                (true, Ordering::Greater) => (x - 2 * x.abs_diff(coord), y),
                (false, Ordering::Greater) => (x, y - 2 * y.abs_diff(coord)),
                (_, Ordering::Less) => (x, y),
                (_, Ordering::Equal) => panic!("Fold on line containing dots"),
            };

            dots.insert((x, y));
        }
    }

    dots
}

pub fn day13(input: &str) -> Solution {
    day13_part(input, 1).len().into()
}

pub fn day13_part2(input: &str) -> Solution {
    use std::fmt::Write;

    let dots = day13_part(input, 9999);
    let mut res = String::new();

    let mut max_x = 1;
    let mut max_y = 1;
    for (x, y) in &dots {
        max_x = max(max_x, *x);
        max_y = max(max_y, *y);
    }

    dbg!(max_x, max_y);

    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&(x, y)) {
                write!(res, "#").unwrap();
            } else {
                write!(res, ".").unwrap();
            }
        }
        writeln!(res).unwrap();
    }

    Solution::String(res)
}

#[test]
#[cfg(test)]
fn test() {
    let input = "6,10
                0,14
                9,10
                0,3
                10,4
                4,11
                6,0
                6,12
                4,1
                0,13
                10,12
                3,4
                3,0
                8,4
                1,10
                2,14
                8,10
                9,0
                
                fold along y=7";

    assert_eq!(day13(input), Solution::Int(17));
}
