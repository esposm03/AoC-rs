use crate::SolutionType;

pub fn day1(input: &str) -> SolutionType {
    let mut sum = 0.0;

    for line in input.lines() {
        let num = line.parse::<f32>().unwrap();
        sum += (num / 3.0).floor() - 2.0;
    }

    (sum as i64).into()
}

pub fn day1_part2(input: &str) -> SolutionType {
    let mut sum = 0.0;

    for module_mass in input.lines().map(|line| line.parse::<f32>().unwrap()) {
        let mut fuel_mass = (module_mass / 3.0).floor() - 2.0;
        while fuel_mass > 0.0 {
            sum += fuel_mass;
            fuel_mass = (fuel_mass / 3.0).floor() - 2.0;
        }
    }

    (sum as i64).into()
}
