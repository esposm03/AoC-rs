use crate::Solution;

use std::fmt::Write;

pub fn day10(input: &str) -> Solution {
    let mut x = 1;
    let mut states = vec![];

    for command in input.trim().lines() {
        let mut iter = command.trim().split(' ');
        match iter.next().unwrap() {
            "noop" => states.push(x),
            "addx" => {
                states.push(x);
                states.push(x);
                x += iter.next().unwrap().parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }

    let mut sum = 0;
    let mut i = 19;
    while let Some(state) = states.get(i) {
        sum += *state * (i + 1) as i32;
        i += 40;
    }

    Solution::Int(sum as _)
}

pub fn day10_part2(input: &str) -> Solution {
    let mut x = 1;
    let mut states = vec![];

    for command in input.trim().lines() {
        let mut iter = command.trim().split(' ');
        match iter.next().unwrap() {
            "noop" => states.push(x),
            "addx" => {
                states.push(x);
                states.push(x);
                x += iter.next().unwrap().parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }

    let mut res = String::new();

    for cycle in 0..240 {
        let horiz = cycle % 40;

        if states[cycle].abs_diff(horiz as i32) <= 1 {
            write!(res, "#").unwrap();
        } else {
            write!(res, ".").unwrap();
        }

        if horiz == 39 {
            writeln!(res).unwrap();
        }
    }

    Solution::String(res)
}

#[test]
#[cfg(test)]
fn test() {
    let input = "addx 15
                addx -11
                addx 6
                addx -3
                addx 5
                addx -1
                addx -8
                addx 13
                addx 4
                noop
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx -35
                addx 1
                addx 24
                addx -19
                addx 1
                addx 16
                addx -11
                noop
                noop
                addx 21
                addx -15
                noop
                noop
                addx -3
                addx 9
                addx 1
                addx -3
                addx 8
                addx 1
                addx 5
                noop
                noop
                noop
                noop
                noop
                addx -36
                noop
                addx 1
                addx 7
                noop
                noop
                noop
                addx 2
                addx 6
                noop
                noop
                noop
                noop
                noop
                addx 1
                noop
                noop
                addx 7
                addx 1
                noop
                addx -13
                addx 13
                addx 7
                noop
                addx 1
                addx -33
                noop
                noop
                noop
                addx 2
                noop
                noop
                noop
                addx 8
                noop
                addx -1
                addx 2
                addx 1
                noop
                addx 17
                addx -9
                addx 1
                addx 1
                addx -3
                addx 11
                noop
                noop
                addx 1
                noop
                addx 1
                noop
                noop
                addx -13
                addx -19
                addx 1
                addx 3
                addx 26
                addx -30
                addx 12
                addx -1
                addx 3
                addx 1
                noop
                noop
                noop
                addx -9
                addx 18
                addx 1
                addx 2
                noop
                noop
                addx 9
                noop
                noop
                noop
                addx -1
                addx 2
                addx -37
                addx 1
                addx 3
                noop
                addx 15
                addx -21
                addx 22
                addx -6
                addx 1
                noop
                addx 2
                addx 1
                noop
                addx -10
                noop
                noop
                addx 20
                addx 1
                addx 2
                addx 2
                addx -6
                addx -11
                noop
                noop
                noop";

    assert_eq!(day10(input), Solution::Int(13140));
    assert_eq!(
        day10_part2(input),
        Solution::String(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....\n"
                .to_string()
        )
    )
}
