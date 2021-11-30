use crate::SolutionType;
use std::collections::HashMap;

pub fn day7(input: &str) -> SolutionType {
    let map = input
        .lines()
        .map(|ln| parsing::line(ln).unwrap().1)
        .collect::<HashMap<_, _>>();

    assert!(map.contains_key("shiny gold"));

    let mut count = 0_i64;
    for container in map.keys() {
        if has_gold(&map, container) {
            count += 1;
        }
    }

    count.into()
}

pub fn day7_part2(input: &str) -> SolutionType {
    let map = input
        .lines()
        .map(|ln| parsing::line(ln).unwrap().1)
        .collect::<HashMap<_, _>>();

    (contents(&map, "shiny gold") - 1).into()
}

fn has_gold(map: &HashMap<&str, Vec<(i64, &str)>>, name: &str) -> bool {
    map.get(name)
        .unwrap()
        .iter()
        .any(|i| i.1 == "shiny gold" || has_gold(map, i.1))
}

fn contents(map: &HashMap<&str, Vec<(i64, &str)>>, name: &str) -> i64 {
    map.get(name)
        .unwrap()
        .iter()
        .map(|(number, child)| number * contents(map, child))
        .sum::<i64>()
        + 1
}

mod parsing {
    use nom::{
        bytes::complete as bytes, character::complete as character, combinator, multi, IResult,
    };

    type Bag<'a> = (i64, &'a str);

    pub fn line(i: &str) -> IResult<&str, (&str, Vec<Bag>)> {
        let (i, container) = bag(i)?;
        let (i, _) = bytes::tag(" bags contain ")(i)?;
        let (i, contents) = nom::branch::alt((multi::many1(content), no_content))(i)?;

        Ok((i, (container, contents)))
    }

    fn no_content(i: &str) -> IResult<&str, Vec<(i64, &str)>> {
        let (i, _) = bytes::tag("no other bags")(i)?;
        Ok((i, vec![]))
    }

    fn content(i: &str) -> IResult<&str, (i64, &str)> {
        let (i, _) = character::space0(i)?;
        let (i, num) = character::digit1(i)?;
        let (i, _) = bytes::tag(" ")(i)?;
        let (i, bag) = bag(i)?;
        let (i, _) = bytes::tag(" bag")(i)?;

        let (i, _) = combinator::opt(bytes::tag("s"))(i)?;
        let (i, _) = combinator::opt(bytes::tag(", "))(i)?;

        Ok((i, (num.parse().unwrap(), bag)))
    }

    fn bag(i: &str) -> IResult<&str, &str> {
        let (i, _) = character::space0(i)?;
        let start = i;

        let (i, _) = character::alpha1(i)?;
        let (i, _) = character::space0(i)?;
        let (i, _) = character::alpha1(i)?;

        Ok((i, &start[..(start.len() - i.len())]))
    }

    #[test]
    #[cfg(test)]
    fn parsing_test() {
        assert_eq!(
            line("light red bags contain 1 bright white bag, 2 muted yellow bags.")
                .unwrap()
                .1,
            ("light red", vec![(1, "bright white"), (2, "muted yellow")])
        );
        assert_eq!(
            line("shiny gold bags contain 2 dark red bags.").unwrap().1,
            ("shiny gold", vec![(2, "dark red")])
        );
    }
}

#[cfg(test)]
#[test]
fn test() {
    let input1 = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

    let map = input1
        .lines()
        .map(|ln| parsing::line(ln).unwrap().1)
        .collect::<HashMap<_, _>>();

    assert!(has_gold(&map, "bright white"));
    assert!(has_gold(&map, "light red"));
    assert_eq!(day7_part2(input1), 32_i64.into());

    let input2 = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

    let map = input1
        .lines()
        .map(|ln| parsing::line(ln).unwrap().1)
        .collect::<HashMap<_, _>>();

    assert!(has_gold(&map, "bright white"));
    assert!(has_gold(&map, "light red"));
    assert_eq!(day7_part2(input2), 126_i64.into());
}
