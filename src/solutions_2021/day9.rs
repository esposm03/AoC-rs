use std::collections::HashSet;

use crate::Solution;

pub fn day9(input: &str) -> Solution {
    let hrows = input.lines().count();
    let hcols = input.lines().next().unwrap().len();
    let mut heightmap = vec![];

    for line in input.lines() {
        let mut line_nums = vec![];

        for char in line.trim().chars() {
            line_nums.push(char.to_digit(10).unwrap());
        }

        assert_eq!(line_nums.len(), hcols);
        heightmap.push(line_nums);
    }
    assert_eq!(heightmap.len(), hrows);

    // First index is row, second index is column
    let mut risk_level = 0;
    for col in 0..hcols {
        for row in 0..hrows {
            let coli = col as isize;
            let rowi = row as isize;
            let n = heightmap[row][col];

            let a = !is_inside(coli - 1, rowi, hcols, hrows) || heightmap[row][col - 1] > n;
            let b = !is_inside(coli + 1, rowi, hcols, hrows) || heightmap[row][col + 1] > n;
            let c = !is_inside(coli, rowi - 1, hcols, hrows) || heightmap[row - 1][col] > n;
            let d = !is_inside(coli, rowi + 1, hcols, hrows) || heightmap[row + 1][col] > n;

            if a && b && c && d {
                risk_level += n + 1;
            }
        }
    }

    Solution::Int(risk_level as i64)
}

pub fn day9_part2(input: &str) -> Solution {
    let hrows = input.lines().count();
    let hcols = input.lines().next().unwrap().len();
    let mut heightmap = vec![];

    for line in input.lines() {
        let mut line_nums = vec![];

        for char in line.trim().chars() {
            line_nums.push(char.to_digit(10).unwrap());
        }

        assert_eq!(line_nums.len(), hcols);
        heightmap.push(line_nums);
    }
    assert_eq!(heightmap.len(), hrows);

    // First index is row, second index is column
    let mut low_points = vec![];
    for col in 0..hcols {
        for row in 0..hrows {
            let coli = col as isize;
            let rowi = row as isize;
            let n = heightmap[row][col];

            let a = !is_inside(coli - 1, rowi, hcols, hrows) || heightmap[row][col - 1] > n;
            let b = !is_inside(coli + 1, rowi, hcols, hrows) || heightmap[row][col + 1] > n;
            let c = !is_inside(coli, rowi - 1, hcols, hrows) || heightmap[row - 1][col] > n;
            let d = !is_inside(coli, rowi + 1, hcols, hrows) || heightmap[row + 1][col] > n;

            if a && b && c && d {
                low_points.push((row, col));
            }
        }
    }

    let mut basins = vec![];
    for point in low_points {
        let mut already_visited = HashSet::new();
        basins.push(visit_basin(
            (point.0 as isize, point.1 as isize),
            &mut already_visited,
            &heightmap,
            hcols,
            hrows,
        ));
    }

    basins.sort();
    basins.reverse();

    Solution::Int((basins[0] * basins[1] * basins[2]) as i64)
}

fn visit_basin(
    coord: (isize, isize),
    already_visited: &mut HashSet<(isize, isize)>,
    heightmap: &Vec<Vec<u32>>,
    heightmap_ncols: usize,
    heightmap_nrows: usize,
) -> u32 {
    if !is_inside(coord.0, coord.1, heightmap_nrows, heightmap_ncols) {
        return 0;
    }
    if !already_visited.insert(coord) {
        return 0;
    }

    #[rustfmt::skip]
    let adj_cells = [
        (coord.0 + 1, coord.1    ),
        (coord.0    , coord.1 + 1),
        (coord.0    , coord.1 - 1),
        (coord.0 - 1, coord.1    ),
    ];
    if heightmap[coord.0 as usize][coord.1 as usize] == 9 {
        0
    } else {
        adj_cells
            .iter()
            .map(|coord| {
                visit_basin(
                    *coord,
                    already_visited,
                    heightmap,
                    heightmap_ncols,
                    heightmap_nrows,
                )
            })
            .sum::<u32>()
            + 1
    }
}

fn is_inside(col: isize, row: isize, heightmap_ncols: usize, heightmap_nrows: usize) -> bool {
    let heightmap_ncols: isize = heightmap_ncols as _;
    let heightmap_nrows: isize = heightmap_nrows as _;

    (0..heightmap_ncols).contains(&col) && (0..heightmap_nrows).contains(&row)
}

#[test]
#[cfg(test)]
fn test() {
    let input = "2199943210
                3987894921
                9856789892
                8767896789
                9899965678";

    {
        let hrows = input.lines().count();
        let hcols = input.lines().next().unwrap().len();
        let mut heightmap = vec![];

        for line in input.lines() {
            let mut line_nums = vec![];

            for char in line.trim().chars() {
                line_nums.push(char.to_digit(10).unwrap());
            }

            assert_eq!(line_nums.len(), hcols);
            heightmap.push(line_nums);
        }
        assert_eq!(heightmap.len(), hrows);
        assert_eq!(
            visit_basin((0, 0), &mut HashSet::new(), &heightmap, hcols, hrows),
            3
        );
        assert_eq!(
            visit_basin((4, 9), &mut HashSet::new(), &heightmap, hcols, hrows),
            9
        );
    }

    assert_eq!(day9(input), Solution::Int(15));
    assert_eq!(day9_part2(input), Solution::Int(1134));
}
