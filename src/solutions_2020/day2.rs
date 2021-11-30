use crate::SolutionType;
use nom::{bytes::complete as bytes, character::complete as character, IResult};

pub fn day2(input: &str) -> SolutionType {
    input
        .lines()
        .map(|line| Rule::parse(line).unwrap())
        .filter(|(_, r)| r.is_valid_old())
        .count()
        .into()
}

pub fn day2_part2(input: &str) -> SolutionType {
    input
        .lines()
        .map(|line| Rule::parse(line).unwrap())
        .filter(|(_, r)| r.is_valid_toboggan())
        .count()
        .into()
}

pub struct Rule<'a> {
    start: u32,
    end: u32,
    letter: char,
    password: &'a str,
}

impl Rule<'_> {
    pub fn parse(input: &str) -> IResult<&str, Rule> {
        let (i, start) = character::digit1(input)?;
        let (i, _) = bytes::tag("-")(i)?;
        let (i, end) = character::digit1(i)?;
        let (i, _) = character::space1(i)?;
        let (i, letter) = character::anychar(i)?;
        let (i, _) = bytes::tag(": ")(i)?;
        let (i, password) = character::alpha1(i)?;

        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        Ok((
            i,
            Rule {
                start,
                end,
                letter,
                password,
            },
        ))
    }

    pub fn is_valid_old(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|ch| *ch == self.letter)
            .count() as u32;
        count >= self.start && count <= self.end
    }

    pub fn is_valid_toboggan(&self) -> bool {
        let start = self.password.chars().nth(self.start as usize - 1).unwrap();
        let end = self.password.chars().nth(self.end as usize - 1).unwrap();
        let letter = self.letter;

        (letter == start) ^ (letter == end)
    }
}

#[cfg(test)]
#[test]
fn test() {
    let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n";
    assert_eq!(day2(input), 2_i64.into());
}
