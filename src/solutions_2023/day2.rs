use std::cmp::max;

use crate::Solution;

pub fn day2(input: &str) -> Solution {
    let mut possible = 0;

    for (id, game) in input.trim().split('\n').enumerate() {
        let game = game.split_once(':').unwrap().1.trim();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in game.split(';') {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cube in set.trim().split(',') {
                let color = cube.trim().split_once(' ').unwrap().1;
                let amt = cube.trim().split_once(' ').unwrap().0;
                let amt: u32 = amt.parse().unwrap();

                match color {
                    "red" => red += amt,
                    "green" => green += amt,
                    "blue" => blue += amt,
                    _ => unreachable!(),
                }
            }

            max_red = max(max_red, red);
            max_blue = max(max_blue, blue);
            max_green = max(max_green, green);
        }

        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            possible += id + 1;
        }
    }

    Solution::Int(possible as i64)
}

pub fn day2_part2(input: &str) -> Solution {
    let mut score = 0i64;

    for game in input.trim().split('\n') {
        let game = game.split_once(':').unwrap().1.trim();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in game.split(';') {
            for cube in set.trim().split(',') {
                let color = cube.trim().split_once(' ').unwrap().1;
                let amt = cube.trim().split_once(' ').unwrap().0;
                let amt: i64 = amt.parse().unwrap();

                match color {
                    "red" => red = max(red, amt),
                    "green" => blue = max(blue, amt),
                    "blue" => green = max(green, amt),
                    _ => unreachable!(),
                }
            }
        }

        score += red * green * blue;
    }

    Solution::Int(score)
}

#[test]
#[cfg(test)]
fn test() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(day2(input), Solution::Int(8));
    assert_eq!(day2_part2(input), Solution::Int(2286));
}
