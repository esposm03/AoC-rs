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

pub fn day3_part2(input: &str) -> SolutionType {
    let oxygen_generator_rating = extract_rating(input, 1);
    let co2_scrubber_rating = extract_rating(input, 0);

    SolutionType::Int(bin_to_number(oxygen_generator_rating) * bin_to_number(co2_scrubber_rating))
}

fn extract_rating(input: &str, rounding: u8) -> &str {
    let input = input.trim();
    let num_bits = input.split('\n').next().unwrap().chars().count();
    let mut numbers = input.split('\n').collect::<Vec<_>>();
    let mut bit_index = num_bits;

    while numbers.len() != 1 {
        bit_index -= 1;
        let mut ones_count = 0;
        let mut zeros_count = 0;
        for number in &numbers {
            if number.chars().nth(num_bits - bit_index - 1).unwrap() == '1' {
                ones_count += 1;
            } else {
                zeros_count += 1;
            }
        }

        let most_common_bit = if rounding == 1 {
            if ones_count == zeros_count {
                '1'
            } else if ones_count > zeros_count {
                '1'
            } else {
                '0'
            }
        } else {
            if ones_count == zeros_count {
                '0'
            } else if ones_count > zeros_count {
                '0'
            } else {
                '1'
            }
        };

        numbers.retain(|n| n.chars().nth(num_bits - bit_index - 1).unwrap() == most_common_bit);
    }

    numbers[0]
}

fn bin_to_number(binary_string: &str) -> i64 {
    let mut res = 0;
    for (i, ch) in binary_string.chars().rev().enumerate() {
        let bit = if ch == '1' { 1 } else { 0 };
        res += bit * 2_i64.pow(i as u32);
    }
    res
}

#[test]
#[cfg(test)]
fn test() {
    assert_eq!(
        day3("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"),
        SolutionType::Int(198),
    );

    assert_eq!(
        day3_part2(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        ),
        SolutionType::Int(230),
    );
}
