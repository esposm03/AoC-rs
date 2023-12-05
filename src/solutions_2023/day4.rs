use std::{cmp::min, collections::HashSet};

use crate::Solution;

pub fn day4(input: &str) -> Solution {
    let mut score = 0i64;

    for card in input.trim().lines() {
        let card = card.replace("  ", " ");
        let (_, card) = card.split_once(':').unwrap();
        let (winning, got) = card.trim().split_once(" | ").unwrap();

        let winning: HashSet<i64> = winning.split(' ').map(|s| s.parse().unwrap()).collect();
        let got: HashSet<i64> = got.split(' ').map(|s| s.parse().unwrap()).collect();
        let num_winning = winning.intersection(&got).count();

        if num_winning >= 1 {
            score += 2i64.pow(num_winning as u32 - 1);
        }
    }

    Solution::Int(score)
}

pub fn day4_part2(input: &str) -> Solution {
    struct Card {
        num_winning: usize,
        amount: usize,
    }

    let mut list = vec![];

    for card in input.trim().lines() {
        let card = card.replace("  ", " ");
        let (_, card) = card.split_once(':').unwrap();
        let (winning, got) = card.trim().split_once(" | ").unwrap();

        let winning: HashSet<i64> = winning.split(' ').map(|s| s.parse().unwrap()).collect();
        let got: HashSet<i64> = got.split(' ').map(|s| s.parse().unwrap()).collect();
        let num_winning = winning.intersection(&got).count();
        list.push(Card {
            amount: 1,
            num_winning,
        });
    }

    let mut i = 0;
    while i < list.len() {
        let n = list[i].num_winning;
        for j in i + 1..min(i + n + 1, list.len()) {
            list[j].amount += list[i].amount;
        }

        i += 1;
    }

    Solution::Int(list.into_iter().map(|c| c.amount as i64).sum())
}

#[test]
#[cfg(test)]
fn test() {
    let input = "
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    assert_eq!(day4(input), Solution::Int(13));
    assert_eq!(day4_part2(input), Solution::Int(30));
}
