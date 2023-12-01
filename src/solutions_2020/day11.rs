use crate::Solution;
use Tile::*;

pub fn day11(input: &str) -> Solution {
    let mut map = Map::parse(input);
    while map != map.simulate(true) {
        map = map.simulate(true);
    }

    map.map
        .iter()
        .map(|vec| vec.iter().filter(|i| **i == Full).count() as i64)
        .sum::<i64>()
        .into()
}

pub fn day11_part2(input: &str) -> Solution {
    let mut map = Map::parse(input);
    while map != map.simulate(false) {
        map = map.simulate(false);
    }

    map.map
        .iter()
        .map(|vec| vec.iter().filter(|i| **i == Full).count() as i64)
        .sum::<i64>()
        .into()
}

// When indexing, the first number is the x, the second the y.
// In other words, we first specify the column, then the row
// As such, we need an array of columns, which then contain single cells
// (the "rows")
#[derive(Debug, Eq, PartialEq, Clone)]
struct Map {
    map: Vec<Vec<Tile>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Full,
}

impl Map {
    fn count_near_seats(&self, x: usize, y: usize, part1: bool) -> usize {
        let deltas = &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut res = 0;

        for (dx, dy) in deltas {
            if part1 {
                if self.get(x as isize + dx, y as isize + dy) == Some(Full) {
                    res += 1
                }
            } else {
                let mut i = 1;
                while let Some(Floor) = self.get(x as isize + (dx * i), y as isize + (dy * i)) {
                    i += 1;
                }
                if let Some(Full) = self.get(x as isize + (dx * i), y as isize + (dy * i)) {
                    res += 1;
                }
            }
        }

        res
    }

    fn simulate(&self, part1: bool) -> Self {
        let mut res = self.clone();

        for y in 0..self.map[0].len() {
            for x in 0..self.map.len() {
                let near_full_seats = self.count_near_seats(x, y, part1);

                if self.map[x][y] == Empty && near_full_seats == 0 {
                    res.map[x][y] = Full;
                }
                if part1 && self.map[x][y] == Full && near_full_seats >= 4 {
                    res.map[x][y] = Empty;
                }
                if !part1 && self.map[x][y] == Full && near_full_seats >= 5 {
                    res.map[x][y] = Empty;
                }
            }
        }

        res
    }

    fn get(&self, x: isize, y: isize) -> Option<Tile> {
        if x < 0 || y < 0 {
            return None;
        }

        self.map
            .get(x as usize)
            .and_then(|i| i.get(y as usize))
            .copied()
    }

    fn parse(input: &str) -> Self {
        let mut map = vec![];

        for _ in input.lines().next().unwrap().chars() {
            map.push(vec![]);
        }

        for line in input.lines() {
            for (i, ch) in line.trim().char_indices() {
                map[i].push(match ch {
                    '#' => Full,
                    'L' => Empty,
                    '.' => Floor,
                    _ => unreachable!(),
                });
            }
        }

        Map { map }
    }

    #[cfg(test)]
    fn write(&self) -> String {
        let mut string = String::with_capacity(self.map.len() * self.map[0].len());

        for y in 0..self.map[0].len() {
            for x in 0..self.map.len() {
                string.push(match self.map[x][y] {
                    Full => '#',
                    Empty => 'L',
                    Floor => '.',
                })
            }
            string.push('\n');
        }

        string
    }
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(
        Map::parse(".##L\n.##L\n.##L"),
        Map {
            map: vec![
                vec![Floor, Floor, Floor],
                vec![Full, Full, Full],
                vec![Full, Full, Full],
                vec![Empty, Empty, Empty],
            ]
        },
    );
    assert_eq!(Map::parse(".##L\n.##L\n.##L").write(), ".##L\n.##L\n.##L\n",);

    let input1 = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
    ";
    let input2 = "#.##.##.##
        #######.##
        #.#.#..#..
        ####.##.##
        #.##.##.##
        #.#####.##
        ..#.#.....
        ##########
        #.######.#
        #.#####.##
    ";
    let input3 = "#.LL.L#.##
        #LLLLLL.L#
        L.L.L..L..
        #LLL.LL.L#
        #.LL.LL.LL
        #.LLLL#.##
        ..L.L.....
        #LLLLLLLL#
        #.LLLLLL.L
        #.#LLLL.##
    ";
    let input4 = "#.##.L#.##
        #L###LL.L#
        L.#.#..#..
        #L##.##.L#
        #.##.LL.LL
        #.###L#.##
        ..#.#.....
        #L######L#
        #.LL###L.L
        #.#L###.##
    ";
    let input5 = "#.#L.L#.##
        #LLL#LL.L#
        L.L.L..#..
        #LLL.##.L#
        #.LL.LL.LL
        #.LL#L#.##
        ..L.L.....
        #L#LLLL#L#
        #.LLLLLL.L
        #.#L#L#.##
    ";
    let input6 = "#.#L.L#.##
        #LLL#LL.L#
        L.#.L..#..
        #L##.##.L#
        #.#L.LL.LL
        #.#L#L#.##
        ..L.L.....
        #L#L##L#L#
        #.LLLLLL.L
        #.#L#L#.##
    ";

    let map1 = Map::parse(input1);
    let map2 = Map::parse(input2);
    let map3 = Map::parse(input3);
    let map4 = Map::parse(input4);
    let map5 = Map::parse(input5);
    let map6 = Map::parse(input6);

    assert_eq!(map1.simulate(true), map2);
    assert_eq!(map2.simulate(true), map3);
    assert_eq!(map3.simulate(true), map4);
    assert_eq!(map4.simulate(true), map5);
    assert_eq!(map5.simulate(true), map6);
    assert_eq!(map6.simulate(true), map6);

    let part2_vis1 = Map::parse(
        ".......#.
        ...#.....
        .#.......
        .........
        ..#L....#
        ....#....
        .........
        #........
        ...#.....
    ",
    );
    assert_eq!(part2_vis1.get(3, 4), Some(Empty));
    assert_eq!(part2_vis1.count_near_seats(3, 4, false), 8);

    eprintln!("\n\n");
    let part2_vis2 = Map::parse(
        ".............
        .L.L.#.#.#.#.
        .............
    ",
    );
    assert_eq!(part2_vis2.count_near_seats(1, 1, false), 0);

    let part2_input1 = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
    ";
    assert_eq!(day11_part2(part2_input1), 26_i64.into());
}
