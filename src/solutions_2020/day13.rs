use crate::Solution;

pub fn day13(input: &str) -> Solution {
    let mut lines = input.split('\n');
    let min = lines.next().unwrap().parse::<i64>().unwrap();
    let ids = lines.next().unwrap().split(',');

    let (id, time) = ids
        .filter_map(|i| i.parse::<i64>().ok())
        .map(|id| {
            let mut i = id;
            while i < min {
                i += id;
            }
            (id, i)
        })
        .min_by_key(|(_, i)| *i)
        .unwrap();

    (id * (time - min)).into()
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(day13("939\n7,13,x,x,59,x,31,19"), 295_i64.into());
}
