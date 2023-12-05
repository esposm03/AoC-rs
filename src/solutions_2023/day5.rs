use std::cmp::min;

use crate::Solution;

pub fn day5(input: &str) -> Solution {
    struct MappingRange {
        dst: i64,
        src: i64,
        len: i64,
    }

    let mut groups = input.trim().split("\n\n");
    let seeds: Vec<i64> = groups
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut mappings = vec![];
    for mapping in groups {
        let mut mapping_list = vec![];

        for range in mapping.lines().skip(1) {
            let mut iter = range.split(' ');
            mapping_list.push(MappingRange {
                dst: iter.next().unwrap().parse().unwrap(),
                src: iter.next().unwrap().parse().unwrap(),
                len: iter.next().unwrap().parse().unwrap(),
            });
        }

        mappings.push(mapping_list);
    }

    let mut min = seeds[0];
    for mut seed in seeds {
        for map in &mappings {
            for range in map {
                if (range.src..=range.src + range.len).contains(&seed) {
                    seed = seed - range.src + range.dst;
                    break;
                }
            }
        }

        if seed < min {
            min = seed;
        }
    }

    Solution::Int(min)
}

#[derive(Debug)]
struct MappingRange {
    dst: i64,
    src: i64,
    len: i64,
}
fn submap(from_maps: &[MappingRange], to_maps: &[MappingRange]) -> Vec<MappingRange> {
    let mut res = vec![];

    dbg!(to_maps);
    for x in from_maps {
        let mut start = x.dst;
        dbg!(x);

        while start < x.dst + x.len {
            dbg!(start);
            let y = to_maps
                .iter()
                .find(|r| (r.src..r.src + r.len).contains(&start))
                .unwrap();

            let src = start;
            let dst = x.dst - y.src + y.dst;
            let len = min(x.len, y.dst + y.len - dst);
            res.push(MappingRange { dst, src, len });

            start += len;
        }
    }

    res
}

pub fn day5_part2(input: &str) -> Solution {
    let mut groups = input.trim().split("\n\n");
    let mut mappings = vec![];
    let seeds_str: Vec<&str> = groups
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(' ')
        .collect();
    for seed_range in seeds_str.chunks(2) {
        let start: i64 = seed_range[0].parse().unwrap();
        let len = seed_range[1].parse().unwrap();
        mappings.push(MappingRange {
            dst: start,
            src: start,
            len,
        });
    }

    for mapping in groups {
        let mut mapping_list = vec![];

        for range in mapping.lines().skip(1) {
            let mut iter = range.split(' ');
            mapping_list.push(MappingRange {
                dst: iter.next().unwrap().parse().unwrap(),
                src: iter.next().unwrap().parse().unwrap(),
                len: iter.next().unwrap().parse().unwrap(),
            });
        }

        mappings = submap(dbg!(&mappings), &mapping_list);
        println!("{mappings:?}");
    }

    let mut min = i64::MAX;
    for seed_range in seeds_str.chunks(2) {
        let start: i64 = seed_range[0].parse().unwrap();
        let len = seed_range[1].parse().unwrap();
        for i in 0..len {
            let mut seed = start + i;
            for range in &mappings {
                if (range.src..=range.src + range.len).contains(&seed) {
                    seed = seed - range.src + range.dst;
                    break;
                }
            }

            if seed < min {
                min = seed;
            }
        }
    }

    Solution::Int(min)
}

#[test]
#[cfg(test)]
fn test() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
    ";

    assert_eq!(day5(input), Solution::Int(35));
    assert_eq!(day5_part2(input), Solution::Int(46));
}
