use std::cmp::{max, min};

use crate::Solution;

#[cfg(not(test))]
use std::process::{ChildStdin, Command, Stdio};

pub fn day14(input: &str) -> Solution {
    #[cfg(not(test))]
    let ffmpeg = Command::new("ffmpeg")
        .args(&[
            "-loglevel",
            "warning",
            "-stats",
            "-f",
            "rawvideo",
            "-pixel_format",
            "rgb24",
            "-video_size",
            "1280x720",
            "-framerate",
            "30",
            "-i",
            "-",
            "-pix_fmt",
            "yuv420p",
            "-y",
            "output.mp4",
        ])
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    #[cfg(not(test))]
    let mut stdin = ffmpeg.stdin.unwrap();

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

    #[cfg(not(test))]
    draw(&mut stdin, &mat);

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
            let dc = mat.get(x).map(|mat| mat.get(y)).flatten();
            let dl = mat.get(xl).map(|mat| mat.get(y)).flatten();
            let dr = mat.get(xr).map(|mat| mat.get(y)).flatten();

            if let Some(Tile::Air) = dc {
                sand.1 += 1;
            } else if let None = dc {
                finished = true;
            } else if let Some(Tile::Air) = dl {
                sand.0 -= 1;
                sand.1 += 1;
            } else if let None = dl {
                finished = true;
            } else if let Some(Tile::Air) = dr {
                sand.0 += 1;
                sand.1 += 1;
            } else if let None = dr {
                finished = true;
            } else {
                to_rest = true;
                deposited += 1;
                mat[sand.0][sand.1] = Tile::Sand;
            }
        }

        #[cfg(not(test))]
        draw(&mut stdin, &mat);
    }

    Solution::Int(deposited)
}

#[cfg(not(test))]
fn draw(stdin: &mut ChildStdin, mat: &Vec<Vec<Tile>>) {
    use std::{io::Write, sync::Mutex};

    static FRAME: Mutex<[u8; 1280 * 720 * 3]> = Mutex::new([0; 1280 * 720 * 3]);
    let mut frame = FRAME.lock().unwrap();

    assert!(
        mat.len() * mat[0].len() <= 1280 * 720,
        "Matrix too big for frame"
    );

    let mut set_pixel = |x, y, (r, g, b)| {
        frame[y * 1280 * 3 + x * 3 + 0] = r;
        frame[y * 1280 * 3 + x * 3 + 1] = g;
        frame[y * 1280 * 3 + x * 3 + 2] = b;
    };

    let scale = 4;
    for x in 0..mat.len() {
        for y in 0..mat[0].len() {
            let rgb = match mat[x][y] {
                Tile::Sand => (255, 255, 0),
                Tile::Rock => (255, 255, 255),
                Tile::Air => (0, 0, 0),
            };

            for xs in 0..scale {
                for ys in 0..scale {
                    set_pixel(scale * x + xs, scale * y + ys, rgb);
                }
            }
        }
    }

    stdin.write_all(frame.as_slice()).unwrap();
    frame.fill(0);
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
    let input = "498,4 -> 498,6 -> 496,6
                503,4 -> 502,4 -> 502,9 -> 494,9";

    assert_eq!(day14(input), Solution::Int(24));
}
