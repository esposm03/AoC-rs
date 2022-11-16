use std::collections::{HashMap, HashSet};

use crate::SolutionType;

pub fn day12(input: &str) -> SolutionType {
    fn visit_node<'a>(
        nodes: &mut HashMap<&'a str, Node<'a>>,
        mut already_visited: HashSet<&'a str>,
        node: &'a str,
    ) -> i64 {
        let node = nodes.get(node).unwrap();

        if node.val == "end" {
            return 1;
        }
        if node.is_small && !already_visited.insert(node.val) {
            return 0;
        }

        let mut to_visit = vec![];
        for node in node.next.iter() {
            to_visit.push(*node);
        }

        to_visit
            .iter()
            .map(|node| visit_node(nodes, already_visited.clone(), node))
            .sum()
    }

    let mut nodes = HashMap::new();

    for line in input.lines() {
        let mut iter = line.trim().split('-');
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();

        nodes.entry(from).or_insert(Node::new(from)).next.push(to);
        nodes.entry(to).or_insert(Node::new(to)).next.push(from);
    }

    visit_node(&mut nodes, HashSet::new(), "start").into()
}

pub fn day12_part2(input: &str) -> SolutionType {
    fn visit_node<'a>(
        nodes: &mut HashMap<&'a str, Node<'a>>,
        mut already_visited: HashSet<&'a str>,
        mut interesting_visited: bool,
        interesting_node: &'a str,
        node: &'a str,

        mut path: Vec<&'a str>,
    ) -> Vec<Vec<&'a str>> {
        path.push(node);
        let node = nodes.get(node).unwrap();

        if node.val == "end" {
            return vec![path];
        }
        if node.val == interesting_node {
            if interesting_visited && !already_visited.insert(node.val) {
                return vec![];
            }
            interesting_visited = true;
        } else {
            if node.is_small && !already_visited.insert(node.val) {
                return vec![];
            }
        }

        let mut to_visit = vec![];
        for node in node.next.iter() {
            to_visit.push(*node);
        }

        to_visit
            .iter()
            .map(|node| {
                visit_node(
                    nodes,
                    already_visited.clone(),
                    interesting_visited,
                    interesting_node,
                    node,
                    path.clone(),
                )
            })
            .flatten()
            .collect()
    }

    let mut nodes = HashMap::new();

    for line in input.lines() {
        let mut iter = line.trim().split('-');
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();

        nodes.entry(from).or_insert(Node::new(from)).next.push(to);
        nodes.entry(to).or_insert(Node::new(to)).next.push(from);
    }

    let mut paths = vec![];
    for (interesting, node) in nodes.clone() {
        if !node.is_small || node.val == "start" || node.val == "end" {
            continue;
        }

        paths.extend_from_slice(&visit_node(
            &mut nodes,
            HashSet::new(),
            false,
            interesting,
            "start",
            vec![],
        ));
    }
    paths.sort_unstable();
    paths.dedup();
    paths.len().into()
}

#[derive(Clone)]
struct Node<'a> {
    val: &'a str,
    is_small: bool,
    next: Vec<&'a str>,
}

impl<'a> Node<'a> {
    pub fn new(val: &'a str) -> Self {
        Self {
            val,
            is_small: val.chars().next().unwrap().is_ascii_lowercase(),
            next: vec![],
        }
    }
}

#[test]
#[cfg(test)]
fn test() {
    let input = "start-A
                start-b
                A-c
                A-b
                b-d
                A-end
                b-end";
    let input2 = "dc-end
                HN-start
                start-kj
                dc-start
                dc-HN
                LN-dc
                HN-end
                kj-sa
                kj-HN
                kj-dc";
    let input3 = "fs-end
                he-DX
                fs-he
                start-DX
                pj-DX
                end-zg
                zg-sl
                zg-pj
                pj-he
                RW-he
                fs-DX
                pj-RW
                zg-RW
                start-pj
                he-WI
                zg-he
                pj-fs
                start-RW";

    assert_eq!(day12(input), SolutionType::Int(10));
    assert_eq!(day12(input2), SolutionType::Int(19));
    assert_eq!(day12(input3), SolutionType::Int(226));

    assert_eq!(day12_part2(input), SolutionType::Int(36));
    assert_eq!(day12_part2(input2), SolutionType::Int(103));
    assert_eq!(day12_part2(input3), SolutionType::Int(3509));
}
