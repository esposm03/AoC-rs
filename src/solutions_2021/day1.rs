use crate::SolutionType;

pub fn day1(input: &str) -> SolutionType {
    let numbers = input
        .split('\n')
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut count = 0;
    let mut previous = i64::MAX;
    for i in numbers {
        if i > previous {
            count += 1;
        }
        previous = i;
    }

    SolutionType::Int(count)
}

pub fn day1_part2(input: &str) -> SolutionType {
    let numbers = input
        .split('\n')
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let window_sums = numbers
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<_>>();

    let mut count = 0;
    let mut previous = i64::MAX;
    for i in window_sums {
        if i > previous {
            count += 1;
        }
        previous = i;
    }

    SolutionType::Int(count)
}

#[test]
#[cfg(test)]
fn test() {
    assert_eq!(
        day1("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"),
        SolutionType::Int(7)
    );
    assert_eq!(
        day1_part2("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"),
        SolutionType::Int(5)
    );
}
