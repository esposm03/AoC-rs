use std::{collections::HashMap, ops::RangeInclusive};

use crate::SolutionType;

pub fn day5(input: &str) -> SolutionType {
    let mut hm = HashMap::new();

    for line in input.lines() {
        push_all_cells(&mut hm, line);
    }

    hm.values().filter(|i| **i >= 2).count().into()
}

fn push_all_cells(hm: &mut HashMap<(i64, i64), i64>, line: &str) {
    let n1 = line.split(" -> ").next().unwrap();
    let n2 = line.split(" -> ").nth(1).unwrap();

    let x1 = n1.split(',').next().unwrap().parse().unwrap();
    let y1 = n1.split(',').nth(1).unwrap().parse().unwrap();
    let x2 = n2.split(',').next().unwrap().parse().unwrap();
    let y2 = n2.split(',').nth(1).unwrap().parse().unwrap();

    println!("Considering {}", line);

    if (x1 != x2) && (y1 != y2) {
        println!("  Skipping line");
        return;
    }

    if x1 == x2 {
        for y in y1..=y2 {
            *hm.entry((x1, y)).or_insert(0) += 1;
        }
    } else if y1 == y2 {
        for x in x1..=x2 {
            *hm.entry((x, y1)).or_insert(0) += 1;
        }
    }

    println!("Cells:");
    for (pos, n) in hm.iter() {
        println!("  {:?}: {}", pos, n);
    }
}

struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

impl Line {
    fn new(line: &str) -> Self {
        let n1 = line.split(" -> ").next().unwrap();
        let n2 = line.split(" -> ").nth(1).unwrap();

        let x1 = n1.split(',').next().unwrap().parse().unwrap();
        let y1 = n1.split(',').nth(1).unwrap().parse().unwrap();
        let x2 = n2.split(',').next().unwrap().parse().unwrap();
        let y2 = n2.split(',').nth(1).unwrap().parse().unwrap();

        Self { x1, y1, x2, y2 }
    }

    fn orientation(&self) -> Orientation {
        if self.x1 == self.x2 {
            Orientation::Horizontal
        } else if self.y1 == self.y2 {
            Orientation::Vertical
        } else {
            unreachable!()
        }
    }
}

#[derive(PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

fn intersections(l1: Line, l2: Line) -> Vec<(i64, i64)> {
    let mut res = vec![];

    if l1.orientation() != l2.orientation() {
        
    }

    res
}

#[test]
#[cfg(test)]
fn test() {
    assert_eq!(day5("0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2"), SolutionType::Int(5));
    todo!()
}