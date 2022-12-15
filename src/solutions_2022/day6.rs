use crate::Solution;

pub fn day6(input: &str) -> Solution {
    let mut iter = input.trim().chars();
    let mut a = iter.next().unwrap();
    let mut b = iter.next().unwrap();
    let mut c = iter.next().unwrap();
    let mut d = iter.next().unwrap();
    let mut received: i64 = 4;

    if a != b && a != c && a != d && b != c && b != d && c != d {
        return received.into();
    }

    for i in iter {
        match received % 4 {
            0 => a = i,
            1 => b = i,
            2 => c = i,
            3 => d = i,
            _ => unreachable!(),
        }

        received += 1;

        if a != b && a != c && a != d && b != c && b != d && c != d {
            return received.into();
        }
    }

    unreachable!()
}

pub fn day6_part2(input: &str) -> Solution {
    let input = input.chars().collect::<Vec<_>>();
    let mut last_received = [
        input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7], input[8],
        input[9], input[10], input[11], input[12], input[13],
    ];
    let mut received: usize = 14;

    let mut vec = Vec::from(last_received);
    vec.sort();
    vec.dedup();
    if vec.len() == 14 {
        return received.into();
    }

    for ch in input.iter().skip(14) {
        last_received[received % 14] = *ch;
        received += 1;

        let mut vec = Vec::from(last_received);
        vec.sort();
        vec.dedup();
        if vec.len() == 14 {
            return received.into();
        }
    }

    unreachable!()
}

#[test]
#[cfg(test)]
#[rustfmt::skip]
fn test() {
    assert_eq!(day6("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Solution::Int(7));
    assert_eq!(day6("bvwbjplbgvbhsrlpgdmjqwftvncz"), Solution::Int(5));
    assert_eq!(day6("nppdvjthqldpwncqszvftbrmjlhg"), Solution::Int(6));
    assert_eq!(day6("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Solution::Int(10));
    assert_eq!(day6("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Solution::Int(11));

    assert_eq!(day6_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Solution::Int(19));
    assert_eq!(day6_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), Solution::Int(23));
    assert_eq!(day6_part2("nppdvjthqldpwncqszvftbrmjlhg"), Solution::Int(23));
    assert_eq!(day6_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Solution::Int(29));
    assert_eq!(day6_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Solution::Int(26));
}
