use crate::Solution;

pub fn day3(input: &str) -> Solution {
    let mut iter = input.split('\n');
    let input = iter.next().unwrap();
    let input_2 = iter.next().unwrap();

    let mut path = Path { path: vec![(0, 0)] };
    let mut path_2 = Path { path: vec![(0, 0)] };

    for i in input.split(',') {
        path.push(Direction::from(i));
    }
    for i in input_2.split(',') {
        path_2.push(Direction::from(i));
    }

    let intersections = path.intersections_with_first_part(&path_2);
    intersections
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
        .into()
}

pub fn day3_part2(input: &str) -> Solution {
    let mut iter = input.split('\n');
    let input = iter.next().unwrap();
    let input_2 = iter.next().unwrap();

    let mut path = Path { path: vec![(0, 0)] };
    let mut path_2 = Path { path: vec![(0, 0)] };

    for i in input.split(',') {
        path.push(Direction::from(i));
    }
    for i in input_2.split(',') {
        path_2.push(Direction::from(i));
    }

    let intersections = path.intersections_with_second_part(&path_2);
    let steps: Vec<((i64, i64), i64)> = intersections
        .iter()
        .map(|i| (*i, path.steps_until(*i) + path_2.steps_until(*i)))
        .collect();

    let mut min_steps = steps[0];
    for i in steps {
        if i.1 < min_steps.1 {
            min_steps = i;
        }
    }

    Solution::Int(min_steps.1)
}

#[derive(Debug)]
struct Path {
    path: Vec<(i64, i64)>,
}

impl Path {
    pub fn push(&mut self, dir: Direction) {
        let starting_x = self.path.last().unwrap().0;
        let starting_y = self.path.last().unwrap().1;

        match dir {
            Direction::Up(delta_y) => {
                for i in 0..delta_y {
                    self.path.push((starting_x, starting_y + (i + 1)));
                }
            }
            Direction::Down(delta_y) => {
                for i in 0..delta_y {
                    self.path.push((starting_x, starting_y - (i + 1)));
                }
            }
            Direction::Right(delta_x) => {
                for i in 0..delta_x {
                    self.path.push((starting_x + (i + 1), starting_y))
                }
            }
            Direction::Left(delta_x) => {
                for i in 0..delta_x {
                    self.path.push((starting_x - (i + 1), starting_y));
                }
            }
        }
    }

    pub fn intersections_with_first_part(&self, other: &Path) -> Vec<(i64, i64)> {
        let mut res = vec![];

        for path_1 in &self.path {
            for path_2 in &other.path {
                if *path_1 == *path_2 && *path_1 != (0, 0) {
                    res.push(*path_1);
                }
            }
        }

        res
    }

    pub fn intersections_with_second_part(&self, other: &Path) -> Vec<(i64, i64)> {
        self.path
            .iter()
            .map(|path_1| {
                let mut res = vec![];

                for path_2 in &other.path {
                    if *path_1 == *path_2 {
                        res.push(*path_1);
                    }
                }

                res
            })
            .flatten()
            .filter(|i| *i != (0, 0))
            .collect()
    }

    pub fn steps_until(&self, point: (i64, i64)) -> i64 {
        let mut counter = 0;

        for i in &self.path {
            if *i != point {
                counter += 1;
            } else {
                break;
            };
        }

        counter
    }
}

#[derive(Debug)]
enum Direction {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl Direction {
    fn from(string: &str) -> Self {
        let number_text = string.trim_start_matches(char::is_alphabetic);
        let number = number_text
            .parse::<i64>()
            .expect("Can't parse number in `Direction::from`");

        match string
            .chars()
            .next()
            .expect("No chars in `Direction::from` string")
        {
            'U' => Direction::Up(number),
            'D' => Direction::Down(number),
            'L' => Direction::Left(number),
            'R' => Direction::Right(number),
            _ => panic!("Invalid string in `Direction::from`"),
        }
    }
}
