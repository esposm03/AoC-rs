use crate::SolutionType;

pub fn day3(input: &str) -> SolutionType {
    let num_bits = input.split('\n').next().unwrap().chars().count();
    let mut repetitions = vec![0; num_bits];

    for number in input.trim().split('\n') {
        for (i, ch) in number.char_indices() {
            if ch == '1' {
                repetitions[num_bits - i - 1] += 1;
            }
        }
    }

    let nums = input.trim().split('\n').count();
    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, &repetition) in repetitions.iter().enumerate() {
        let bit = if repetition > nums / 2 { 1 } else { 0 };

        gamma += bit * 2_i64.pow(i as u32);
        epsilon += (1 - bit) * 2_i64.pow(i as u32);
    }

    SolutionType::Int(gamma * epsilon)
}

#[test]
#[cfg(test)]
fn test() {
    assert_eq!(
        day3("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"),
        SolutionType::Int(198),
    );
}
