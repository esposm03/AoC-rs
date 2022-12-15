use crate::Solution;

pub fn day11(input: &str) -> Solution {
    let mut monkeys = vec![];

    for monkey in input.trim().split("\n\n") {
        let mut iter = monkey.trim().lines().skip(1);
        let items = iter.next().unwrap();
        let items = items[18..].trim();
        let items = items.split(", ").map(|i| i.parse().unwrap()).collect();
        let operation = iter.next().unwrap();
        let is_sum = operation.contains('+');
        let op_a = operation.split(' ').nth(5).unwrap().parse().unwrap_or(0);
        let op_b = operation.split(' ').nth(7).unwrap().parse().unwrap_or(0);
        let test = iter.next().unwrap();
        let test = test.split(' ').nth(5).unwrap().parse().unwrap();
        let if_true = iter.next().unwrap()[29..].parse().unwrap();
        let if_false = (&iter.next().unwrap()[30..]).parse().unwrap();

        monkeys.push(Monkey {
            items,
            is_sum,
            op_a,
            op_b,
            test,
            if_true,
            if_false,
        });
    }

    let mut inspections = vec![0; monkeys.len()];

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in &monkey.items {
                inspections[i] += 1;

                let a = if monkey.op_a == 0 { *item } else { monkey.op_a };
                let b = if monkey.op_b == 0 { *item } else { monkey.op_b };
                let mut worry = if monkey.is_sum { a + b } else { a * b };
                worry /= 3;
                if worry % monkey.test == 0 {
                    monkeys[monkey.if_true].items.push(worry);
                } else {
                    monkeys[monkey.if_false].items.push(worry);
                }
                monkeys[i].items.remove(0);
            }
        }
    }

    inspections.sort_unstable();
    inspections.reverse();
    Solution::Int(inspections[0] * inspections[1])
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    is_sum: bool,
    op_a: i64,
    op_b: i64,
    test: i64,
    if_true: usize,
    if_false: usize,
}

#[test]
#[cfg(test)]
fn test() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    assert_eq!(day11(input), Solution::Int(10605));
}
