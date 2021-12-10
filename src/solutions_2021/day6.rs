use crate::SolutionType;

pub fn day6(input: &str) -> SolutionType {
    run_simulation(input, 80).into()
}

pub fn day6_part2(input: &str) -> SolutionType {
    run_simulation(input, 256).into()
}

fn run_simulation(input: &str, n: i64) -> i64 {
    let mut lanternfishes = vec![0i64; 9];

    let temp = input
        .trim()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for n in 0..9 {
        lanternfishes[n] = temp.iter().filter(|i| **i == n as i64).count() as i64;
    }

    for _ in 0..n {
        let old = lanternfishes.clone();
        lanternfishes[0] = old[1];
        lanternfishes[1] = old[2];
        lanternfishes[2] = old[3];
        lanternfishes[3] = old[4];
        lanternfishes[4] = old[5];
        lanternfishes[5] = old[6];
        lanternfishes[6] = old[7] + old[0];
        lanternfishes[7] = old[8];
        lanternfishes[8] = old[0];
    }

    lanternfishes.iter().sum::<i64>()
}

#[test]
#[cfg(test)]
fn test() {
    assert_eq!(day6("3,4,3,1,2"), SolutionType::Int(5934));
    assert_eq!(day6_part2("3,4,3,1,2"), SolutionType::Int(26984457539));
}
