use crate::Solution;

pub fn day11(input: &str) -> Solution {
    day11_part1(input, 100)
}

fn day11_part1(input: &str, n_steps: usize) -> Solution {
    let mut mat = Matrix::new();

    for (row, line) in input.lines().enumerate() {
        for (col, octopus) in line.trim().char_indices() {
            let octopus = octopus.to_digit(10).unwrap();
            mat.set(col, row, octopus);
        }
    }

    let mut total_flashed = 0;

    for _step in 0..n_steps {
        for col in 0..10 {
            for row in 0..10 {
                mat.inner[col][row] += 1;
            }
        }

        let mut already_flashed = vec![];
        let mut to_check = vec![];
        for col in 0..10 {
            for row in 0..10 {
                to_check.push((col as isize, row as isize));
            }
        }
        while let Some((col, row)) = to_check.pop() {
            if !(0..=9).contains(&col) || !(0..=9).contains(&row) {
                continue;
            }

            if mat.get(col as usize, row as usize) > 9 && !already_flashed.contains(&(col, row)) {
                already_flashed.push((col, row));
                total_flashed += 1;

                mat.incr(col - 1, row - 1);
                mat.incr(col, row - 1);
                mat.incr(col + 1, row - 1);
                mat.incr(col - 1, row);
                mat.incr(col, row);
                mat.incr(col + 1, row);
                mat.incr(col - 1, row + 1);
                mat.incr(col, row + 1);
                mat.incr(col + 1, row + 1);

                to_check.push((col - 1, row - 1));
                to_check.push((col, row - 1));
                to_check.push((col + 1, row - 1));
                to_check.push((col - 1, row));
                to_check.push((col, row));
                to_check.push((col + 1, row));
                to_check.push((col - 1, row + 1));
                to_check.push((col, row + 1));
                to_check.push((col + 1, row + 1));
            }
        }

        for col in 0..10 {
            for row in 0..10 {
                if mat.inner[col][row] > 9 {
                    mat.inner[col][row] = 0;
                }
            }
        }
    }

    Solution::Int(total_flashed)
}

pub fn day11_part2(input: &str) -> Solution {
    let mut mat = Matrix::new();

    for (row, line) in input.lines().enumerate() {
        for (col, octopus) in line.trim().char_indices() {
            let octopus = octopus.to_digit(10).unwrap();
            mat.set(col, row, octopus);
        }
    }

    let mut step = 0;
    loop {
        step += 1;

        for col in 0..10 {
            for row in 0..10 {
                mat.inner[col][row] += 1;
            }
        }

        let mut already_flashed = vec![];
        let mut to_check = vec![];
        let mut step_flashed = 0;
        for col in 0..10 {
            for row in 0..10 {
                to_check.push((col as isize, row as isize));
            }
        }
        while let Some((col, row)) = to_check.pop() {
            if !(0..=9).contains(&col) || !(0..=9).contains(&row) {
                continue;
            }

            if mat.get(col as usize, row as usize) > 9 && !already_flashed.contains(&(col, row)) {
                already_flashed.push((col, row));
                step_flashed += 1;

                mat.incr(col - 1, row - 1);
                mat.incr(col, row - 1);
                mat.incr(col + 1, row - 1);
                mat.incr(col - 1, row);
                mat.incr(col, row);
                mat.incr(col + 1, row);
                mat.incr(col - 1, row + 1);
                mat.incr(col, row + 1);
                mat.incr(col + 1, row + 1);

                to_check.push((col - 1, row - 1));
                to_check.push((col, row - 1));
                to_check.push((col + 1, row - 1));
                to_check.push((col - 1, row));
                to_check.push((col, row));
                to_check.push((col + 1, row));
                to_check.push((col - 1, row + 1));
                to_check.push((col, row + 1));
                to_check.push((col + 1, row + 1));
            }
        }

        if step_flashed == 100 {
            return Solution::Int(step);
        }

        for col in 0..10 {
            for row in 0..10 {
                if mat.inner[col][row] > 9 {
                    mat.inner[col][row] = 0;
                }
            }
        }
    }
}

struct Matrix {
    inner: [[u32; 10]; 10],
}

impl Matrix {
    fn new() -> Self {
        Matrix {
            inner: Default::default(),
        }
    }

    fn set(&mut self, col: usize, row: usize, n: u32) {
        self.inner[col][row] = n;
    }

    fn get(&self, col: usize, row: usize) -> u32 {
        self.inner[col][row]
    }

    fn incr(&mut self, col: isize, row: isize) {
        if (0..10).contains(&col) && (0..10).contains(&row) {
            self.inner[col as usize][row as usize] += 1;
        }
    }
}

#[test]
#[cfg(test)]
fn test() {
    let input = "5483143223
                2745854711
                5264556173
                6141336146
                6357385478
                4167524645
                2176841721
                6882881134
                4846848554
                5283751526";

    assert_eq!(day11_part1(input, 100), Solution::Int(1656));
    assert_eq!(day11_part2(input), Solution::Int(195));
}
