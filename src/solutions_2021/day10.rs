use crate::SolutionType;
use Parens::*;

pub fn day10(input: &str) -> SolutionType {
    let mut score = 0;
    for line in input.lines() {
        let mut stack = vec![];
        for char in line.trim().chars() {
            match char {
                '(' => stack.push(Round),
                '[' => stack.push(Square),
                '{' => stack.push(Curly),
                '<' => stack.push(Angled),
                ')' => {
                    if let Some(Round) = stack.pop() {
                    } else {
                        score += 3;
                        break;
                    }
                }
                ']' => {
                    if let Some(Square) = stack.pop() {
                    } else {
                        score += 57;
                        break;
                    }
                }
                '}' => {
                    if let Some(Curly) = stack.pop() {
                    } else {
                        score += 1197;
                        break;
                    }
                }
                '>' => {
                    if let Some(Angled) = stack.pop() {
                    } else {
                        score += 25137;
                        break;
                    }
                }
                e => unreachable!("what is {e}?"),
            }
        }
    }

    SolutionType::Int(score)
}

pub fn day10_part2(input: &str) -> SolutionType {
    let mut scores = vec![];

    for line in input.lines() {
        let mut corrupted = false;
        let mut stack = vec![];

        for char in line.trim().chars() {
            match char {
                '(' => stack.push(Round),
                '[' => stack.push(Square),
                '{' => stack.push(Curly),
                '<' => stack.push(Angled),
                ')' => match stack.pop() {
                    Some(Round) => {}
                    Some(_) => corrupted = true,
                    None => break,
                },
                ']' => match stack.pop() {
                    Some(Square) => {}
                    Some(_) => corrupted = true,
                    None => break,
                },
                '}' => match stack.pop() {
                    Some(Curly) => {}
                    Some(_) => corrupted = true,
                    None => break,
                },
                '>' => match stack.pop() {
                    Some(Angled) => {}
                    Some(_) => corrupted = true,
                    None => break,
                },
                e => unreachable!("what is {e}?"),
            }
        }

        if corrupted {
            continue;
        }

        stack.reverse();
        if !stack.is_empty() {
            let mut score = 0;
            for leftover in stack {
                score *= 5;
                score += match leftover {
                    Round => 1,
                    Square => 2,
                    Curly => 3,
                    Angled => 4,
                };
            }
            scores.push(score);
        }
    }

    scores.sort();
    SolutionType::Int(scores[scores.len() / 2])
}

#[derive(PartialEq, Eq)]
enum Parens {
    Round,
    Square,
    Curly,
    Angled,
}

#[test]
#[cfg(test)]
fn test() {
    let input = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    assert_eq!(day10(input), SolutionType::Int(26397));
    assert_eq!(day10_part2(input), SolutionType::Int(288957));
}
