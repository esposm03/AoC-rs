use crate::SolutionType;

pub fn day6(input: &str) -> SolutionType {
    let mut lanternfishes = input.trim().split(',').map(|i| i.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut temp_storage = vec![];

    //println!("Initial state: {:?}", lanternfishes);

    for i in 0..80 {
        for fish in &mut lanternfishes {
            if *fish != 0 {
                *fish -= 1;
            } else {
                temp_storage.push(8);
                *fish = 6;
            }
        }

        lanternfishes.append(&mut temp_storage);
        //println!("After {} days: {:?}", i, lanternfishes);
    }

    lanternfishes.len().into()
}

pub fn day6_part2(input: &str) -> SolutionType {
    let mut lanternfishes = input.trim().split(',').map(|i| i.parse::<i64>().unwrap()).map(|i| 6 - i).collect::<Vec<_>>();
    let mut temp_storage = vec![];

    //println!("Initial state: {:?}", lanternfishes);

    for mut fish in lanternfishes.clone() {
        fish += 256;
        let spawned = fish / 7;
        fish = fish % 7;
    }

    for i in 0..256 {
        for fish in &mut lanternfishes {
            if *fish != 0 {
                *fish -= 1;
            } else {
                temp_storage.push(8);
                *fish = 6;
            }
        }

        lanternfishes.append(&mut temp_storage);
        //println!("After {} days: {:?}", i, lanternfishes);
    }

    lanternfishes.len().into()
}

#[test]
#[cfg(test)]
fn test() {
    assert_eq!(day6("3,4,3,1,2"), SolutionType::Int(5934));
    assert_eq!(day6_part2("3,4,3,1,2"), SolutionType::Int(26984457539));
}