use crate::prelude::*;

fn parse_group(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .skip(1)
        .map(|x| parse3(x.split_whitespace()))
        .collect()
}

fn apply_map(m: &[(i64, i64, i64)], v: i64) -> i64 {
    for (dest_start, source_start, range_len) in m {
        if v >= *source_start && v < *source_start + *range_len {
            return *dest_start + v - *source_start;
        }
    }
    v
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i64 {
    let (seed_str, input) = split1(input, "\n\n");
    let seed_str = split1(seed_str, ": ").1;

    let mut v = seed_str.split_whitespace().map(parse1).collect::<Vec<_>>();
    for map in input.split("\n\n") {
        let map = parse_group(map);
        v = v.into_iter().map(|s| apply_map(&map, s)).collect();
    }

    v.into_iter().min().unwrap()
}

fn apply_map2(
    m: &[(i64, i64, i64)],
    intervals: Vec<RangeInclusive<i64>>,
) -> Vec<RangeInclusive<i64>> {
    let mut r = vec![];

    for int in intervals {
        let mut base = vec![int];
        for (dest_start, source_start, range_len) in m {
            base = base
                .into_iter()
                .flat_map(|int| {
                    match compute_overlaps(int, *source_start..=source_start + range_len) {
                        OverlapResult::NonOverlapping { a, .. } => {
                            vec![a]
                        }
                        OverlapResult::Overlapping { overlap, a, .. } => {
                            let offset = dest_start - source_start;
                            r.push(overlap.start() + offset..=overlap.end() + offset);
                            a
                        }
                    }
                })
                .collect();
        }
        r.extend(base);
    }
    r
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i64 {
    let (seed_str, input) = split1(input, "\n\n");
    let seed_str = split1(seed_str, ": ").1;

    let seeds = seed_str.split_whitespace().map(parse1).collect::<Vec<_>>();
    let mut v = seeds
        .chunks(2)
        .map(|x| x[0]..=x[0] + x[1])
        .collect::<Vec<_>>();
    for map in input.split("\n\n") {
        let map = parse_group(map);
        v = apply_map2(&map, v);
    }

    v.into_iter().map(|x| *x.start()).min().unwrap()
}

#[aoc(day5, part1, dumb)]
pub fn part2dumb(input: &str) -> i64 {
    let (seed_str, input) = split1(input, "\n\n");
    let seed_str = split1(seed_str, ": ").1;

    let seeds = seed_str
        .split_whitespace()
        .map(parse1)
        .collect::<Vec<i64>>();
    let mut v = vec![];
    for chunk in seeds.chunks(2) {
        v.extend(chunk[0]..=chunk[0] + chunk[1]);
    }
    for map in input.split("\n\n") {
        let map = parse_group(map);
        v = v.into_iter().map(|s| apply_map(&map, s)).collect();
    }

    v.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"seeds: 79 14 55 13

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
56 93 4"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 46);
    }
}
