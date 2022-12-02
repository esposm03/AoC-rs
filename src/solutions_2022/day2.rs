use crate::SolutionType;

pub fn day2(input: &str) -> SolutionType {
    let mut total_score = 0;

    for round in input.trim().lines() {
        let round = round.trim();

        total_score += match round.chars().nth(2).unwrap() {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => unreachable!(),
        };
        total_score += match (round.chars().next().unwrap(), round.chars().nth(2).unwrap()) {
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            _ => unreachable!(),
        }
    }

    SolutionType::Int(total_score)
}

pub fn day2_part2(input: &str) -> SolutionType {
    let mut total_score = 0;

    for round in input.trim().lines() {
        let round = round.trim();

        total_score += match round.chars().nth(2).unwrap() {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => unreachable!(),
        };
        total_score += match (round.chars().next().unwrap(), round.chars().nth(2).unwrap()) {
            ('A', 'X') => 3, // rock, scissors
            ('A', 'Y') => 1, // rock, rock
            ('A', 'Z') => 2, // rock, paper
            ('B', 'X') => 1, // paper, rock
            ('B', 'Y') => 2, // paper, paper
            ('B', 'Z') => 3, // paper, scissors
            ('C', 'X') => 2, // scissors, paper
            ('C', 'Y') => 3, // scissors, scissors
            ('C', 'Z') => 1, // scissors, rock
            _ => unreachable!(),
        }
    }

    SolutionType::Int(total_score)
}
