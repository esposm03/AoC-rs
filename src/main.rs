#![allow(non_snake_case)]
#![allow(clippy::needless_range_loop)]

use clap::Parser;
use std::{
    fmt::{Debug, Display},
    io::{self, Read, Write},
    ops::{Index, IndexMut},
    process::{Child, Command, Stdio},
};

mod solutions_2019;
mod solutions_2020;
mod solutions_2021;
mod solutions_2022;
mod solutions_2023;

pub struct Array2D<T: Clone + Debug> {
    inner: Vec<Vec<Option<T>>>,
    x: usize,
    y: usize,
}
impl<T: Clone + Debug + PartialEq> Array2D<T> {
    /// Create a new [`Array2D`] and fill it with `elem`
    pub fn splat(elem: T, x: usize, y: usize) -> Self {
        Self {
            inner: vec![vec![Some(elem); y]; x],
            x,
            y,
        }
    }

    /// Create a new, empty, [`Array2D`]
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            inner: vec![vec![None; y]; x],
            x,
            y,
        }
    }

    /// Returns the number of columns in this [`Array2D`]
    pub fn x(&self) -> usize {
        self.x
    }

    /// Returns the number of rows in this [`Array2D`]
    pub fn y(&self) -> usize {
        self.y
    }

    /// Sets an element at position `(x, y)`
    pub fn set(&mut self, x: usize, y: usize, elem: T) {
        self.inner[x][y] = Some(elem);
    }

    /// Return a list of all positions of elements equal to `elem`
    pub fn positions(&self, elem: T) -> Vec<(usize, usize)> {
        let mut res = vec![];

        for x in 0..self.x() {
            for y in 0..self.y() {
                if self.inner[x][y]
                    .as_ref()
                    .expect("called `Array2D::positions` even though it's not completely full")
                    == &elem
                {
                    res.push((x, y));
                }
            }
        }

        res
    }
}
impl<T: Clone + Debug> Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.inner[index.0][index.1].as_ref().unwrap()
    }
}
impl<T: Clone + Debug> IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.inner[index.0][index.1].as_mut().unwrap()
    }
}
impl Array2D<char> {
    pub fn from_map(input: &str) -> Self {
        let input = input.trim();
        let mut chars = Array2D::<char>::new(
            input.lines().next().unwrap().trim().len(),
            input.lines().count(),
        );

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.trim().char_indices() {
                chars.set(x, y, ch);
            }
        }

        chars
    }
}

#[derive(Parser)]
#[clap(
    version = "2021",
    author = "esposm03 <36164633+esposm03@users.noreply.github.com>"
)]
struct Invocation {
    /// The AoC year (default: 2023)
    #[clap(short, long, default_value = "2023")]
    year: usize,
    /// Whether to generate visualizations using ffmpeg
    #[arg(long)]
    visualization: bool,
    /// The problem number
    problem: usize,
    /// The problem part (1 or 2)
    part: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    Int(i64),
    OptionInt(Option<i64>),
    String(String),
}

impl From<i64> for Solution {
    fn from(i: i64) -> Self {
        Self::Int(i)
    }
}
impl From<usize> for Solution {
    fn from(i: usize) -> Self {
        Self::Int(i.try_into().unwrap())
    }
}
impl From<Option<i64>> for Solution {
    fn from(i: Option<i64>) -> Self {
        Self::OptionInt(i)
    }
}
impl From<&'_ str> for Solution {
    fn from(i: &'_ str) -> Self {
        Self::String(i.to_string())
    }
}
impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => f.write_fmt(format_args!("Solution: {i}")),
            Self::OptionInt(i) => f.write_fmt(format_args!("Solution: {i:?}")),
            Self::String(s) => f.write_fmt(format_args!("Solution:\n{s}")),
        }
    }
}

#[macro_export]
macro_rules! declare_solutions {
    () => {
        pub const SOLUTIONS: [for<'r> fn(&'r str) -> Solution; 50] = [
            day1,
            day1_part2,
            day2,
            day2_part2,
            day3,
            day3_part2,
            day4,
            day4_part2,
            day5,
            day5_part2,
            day6,
            day6_part2,
            day7,
            day7_part2,
            day8,
            day8_part2,
            day9,
            day9_part2,
            day10,
            day10_part2,
            day11,
            day11_part2,
            day12,
            day12_part2,
            day13,
            day13_part2,
            day14,
            day14_part2,
            day15,
            day15_part2,
            day16,
            day16_part2,
            day17,
            day17_part2,
            day18,
            day18_part2,
            day19,
            day19_part2,
            day20,
            day20_part2,
            day21,
            day21_part2,
            day22,
            day22_part2,
            day23,
            day23_part2,
            day24,
            day24_part2,
            day25,
            day25_part2,
        ];
    };
}

fn main() {
    let invocation = Invocation::parse();
    let number = invocation.problem * 2 + invocation.part - 3;

    if invocation.problem > 25 || invocation.problem < 1 {
        println!("Invalid problem :(");
        return;
    }
    if invocation.part != 1 && invocation.part != 2 {
        println!("Invalid part :(");
        return;
    }
    if invocation.visualization {
        unsafe {
            VISUALIZATION_ENABLED = true;
        }
    }

    println!("Now paste the input: ");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("IO error");

    println!(
        "{}",
        match invocation.year {
            2019 => solutions_2019::SOLUTIONS[number](&input),
            2020 => solutions_2020::SOLUTIONS[number](&input),
            2021 => solutions_2021::SOLUTIONS[number](&input),
            2022 => solutions_2022::SOLUTIONS[number](&input),
            2023 => solutions_2023::SOLUTIONS[number](&input),
            _ => "Invalid year :(".into(),
        }
    );
}

static mut VISUALIZATION_ENABLED: bool = false;

struct Visualization {
    process: Option<Child>,
    frame: Box<[u8]>,
    scale: u8,
}
impl Visualization {
    pub fn new(scale: u8) -> Self {
        let enabled = unsafe { VISUALIZATION_ENABLED };
        let args = [
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
        ];

        Visualization {
            process: enabled.then(|| {
                Command::new("ffmpeg")
                    .args(args)
                    .stderr(Stdio::null())
                    .stdout(Stdio::piped())
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Failed to start ffmpeg")
            }),
            frame: vec![0; 1280 * 720 * 3].into_boxed_slice(),
            scale,
        }
    }

    pub fn draw<D, F: Fn(&mut Visualization, D)>(&mut self, func: F, data: D) {
        let stdin = self.process.as_mut().and_then(|proc| proc.stdin.take());

        if let Some(mut stdin) = stdin {
            self.frame.fill(0);

            func(self, data);

            stdin.write_all(&self.frame).unwrap();

            self.process.as_mut().unwrap().stdin = Some(stdin);
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, (r, g, b): (u8, u8, u8)) {
        for xs in 0..self.scale {
            for ys in 0..self.scale {
                let x = self.scale as usize * x + xs as usize;
                let y = self.scale as usize * y + ys as usize;

                self.frame[y * 1280 * 3 + x * 3] = r;
                self.frame[y * 1280 * 3 + x * 3 + 1] = g;
                self.frame[y * 1280 * 3 + x * 3 + 2] = b;
            }
        }
    }

    pub fn close(self) {
        if let Some(mut proc) = self.process {
            let _ = proc.stdin.as_mut().unwrap();
            println!("ffmpeg exit status: {}", proc.wait().unwrap());
        }
    }
}
