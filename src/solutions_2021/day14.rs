use std::collections::{BTreeMap, HashMap};

use crate::SolutionType;

pub fn day14(input: &str) -> SolutionType {
    let mut polymer = input
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .collect::<Vec<_>>();

    let mut rules = HashMap::new();
    for rule in input.lines().skip(2) {
        let mut iter = rule.trim().split(" -> ");
        let polymer_template = iter.next().unwrap();
        let insertion_rule = iter.next().unwrap();

        let char1 = polymer_template.chars().next().unwrap();
        let char2 = polymer_template.chars().nth(1).unwrap();
        rules.insert((char1, char2), insertion_rule.chars().next().unwrap());
    }

    for _step in 1..=10 {
        let mut new_polymer = vec![];
        for i in 0..polymer.len() - 1 {
            if let Some(rule) = rules.get(&(polymer[i], polymer[i + 1])) {
                new_polymer.push(polymer[i]);
                new_polymer.push(*rule);
            } else {
                new_polymer.push(polymer[i]);
            }
        }
        new_polymer.push(polymer[polymer.len() - 1]);
        polymer = new_polymer;
    }

    let mut counts = BTreeMap::new();
    for val in polymer {
        *counts.entry(val).or_insert(0) += 1;
    }
    let max_amount = *counts.values().max().unwrap();
    let min_amount = *counts.values().min().unwrap();

    SolutionType::Int(max_amount - min_amount)
}

pub fn day14_part2(input: &str) -> SolutionType {
    let polymer = input
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .collect::<Vec<_>>();
    let mut chars = HashMap::new();
    for char in &polymer {
        *chars.entry(*char).or_insert(0) += 1;
    }
    let mut pairs = polymer
        .windows(2)
        .map(|k| ((k[0], k[1]), 1))
        .collect::<HashMap<_, _>>();

    let mut rules = HashMap::new();
    for rule in input.lines().skip(2) {
        let mut iter = rule.trim().split(" -> ");
        let polymer_template = iter.next().unwrap();
        let insertion_rule = iter.next().unwrap();

        let char1 = polymer_template.chars().next().unwrap();
        let char2 = polymer_template.chars().nth(1).unwrap();
        let charr = insertion_rule.chars().next().unwrap();
        rules.insert((char1, char2), ((char1, charr), (charr, char2)));
    }

    for _step in 1..=40 {
        let mut new_pairs = HashMap::new();
        for (pair, amount) in pairs.iter() {
            if let Some((subs1, subs2)) = rules.get(pair) {
                *new_pairs.entry(*subs1).or_insert(0) += amount;
                *new_pairs.entry(*subs2).or_insert(0) += amount;
                *chars.entry(subs1.1).or_insert(0) += amount;
            } else {
                *new_pairs.entry(*pair).or_insert(0) += amount;
            }
        }
        pairs = new_pairs;
    }

    let max_amount = *chars.values().max().unwrap();
    let min_amount = *chars.values().min().unwrap();

    SolutionType::Int(max_amount - min_amount)
}

#[test]
#[cfg(test)]
pub fn test() {
    let input = "NNCB

                CH -> B
                HH -> N
                CB -> H
                NH -> C
                HB -> C
                HC -> B
                HN -> C
                NN -> C
                BH -> H
                NC -> B
                NB -> B
                BN -> B
                BB -> N
                BC -> B
                CC -> N
                CN -> C";

    assert_eq!(day14(input), SolutionType::Int(1588));
    assert_eq!(day14_part2(input), SolutionType::Int(2188189693529));
}
