use std::cmp::{max, min};

use crate::Solution;

pub fn day15(input: &str) -> Solution {
    part1(input, 2_000_000).into()
}

fn part1(input: &str, line: i32) -> i64 {
    let mut sensors = vec![];
    let mut beacons = vec![];
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;

    for line in input.trim().lines() {
        let tokens = line.split_whitespace().collect::<Vec<_>>();

        let parse = |i: usize| -> i32 {
            tokens[i]
                .trim_matches(['x', 'y', '=', ':', ','].as_slice())
                .parse()
                .unwrap()
        };
        let sx = parse(2);
        let sy = parse(3);
        let bx = parse(8);
        let by = parse(9);
        let dist = manhattan_dist((sx, sy), (bx, by));

        println!("Beacon at {bx} {by}");
        sensors.push((sx, sy, dist));
        beacons.push((bx, by));
        min_x = min(min_x, sx - dist as i32);
        max_x = max(max_x, sx + dist as i32);
    }

    dbg!(max_x, min_x);

    let mut count = 0;
    for x in min_x..=max_x {
        let occupied = sensors
            .iter()
            .copied()
            .any(|(bx, by, d)| manhattan_dist((bx, by), (x, line)) <= d);

        let is_beacon = beacons.iter().any(|(bx, by)| *bx == x && *by == line);
        let is_sensor = sensors.iter().any(|(bx, by, _)| *bx == x && *by == line);

        if occupied && !is_beacon && !is_sensor {
            count += 1;
        }
    }
    println!();

    count
}

type Point = (i32, i32);

fn manhattan_dist(a: Point, b: Point) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[test]
#[cfg(test)]
fn test() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    assert_eq!(part1(input, 10), 26);
}
