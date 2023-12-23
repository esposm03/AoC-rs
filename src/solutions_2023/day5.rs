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

        mappings = submap(&mappings, &mapping_list);
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

#[derive(Debug, PartialEq, Clone)]
struct MappingRange {
    dst: i64,
    src: i64,
    len: i64,
}
fn submap(from_maps: &[MappingRange], to_maps: &[MappingRange]) -> Vec<MappingRange> {
    let mut res = vec![];

    for x in from_maps {
        let mut start = x.src;

        while start < x.src + x.len {
            // TODO: add identity mappings where they are missing
            let y = to_maps
                .iter()
                .find(|r| (r.src..r.src + r.len).contains(&(x.dst + start - x.src)))
                .unwrap();
            let y_start = x.dst + (start - x.src);
            let y_end = min(y.src + y.len, x.dst + x.len);
            let z_dst = y_start - y.src + y.dst;

            let z = MappingRange {
                dst: z_dst,
                src: start,
                len: y_end - y_start,
            };
            start += z.len;

            res.push(z);
        }
    }

    res
}

#[test]
#[cfg(test)]
fn test_remapping() {
    assert_eq!(
        submap(
            &[MappingRange {
                dst: 200,
                src: 150,
                len: 50,
            }],
            &[MappingRange {
                dst: 250,
                src: 220,
                len: 200,
            }],
        ),
        &[MappingRange {
            dst: 250,
            src: 170,
            len: 30,
        }],
        "Remapping with no full correspondence",
    );

    assert_eq!(
        submap(
            &[MappingRange {
                dst: 200,
                src: 150,
                len: 50,
            }],
            &[MappingRange {
                dst: 250,
                src: 200,
                len: 200,
            }],
        ),
        &[MappingRange {
            dst: 250,
            src: 150,
            len: 50,
        }],
        "Simple remapping",
    );

    assert_eq!(
        submap(
            &[MappingRange {
                dst: 200,
                src: 150,
                len: 50,
            }],
            &[MappingRange {
                dst: 250,
                src: 150,
                len: 200,
            }],
        ),
        &[MappingRange {
            dst: 300,
            src: 150,
            len: 50,
        }],
        "Simple remapping into a larger map",
    );

    assert_eq!(
        submap(
            &[MappingRange {
                dst: 200,
                src: 150,
                len: 50,
            }],
            &[MappingRange {
                dst: 250,
                src: 150,
                len: 200,
            }],
        ),
        &[MappingRange {
            dst: 300,
            src: 150,
            len: 50,
        }],
        "Remapping with an offset into the to_map",
    );
}

#[test]
#[cfg(test)]
fn test_submap() {
    assert_eq!(
        submap(
            &[MappingRange {
                dst: 500,
                src: 100,
                len: 100,
            }],
            &[
                MappingRange {
                    dst: 1000,
                    src: 500,
                    len: 50,
                },
                MappingRange {
                    dst: 2000,
                    src: 550,
                    len: 50,
                },
            ],
        ),
        &[
            MappingRange {
                dst: 1000,
                src: 100,
                len: 50,
            },
            MappingRange {
                dst: 2000,
                src: 150,
                len: 50,
            },
        ],
        "1-to-2 submapping",
    );

    assert_eq!(
        submap(
            &[
                MappingRange {
                    dst: 200,
                    src: 150,
                    len: 50,
                },
                MappingRange {
                    dst: 250,
                    src: 350,
                    len: 50,
                }
            ],
            &[MappingRange {
                dst: 500,
                src: 180,
                len: 200,
            }],
        ),
        &[
            MappingRange {
                dst: 520,
                src: 150,
                len: 50,
            },
            MappingRange {
                dst: 570,
                src: 350,
                len: 50,
            },
        ],
        "2-to-1 submapping",
    );
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

    assert_eq!(day5(input), Solution::Int(35), "part 1");
    assert_eq!(day5_part2(input), Solution::Int(46), "part 2");
}
