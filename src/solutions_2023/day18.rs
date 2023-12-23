use std::cmp::{max, min};

use crate::{Array2D, Solution, Visualization};

pub fn day18(input: &str) -> Solution {
    let mut v = Visualization::new(2);

    let mut x = 0;
    let mut y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;
    for line in input.trim().lines() {
        let mut iter = line.trim().split(' ');
        let dir = iter.next().unwrap();
        let amt: isize = iter.next().unwrap().parse().unwrap();

        match dir {
            "R" => x += amt,
            "L" => x -= amt,
            "D" => y += amt,
            "U" => y -= amt,
            _ => unreachable!(),
        }
        max_x = max(max_x, x);
        max_y = max(max_y, y);
        min_x = min(min_x, x);
        min_y = min(min_y, y);
    }
    let max_x = (max_x - min_x + 1) as usize;
    let max_y = (max_y - min_y + 1) as usize;

    let mut map = Array2D::splat(false, max_x, max_y);
    let mut cur = ((-min_x) as usize, (-min_y) as usize);
    map[cur] = true;
    let start = ((-min_x) as usize, (-min_y) as usize);

    for line in input.trim().lines() {
        let mut iter = line.trim().split(' ');
        let dir = iter.next().unwrap();
        let amt = iter.next().unwrap().parse().unwrap();

        match dir {
            "R" => {
                for i in 0..=amt {
                    map[(cur.0 + i, cur.1)] = true;
                }
                cur = (cur.0 + amt, cur.1);
            }
            "L" => {
                for i in 0..=amt {
                    map[(cur.0 - i, cur.1)] = true;
                }
                cur = (cur.0 - amt, cur.1);
            }
            "U" => {
                for i in 0..=amt {
                    map[(cur.0, cur.1 - i)] = true;
                }
                cur = (cur.0, cur.1 - amt);
            }
            "D" => {
                for i in 0..=amt {
                    map[(cur.0, cur.1 + i)] = true;
                }
                cur = (cur.0, cur.1 + amt);
            }
            _ => unreachable!(),
        }

        v.draw(visualize, &map);
    }

    fill(
        &mut map,
        &mut v,
        (start.0 as isize + 1, start.1 as isize + 1),
    );

    Solution::Int(map.positions(true).len() as i64)
}

fn fill(map: &mut Array2D<bool>, v: &mut Visualization, start: (isize, isize)) {
    let mut stack = vec![start];
    let w = map.x() as isize;
    let h = map.y() as isize;

    let mut i = 0;
    while let Some((x, y)) = stack.pop() {
        if i % 100 == 0 {
            v.draw(visualize, map);
        }
        i += 1;

        map[(x as usize, y as usize)] = true;
        for (ndx, ndy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + ndx;
            let ny = y + ndy;
            if (0..w).contains(&nx) && (0..h).contains(&ny) && !map[(nx as usize, ny as usize)] {
                stack.push((nx, ny))
            }
        }
    }
}

fn visualize(v: &mut Visualization, map: &Array2D<bool>) {
    for (x, y) in map.positions(true) {
        v.set_pixel(x, y, (255, 255, 255));
    }
}

#[test]
#[cfg(test)]
fn test() {
    let input = "
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    ";

    assert_eq!(day18(input), Solution::Int(62));
    println!("part 1 successful");
    assert_eq!(day18_part2(input), Solution::Int(952408144115));
    panic!();
}
