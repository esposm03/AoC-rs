use crate::SolutionType;

pub fn day4(input: &str) -> SolutionType {
    let mut count = 0;

    for line in input.trim().lines() {
        let mut elves = line.trim().split(',');

        let mut elf1 = elves.next().unwrap().split('-');
        let e1a = elf1.next().unwrap().parse::<i64>().unwrap();
        let e1b = elf1.next().unwrap().parse::<i64>().unwrap();

        let mut elf2 = elves.next().unwrap().split('-');
        let e2a = elf2.next().unwrap().parse::<i64>().unwrap();
        let e2b = elf2.next().unwrap().parse::<i64>().unwrap();

        if (e1a >= e2a && e1b <= e2b) || (e1a <= e2a && e1b >= e2b) {
            count += 1;
        }
    }

    SolutionType::Int(count)
}

pub fn day4_part2(input: &str) -> SolutionType {
    let mut count = 0;

    for line in input.trim().lines() {
        let mut elves = line.trim().split(',');

        let mut elf1 = elves.next().unwrap().split('-');
        let e1a = elf1.next().unwrap().parse::<i64>().unwrap();
        let e1b = elf1.next().unwrap().parse::<i64>().unwrap();
        let elf1 = e1a..=e1b;

        let mut elf2 = elves.next().unwrap().split('-');
        let e2a = elf2.next().unwrap().parse::<i64>().unwrap();
        let e2b = elf2.next().unwrap().parse::<i64>().unwrap();
        let elf2 = e2a..=e2b;

        count += if elf1.filter(|i| elf2.contains(i)).count() != 0 {
            1
        } else {
            0
        };
    }

    SolutionType::Int(count as i64)
}

#[test]
#[cfg(test)]
fn test() {
    let input = "2-4,6-8
                2-3,4-5
                5-7,7-9
                2-8,3-7
                6-6,4-6
                2-6,4-8";

    assert_eq!(day4(input), SolutionType::Int(2));
    assert_eq!(day4_part2(input), SolutionType::Int(4));
}
