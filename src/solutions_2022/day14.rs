use std::cmp::{max, min};

use crate::{Solution, Visualization};

pub fn day14(input: &str) -> Solution {
    let mut vis = Visualization::new(4);

    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    let mut lines = vec![];

    for line in input.trim().lines() {
        let line: Vec<_> = line
            .split_whitespace()
            .filter(|i| *i != "->")
            .collect::<Vec<_>>();

        for line in line.windows(2) {
            let x1: usize = line[0].split(',').next().unwrap().parse().unwrap();
            let y1: usize = line[0].split(',').nth(1).unwrap().parse().unwrap();
            let x2: usize = line[1].split(',').next().unwrap().parse().unwrap();
            let y2: usize = line[1].split(',').nth(1).unwrap().parse().unwrap();
            lines.push((min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2)));

            min_x = min(min_x, x1);
            min_x = min(min_x, x2);
            max_x = max(max_x, x1);
            max_x = max(max_x, x2);
            max_y = max(max_y, y1);
            max_y = max(max_y, y2);
        }
    }

    let mut mat = vec![vec![Tile::Air; max_y + 1]; max_x + 1];
    for (x1, y1, x2, y2) in lines {
        for x in x1..=x2 {
            for y in y1..=y2 {
                mat[x][y] = Tile::Rock;
            }
        }
    }

    vis.draw(draw, &mat);

    let mut deposited = 0;
    let mut finished = false;
    while !finished {
        let mut sand = (500, 0);
        let mut to_rest = false;

        while !to_rest && !finished {
            if sand.0 < min_x || sand.0 > max_x || sand.1 > max_y {
                finished = true;
                break;
            }

            let y = sand.1.wrapping_add(1);
            let x = sand.0;
            let xl = x.wrapping_sub(1);
            let xr = x.saturating_add(1);
            let dc = mat.get(x).and_then(|mat| mat.get(y));
            let dl = mat.get(xl).and_then(|mat| mat.get(y));
            let dr = mat.get(xr).and_then(|mat| mat.get(y));

            if let Some(Tile::Air) = dc {
                sand.1 += 1;
            } else if dc.is_none() {
                finished = true;
            } else if let Some(Tile::Air) = dl {
                sand.0 -= 1;
                sand.1 += 1;
            } else if dl.is_none() {
                finished = true;
            } else if let Some(Tile::Air) = dr {
                sand.0 += 1;
                sand.1 += 1;
            } else if dr.is_none() {
                finished = true;
            } else {
                to_rest = true;
                deposited += 1;
                mat[sand.0][sand.1] = Tile::Sand;
            }
        }
        vis.draw(draw, &mat);
    }

    vis.close();

    Solution::Int(deposited)
}

pub fn day14_part2(input: &str) -> Solution {
    let mut vis = Visualization::new(4);

    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    let mut lines = vec![];

    for line in input.trim().lines() {
        let line: Vec<_> = line
            .split_whitespace()
            .filter(|i| *i != "->")
            .collect::<Vec<_>>();

        for line in line.windows(2) {
            let x1: usize = line[0].split(',').next().unwrap().parse().unwrap();
            let y1: usize = line[0].split(',').nth(1).unwrap().parse().unwrap();
            let x2: usize = line[1].split(',').next().unwrap().parse().unwrap();
            let y2: usize = line[1].split(',').nth(1).unwrap().parse().unwrap();
            lines.push((min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2)));

            max_y = max(max_y, y1 + 2);
            max_y = max(max_y, y2 + 2);
            min_x = min(min_x, x1 - max_y);
            min_x = min(min_x, x2 - max_y);
            max_x = max(max_x, x1 + max_y);
            max_x = max(max_x, x2 + max_y);
        }
    }

    assert!(min_x > 0);
    lines.push((min_x, max_y, max_x, max_y));

    let mut mat = vec![vec![Tile::Air; max_y + 1]; max_x + 1];
    for (x1, y1, x2, y2) in lines {
        for x in x1..=x2 {
            for y in y1..=y2 {
                mat[x][y] = Tile::Rock;
            }
        }
    }

    vis.draw(draw, &mat);

    let mut deposited = 0;
    let mut finished = false;
    while !finished {
        let mut sand = (500, 0);
        let mut to_rest = false;

        while !to_rest && !finished {
            if sand.0 < min_x || sand.0 > max_x || sand.1 > max_y {
                finished = true;
                break;
            }

            let y = sand.1 + 1;
            let x = sand.0;
            let xl = x - 1;
            let xr = x + 1;
            let dc = mat[x][y];
            let dl = mat[xl][y];
            let dr = mat[xr][y];

            if let Tile::Air = dc {
                sand.1 += 1;
            } else if let Tile::Air = dl {
                sand.0 -= 1;
                sand.1 += 1;
            } else if let Tile::Air = dr {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                to_rest = true;
                deposited += 1;
                mat[sand.0][sand.1] = Tile::Sand;
            }
        }

        vis.draw(draw, &mat);

        if sand == (500, 0) {
            finished = true;
        }
    }

    vis.close();

    Solution::Int(deposited)
}

fn draw(vis: &mut Visualization, mat: &Vec<Vec<Tile>>) {
    assert!(
        mat.len() * mat[0].len() <= 1280 * 720,
        "Matrix too big for frame"
    );

    for x in 0..mat.len() {
        for y in 0..mat[0].len() {
            let rgb = match mat[x][y] {
                Tile::Sand => (255, 255, 0),
                Tile::Rock => (255, 255, 255),
                Tile::Air => (0, 0, 0),
            };

            vis.set_pixel(x, y, rgb);
        }
    }
}

#[derive(Copy, Clone)]
enum Tile {
    Sand,
    Rock,
    Air,
}

#[test]
#[cfg(test)]
fn test() {
    // TODO: This overflows its stack, don't know why

    // let input = "498,4 -> 498,6 -> 496,6
    //             503,4 -> 502,4 -> 502,9 -> 494,9";

    // assert_eq!(day14(input), Solution::Int(24));
    // assert_eq!(day14_part2(input), Solution::Int(93));
}
