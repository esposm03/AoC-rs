use pathfinding::prelude::dijkstra;

use crate::Solution;

pub fn day12(input: &str) -> Solution {
    let input = input.trim();
    let n_cols = input.lines().next().unwrap().len();
    let n_rows = input.lines().count();

    let mut depths = vec![vec![0; n_rows]; n_cols];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.trim().char_indices() {
            depths[col][row] = match char {
                'S' => {
                    start = (col, row);
                    0
                }
                'E' => {
                    end = (col, row);
                    25
                }
                'a'..='z' => (char as u32) - 97,
                _ => unreachable!(),
            };
        }
    }

    dijkstra::<(usize, usize), usize, _, _, _>(
        &start,
        |pos| neighbors(&depths, *pos).into_iter().map(|i| (i, 1)),
        |pos| *pos == end,
    )
    .unwrap()
    .1
    .into()
}

pub fn day12_part2(input: &str) -> Solution {
    let input = input.trim();
    let n_cols = input.lines().next().unwrap().len();
    let n_rows = input.lines().count();

    let mut depths = vec![vec![0; n_rows]; n_cols];
    let mut end = (0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.trim().char_indices() {
            depths[col][row] = match char {
                'S' => 0,
                'E' => {
                    end = (col, row);
                    25
                }
                'a'..='z' => (char as u32) - 97,
                _ => unreachable!(),
            };
        }
    }

    let mut weights = vec![];
    for col in 0..n_cols {
        for row in 0..n_rows {
            if depths[col][row] == 0 {
                let weight = dijkstra::<(usize, usize), usize, _, _, _>(
                    &(col, row),
                    |pos| neighbors(&depths, *pos).into_iter().map(|i| (i, 1)),
                    |pos| *pos == end,
                )
                .unwrap_or((vec![], usize::MAX))
                .1;
                weights.push(weight);
            }
        }
    }
    weights.into_iter().min().unwrap().into()
}

fn neighbors(depths: &Vec<Vec<u32>>, node: (usize, usize)) -> Vec<(usize, usize)> {
    let n_cols = depths.len();
    let n_rows = depths[0].len();

    let col = node.0 as isize;
    let row = node.1 as isize;
    let node = depths[node.0][node.1];

    [
        (col - 1, row),
        (col + 1, row),
        (col, row - 1),
        (col, row + 1),
    ]
    .iter()
    .filter(|(i, _)| (0..n_cols).contains(&(*i as usize)))
    .filter(|(_, i)| (0..n_rows).contains(&(*i as usize)))
    .filter(|(col, row)| depths[*col as usize][*row as usize] <= node + 1)
    .map(|(col, row)| (*col as usize, *row as usize))
    .collect()
}

#[test]
#[cfg(test)]
fn test() {
    let input = "Sabqponm
                abcryxxl
                accszExk
                acctuvwj
                abdefghi";

    assert_eq!(day12(input), Solution::Int(31))
}
