use crate::SolutionType;

pub fn day2(input: &str) -> SolutionType {
    let mut horizontal = 0;
    let mut vertical = 0;

    for row in input.split('\n') {
        let motion = row.split(' ').next().unwrap();
        let amount: i64 = row.split(' ').nth(1).unwrap().parse().unwrap();

        match motion {
            "forward" => horizontal += amount,
            "up" => vertical -= amount,
            "down" => vertical += amount,
            i => panic!("invalid motion `{}`", i),
        }
    }

    SolutionType::Int(horizontal * vertical)
}

pub fn day2_part2(input: &str) -> SolutionType {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    for row in input.split('\n') {
        let motion = row.split(' ').next().unwrap();
        let amount: i64 = row.split(' ').nth(1).unwrap().parse().unwrap();

        match motion {
            "forward" => {
                horizontal += amount;
                vertical += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            i => panic!("invalid motion `{}`", i),
        }
    }

    SolutionType::Int(horizontal * vertical)
}

#[test]
#[cfg(test)]
fn test() {
    assert_eq!(
        day2("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
        SolutionType::Int(150)
    );
    assert_eq!(
        day2_part2("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
        SolutionType::Int(900)
    );
}
