use crate::SolutionType;

pub fn day4(input: &str) -> SolutionType {
    let passports = input.split("\n\n").filter(|i| i != &"");

    passports
        .filter(|passport| {
            let keys = passport
                .split(|ch: char| ch.is_ascii_whitespace())
                .filter(|i| i != &"")
                .map(|entry| entry.split(':').next().unwrap())
                .collect::<Vec<&str>>();

            keys.contains(&"byr")
                && keys.contains(&"iyr")
                && keys.contains(&"eyr")
                && keys.contains(&"hgt")
                && keys.contains(&"hcl")
                && keys.contains(&"ecl")
                && keys.contains(&"pid")
        })
        .count()
        .into()
}

pub fn day4_part2(input: &str) -> SolutionType {
    let passports = input.split("\n\n").filter(|i| i != &"");

    passports
        .filter(|passport| {
            let entries = passport
                .split(|ch: char| ch.is_ascii_whitespace())
                .filter(|i| i != &"")
                .map(|entry| {
                    let mut a = entry.split(':');
                    (a.next().unwrap(), a.next().unwrap())
                });

            let mut res = 0;
            for (key, val) in entries {
                match key {
                    "byr" => {
                        let num = val.parse::<u16>().unwrap();
                        if (1920..=2002).contains(&num) {
                            res += 1
                        }
                    }
                    "iyr" => {
                        let num = val.parse::<u16>().unwrap();
                        if (2010..=2020).contains(&num) {
                            res += 2
                        }
                    }
                    "eyr" => {
                        let num = val.parse::<u16>().unwrap();
                        if (2020..=2030).contains(&num) {
                            res += 4
                        }
                    }
                    "hgt" => {
                        if val.len() != 2 {
                            let num = val[..val.len() - 2].parse::<u16>().unwrap();
                            if val.ends_with("cm") && 150 <= num && num <= 193
                                || val.ends_with("in") && 59 <= num && num <= 76
                            {
                                res += 8
                            }
                        }
                    }
                    "hcl" => {
                        if val.starts_with('#')
                            && val[1..].chars().all(|ch| ch.is_ascii_alphanumeric())
                        {
                            res += 16
                        }
                    }
                    "ecl" => {
                        if val == "amb"
                            || val == "blu"
                            || val == "brn"
                            || val == "gry"
                            || val == "grn"
                            || val == "hzl"
                            || val == "oth"
                        {
                            res += 32
                        }
                    }
                    "pid" => {
                        if val.len() == 9 && val.parse::<u64>().is_ok() {
                            res += 64
                        }
                    }
                    "cid" => {}
                    _ => unreachable!("Invalid key"),
                }
            }

            res == 127
        })
        .count()
        .into()
}

#[test]
#[cfg(test)]
fn test() {
    let input_part1 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in\n";

    assert_eq!(day4(input_part1), 2_i64.into());

    let part2_valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
    let part2_invalid = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";

    assert_eq!(day4_part2(part2_valid), 4_i64.into());
    assert_eq!(day4_part2(part2_invalid), 0_i64.into());
}
