use std::collections::HashSet;

use crate::SolutionType;

pub fn day3(input: &str) -> SolutionType {
    let mut sum = 0;
    let mut comp1_set = HashSet::new();
    let mut comp2_set = HashSet::new();

    for rucksack in input.trim().lines() {
        let rucksack = rucksack.trim();
        let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);

        comp1_set.extend(comp1.chars());
        comp2_set.extend(comp2.chars());

        let shared = *comp1_set.intersection(&comp2_set).next().unwrap();
        if shared.is_ascii_lowercase() {
            sum += shared as i64 - 96;
        } else if shared.is_ascii_uppercase() {
            sum += shared as i64 - 64 + 26;
        }

        comp1_set.clear();
        comp2_set.clear();
    }

    sum.into()
}

pub fn day3_part2(input: &str) -> SolutionType {
    let mut sum = 0;
    let mut sack0_set = HashSet::new();
    let mut sack1_set = HashSet::new();
    let mut sack2_set = HashSet::new();

    let lines = input.trim().lines().collect::<Vec<_>>();

    for group in lines.chunks_exact(3) {
        sack0_set.extend(group[0].trim().chars());
        sack1_set.extend(group[1].trim().chars());
        sack2_set.extend(group[2].trim().chars());

        let badge = *sack0_set
            .intersection(&sack1_set)
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&sack2_set)
            .next()
            .unwrap();

        if badge.is_ascii_lowercase() {
            sum += badge as i64 - 96;
        } else if badge.is_ascii_uppercase() {
            sum += badge as i64 - 64 + 26;
        }

        sack0_set.clear();
        sack1_set.clear();
        sack2_set.clear();
    }

    sum.into()
}

#[test]
#[cfg(test)]
fn test() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(day3(input), SolutionType::Int(157));
    assert_eq!(day3_part2(input), SolutionType::Int(70));
}
