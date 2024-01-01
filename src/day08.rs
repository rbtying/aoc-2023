use crate::prelude::*;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i64 {
    let (instructions, lookups) = split1(input, "\n\n");

    let mut map = DefaultHashMap::default();

    for line in lookups.lines() {
        let (from, leftright) = split1(line, " = (");
        let (left, right) = split1(leftright.trim_end_matches(')'), ", ");
        map[from] = (left, right);
    }

    let mut pos = "AAA";
    for (idx, c) in instructions.chars().cycle().enumerate() {
        pos = match c {
            'L' => map[pos].0,
            'R' => map[pos].1,
            _ => unreachable!(),
        };
        if pos == "ZZZ" {
            return (idx + 1) as i64;
        }
    }
    unreachable!()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    let (instructions, lookups) = split1(input, "\n\n");

    let mut map = DefaultHashMap::default();

    for line in lookups.lines() {
        let (from, leftright) = split1(line, " = (");
        let (left, right) = split1(leftright.trim_end_matches(')'), ", ");
        map[from] = (left, right);
    }

    map.keys()
        .filter(|k| k.ends_with('A'))
        .map(|x| {
            find_cycle_generic(
                (instructions.chars().cycle(), x.to_string()),
                |(iter, pos)| {
                    *pos = match iter.next().unwrap() {
                        'L' => map[pos.as_str()].0.to_string(),
                        'R' => map[pos.as_str()].1.to_string(),
                        _ => unreachable!(),
                    };
                },
                |(_, pos)| pos.to_string(),
                |a, _| a.ends_with('Z'),
            )
            .cycle_len
        })
        .reduce(lcm)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    const EXAMPLE2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE2), 6);
    }
}
