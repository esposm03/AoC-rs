use crate::SolutionType;

pub fn day5(input: &str) -> SolutionType {
    input.lines().map(seat).max().unwrap().into()
}

pub fn day5_part2(input: &str) -> SolutionType {
    let mut vec = vec![];

    for seat in input.lines().map(seat) {
        vec.push(seat);
    }
    vec.sort_unstable();

    for i in 1..vec.len() - 1 {
        if vec[i] == vec[i - 1] + 2 {
            return (vec[i] - 1).into();
        }
    }

    unreachable!();
}

fn seat(line: &str) -> i64 {
    let row = line[..7]
        .chars()
        .map(|ch| if ch == 'F' { '0' } else { '1' })
        .collect::<String>();
    let col = line[7..]
        .chars()
        .map(|ch| if ch == 'L' { '0' } else { '1' })
        .collect::<String>();

    let row = i64::from_str_radix(&row, 2).unwrap();
    let col = i64::from_str_radix(&col, 2).unwrap();

    row * 8 + col
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(seat("FBFBBFFRLR"), 357);
    assert_eq!(seat("BFFFBBFRRR"), 567);
    assert_eq!(seat("FFFBBBFRRR"), 119);
    assert_eq!(seat("BBFFBBFRLL"), 820);
}
