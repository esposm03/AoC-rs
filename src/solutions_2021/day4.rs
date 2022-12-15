use crate::Solution;

pub fn day4(input: &str) -> Solution {
    let mut input = input.split("\n\n");
    let mut extracted = input.next().unwrap().trim().split(',');
    let mut boards = input.map(Board::new).collect::<Vec<_>>();
    let mut last_number;

    let winner_board = loop {
        let i = extracted.next().unwrap().parse().unwrap();
        last_number = i;

        for b in &mut boards {
            b.extracted_number(i);
        }

        if let Some(b) = boards.iter().find(|b| b.is_winner()) {
            break b;
        }
    };

    let all_unmarked: i64 = winner_board
        .numbers
        .iter()
        .enumerate()
        .filter(|(pos, _)| {
            let pos = *pos as u8;
            let pos = (pos % 5, pos / 5);
            !winner_board.marked.contains(&pos)
        })
        .map(|(_, n)| *n as i64)
        .sum();

    Solution::Int(all_unmarked * last_number as i64)
}

pub fn day4_part2(input: &str) -> Solution {
    let mut input = input.split("\n\n");
    let mut extracted = input.next().unwrap().trim().split(',');
    let mut boards = input.map(Board::new).collect::<Vec<_>>();
    let mut winners = vec![];
    let mut i = extracted
        .clone()
        .peekable()
        .peek()
        .unwrap()
        .parse()
        .unwrap();
    let mut last_number = i;

    while !boards.iter().all(|b| b.is_winner()) {
        i = extracted.next().unwrap().parse().unwrap();
        last_number = i;

        for b in &mut boards {
            b.extracted_number(i);
        }

        for (i, _) in boards.iter().enumerate().filter(|(_, b)| b.is_winner()) {
            if !winners.contains(&i) {
                winners.push(i);
            }
        }
    }

    let last_winner_board = &boards[*winners.last().unwrap()];
    let all_unmarked: i64 = last_winner_board
        .numbers
        .iter()
        .enumerate()
        .filter(|(pos, _)| {
            let pos = *pos as u8;
            let pos = (pos % 5, pos / 5);
            !last_winner_board.marked.contains(&pos)
        })
        .map(|(_, n)| *n as i64)
        .sum();

    Solution::Int(all_unmarked * last_number as i64)
}

struct Board {
    pub numbers: Vec<u8>,
    // (x, y) - positions are 0..5
    pub marked: Vec<(u8, u8)>,
}

impl Board {
    pub fn new(input: &str) -> Self {
        let mut numbers = Vec::with_capacity(25);

        for row in 0..5 {
            for column in 0..5 {
                let num: u8 = input[3 * column + 15 * row..][..2].trim().parse().unwrap();
                numbers.push(num);
            }
        }

        Board {
            numbers,
            marked: vec![],
        }
    }

    // If given number is contained within `self.numbers`, push its position into `self.marked`
    pub fn extracted_number(&mut self, n: u8) {
        if let Some(pos) = self.numbers.iter().position(|i| *i == n) {
            let pos = pos as u8;
            self.marked.push((pos % 5, pos / 5));
        }
    }

    pub fn is_winner(&self) -> bool {
        for row in 0..5 {
            if self.marked.iter().filter(|(_, y)| *y == row).count() == 5 {
                return true;
            }
        }

        for col in 0..5 {
            if self.marked.iter().filter(|(x, _)| *x == col).count() == 5 {
                return true;
            }
        }

        false
    }
}

#[test]
#[cfg(test)]
fn test() {
    let mut board = Board::new(
        "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
",
    );

    assert_eq!(board.numbers[0], 22);
    assert_eq!(board.numbers[4], 0);
    assert_eq!(board.numbers[5], 8);

    board.extracted_number(21);
    board.extracted_number(9);
    assert!(!board.is_winner());
    board.extracted_number(14);
    board.extracted_number(16);
    assert!(!board.is_winner());
    board.extracted_number(7);

    assert_eq!(board.marked[0], (0, 2));
    assert!(board.is_winner());

    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
    assert_eq!(day4(input), Solution::Int(4512));
    assert_eq!(day4_part2(input), Solution::Int(1924));
}
