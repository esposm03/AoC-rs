use crate::Solution;

pub fn day8(input: &str) -> Solution {
    let mut sum = 0;

    for line in input.lines() {
        let outputs = line.split('|').nth(1).unwrap();

        for digit in outputs.split(' ') {
            let l = digit.len();
            if l == 2 || l == 4 || l == 3 || l == 7 {
                sum += 1;
            }
        }
    }

    Solution::Int(sum)
}

pub fn day8_part2(input: &str) -> Solution {
    let mut sum = 0;

    for line in input.lines() {
        let inputs = line.split('|').next().unwrap();
        let outputs = line.split('|').nth(1).unwrap();

        let segment_e = find_char_occurring(inputs, 4);
        let segment_b = find_char_occurring(inputs, 6);
        let segment_f = find_char_occurring(inputs, 9);

        let segment_c = find_digit_with_size(inputs, 2)
            .chars()
            .find(|ch| *ch != segment_f)
            .unwrap();

        let segment_d = find_extra_char(
            find_digit_with_size(inputs, 4),
            &[segment_b, segment_c, segment_f],
        );

        let segment_a = find_extra_char(find_digit_with_size(inputs, 3), &[segment_c, segment_f]);
        let segment_g = find_extra_char(
            find_digit_with_size(inputs, 7),
            &[
                segment_a, segment_b, segment_c, segment_d, segment_e, segment_f,
            ],
        );

        let mut outputs2 = String::with_capacity(outputs.len());
        for ch in outputs.chars() {
            outputs2.push(if ch == segment_a {
                'a'
            } else if ch == segment_b {
                'b'
            } else if ch == segment_c {
                'c'
            } else if ch == segment_d {
                'd'
            } else if ch == segment_e {
                'e'
            } else if ch == segment_f {
                'f'
            } else if ch == segment_g {
                'g'
            } else {
                ch
            });
        }
        let outputs = outputs2;

        let mut number = 0;
        let mut magnitude = 1000;
        for digit in outputs.trim().split(' ') {
            let mut vec = digit.chars().collect::<Vec<_>>();
            vec.sort();
            let digit = vec.iter().collect::<String>();

            number += magnitude
                * match digit.as_str() {
                    "abcefg" => 0,
                    "cf" => 1,
                    "acdeg" => 2,
                    "acdfg" => 3,
                    "bcdf" => 4,
                    "abdfg" => 5,
                    "abdefg" => 6,
                    "acf" => 7,
                    "abcdefg" => 8,
                    "abcdfg" => 9,
                    e => unreachable!("what digit is {e}?"),
                };
            magnitude /= 10;
        }
        sum += number;
    }

    Solution::Int(sum)
}

fn find_char_occurring(haystack: &str, wanted_occurrences: usize) -> char {
    let all_segments = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let count_occurrences = |ch: char| haystack.chars().filter(|chi| *chi == ch).count();

    *all_segments
        .iter()
        .find(|ch| count_occurrences(**ch) == wanted_occurrences)
        .unwrap()
}

fn find_digit_with_size(haystack: &str, size: usize) -> &str {
    for word in haystack.split(' ') {
        if word.len() == size {
            return word;
        }
    }
    unreachable!()
}

fn find_extra_char(haystack: &str, known_digits: &[char]) -> char {
    for ch in haystack.chars() {
        if known_digits.iter().all(|known_ch| *known_ch != ch) {
            return ch;
        }
    }

    unreachable!()
}

#[test]
#[cfg(test)]
fn test() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    assert_eq!(day8(input), Solution::Int(26));
}

#[test]
#[cfg(test)]
fn test_part2() {
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    let input2 =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    assert_eq!(day8_part2(input), Solution::Int(5353));
    assert_eq!(day8_part2(input2), Solution::Int(61229));
}
