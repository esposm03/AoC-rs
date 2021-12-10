use crate::SolutionType;

pub fn day6(input: &str) -> SolutionType {
    let mut lanternfishes = input
        .trim()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for fish in lanternfishes.iter_mut() {
        *fish = 6 - *fish;
    }

    run_simulation(&mut lanternfishes, 80).into()
}

pub fn day6_part2(input: &str) -> SolutionType {
    let mut lanternfishes = input
        .trim()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for fish in lanternfishes.iter_mut() {
        *fish = 6 - *fish;
    }

    run_simulation(&mut lanternfishes, 256).into()
}

fn run_simulation(lanternfishes: &mut Vec<i64>, n: i64) -> usize {
    let mut sum = lanternfishes.len();

    while let Some(o) = lanternfishes.pop() {
        print!("Pushing all {} children of {}: ", num_of_children(n, o), o);
        for z in 0..num_of_children(n, o) {
            let o_child = child_offset(z as i64, o);
            print!("{} ", o_child);
            lanternfishes.push(o_child);
        }
        println!();

        sum += num_of_children(n, o);
    }

    sum
}

/// Return the number of childs spawned in `n` days by a fish with offset `o`
fn num_of_children(n: i64, o: i64) -> usize {
    ((n + o) / 7) as usize
}

fn child_offset(z: i64, o: i64) -> i64 {
    -(7 * z) - (7 - o) - 2
}

#[test]
#[cfg(test)]
fn test() {
    assert_eq!(day6("3,4,3,1,2"), SolutionType::Int(5934));
    assert_eq!(day6_part2("3,4,3,1,2"), SolutionType::Int(26984457539));
}

