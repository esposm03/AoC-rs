use crate::Solution;
use Action::*;

pub fn day12(input: &str) -> Solution {
    // Coordinates: we have x, which is the east-west axis, positive when moving towards east
    // we have y, which is the north-south axis, positive when moving towards north
    let (x, y, _) = input.lines().map(Action::parse).fold(
        (0i64, 0i64, Rotation::East),
        |(mut x, mut y, mut rot), elem| {
            match elem {
                North(n) => y += n as i64,
                South(n) => y -= n as i64,
                East(n) => x += n as i64,
                West(n) => x -= n as i64,
                Forward(n) => match rot {
                    Rotation::North => y += n as i64,
                    Rotation::South => y -= n as i64,
                    Rotation::East => x += n as i64,
                    Rotation::West => x -= n as i64,
                },
                Left(n) => {
                    let mut coord_num = match rot {
                        Rotation::North => 0,
                        Rotation::East => 90,
                        Rotation::South => 180,
                        Rotation::West => 270,
                    };

                    while coord_num - (n as i64) < 0 {
                        coord_num += 360;
                    }

                    rot = match (coord_num - n as i64) % 360 {
                        0 => Rotation::North,
                        90 => Rotation::East,
                        180 => Rotation::South,
                        270 => Rotation::West,
                        _ => unreachable!(),
                    };
                }
                Right(n) => {
                    let coord_num = match rot {
                        Rotation::North => 0,
                        Rotation::East => 90,
                        Rotation::South => 180,
                        Rotation::West => 270,
                    };

                    rot = match (coord_num + n) % 360 {
                        0 => Rotation::North,
                        90 => Rotation::East,
                        180 => Rotation::South,
                        270 => Rotation::West,
                        _ => unreachable!(),
                    };
                }
            };

            (x, y, rot)
        },
    );

    (x.abs() + y.abs()).into()
}

pub fn day12_part2(input: &str) -> Solution {
    let mut wp_x = 10;
    let mut wp_y = 1;
    let mut x = 0;
    let mut y = 0;

    for action in input.lines().map(Action::parse) {
        match action {
            North(n) => wp_y += n as i64,
            South(n) => wp_y -= n as i64,
            East(n) => wp_x += n as i64,
            West(n) => wp_x -= n as i64,
            Forward(n) => {
                x += wp_x * n as i64;
                y += wp_y * n as i64;
            }
            Left(n) => {
                let num = n / 90;
                for _ in 0..num {
                    let temp = wp_y;
                    wp_y = wp_x;
                    wp_x = -temp;
                }
            }
            Right(n) => {
                let num = n / 90;
                for _ in 0..num {
                    let temp = wp_x;
                    wp_x = wp_y;
                    wp_y = -temp;
                }
            }
        };
    }

    (x.abs() + y.abs()).into()
}

#[derive(Debug, Eq, PartialEq)]
enum Rotation {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, PartialEq)]
enum Action {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Forward(usize),
    Left(usize),
    Right(usize),
}

impl Action {
    fn parse(i: &str) -> Action {
        let num = i.chars().skip(1).collect::<String>().parse().unwrap();
        match i.chars().next().unwrap() {
            'N' => North(num),
            'S' => South(num),
            'E' => East(num),
            'W' => West(num),
            'F' => Forward(num),
            'L' => Left(num),
            'R' => Right(num),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(Action::parse("N32"), North(32));
    assert_eq!(day12("F10\nN3\nF7\nR90\nF11\n"), 25_i64.into());
    assert_eq!(day12_part2("F10\nN3\nF7\nR90\nF11\n"), 286_i64.into());
}
