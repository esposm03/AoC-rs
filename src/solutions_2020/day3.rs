use crate::Solution;

pub fn day3(input: &str) -> Solution {
    let map = Map::parse(input);

    let mut treecount = 0_i64;
    let (mut x, mut y) = (0, 0);

    while y < map.height() {
        if map.at(x, y) {
            treecount += 1
        }
        x += 3;
        y += 1;
    }

    treecount.into()
}

pub fn day3_part2(input: &str) -> Solution {
    let map = Map::parse(input);
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut total = 1_i64;

    for (dx, dy) in &slopes {
        let mut treecount = 0;
        let (mut x, mut y) = (0, 0);

        while y < map.height() {
            if map.at(x, y) {
                treecount += 1
            }
            x += dx;
            y += dy;
        }

        total *= treecount;
    }

    total.into()
}

/// The map of trees. It contains a matrix of booleans, where
/// `true` means there is a tree, and `false` means there isn't
#[derive(Debug, Eq, PartialEq)]
struct Map {
    columns: Vec<Vec<bool>>,
}

impl Map {
    /// Parse map from input string
    fn parse(input: &str) -> Self {
        let mut cols = vec![];

        for _ in input.lines().next().unwrap().chars() {
            cols.push(vec![]);
        }

        for line in input.lines() {
            for (i, ch) in line.char_indices() {
                cols[i].push(ch == '#');
            }
        }

        Map { columns: cols }
    }

    /// Is there a tree at this position? (Accounts for map repeating itself)
    fn at(&self, x: usize, y: usize) -> bool {
        self.columns[x % self.columns.len()][y % self.columns[0].len()]
    }

    /// The height of the map (the number of cells in a column)
    fn height(&self) -> usize {
        self.columns[0].len()
    }
}

#[cfg(test)]
#[test]
fn test() {
    let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    assert_eq!(day3(input), 7_i64.into());
    assert_eq!(day3_part2(input), 336_i64.into());
}
