use crate::prelude::*;

pub fn part1(input: &str) -> isize {
    let (instructions, lookups) = split1(input, "\n\n");

    let mut map = DefaultHashMap::new();

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
            return (idx + 1) as isize;
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> isize {
    let (instructions, lookups) = split1(input, "\n\n");

    let mut map = DefaultHashMap::new();

    for line in lookups.lines() {
        let (from, leftright) = split1(line, " = (");
        let (left, right) = split1(leftright.trim_end_matches(')'), ", ");
        map[from] = (left, right);
    }

    let mut pos = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    let mut found_zs = vec![-1isize; pos.len()];

    for (steps, c) in instructions.chars().cycle().enumerate() {
        for (idx, x) in pos.iter_mut().enumerate() {
            *x = match c {
                'L' => map[x.as_str()].0.to_string(),
                'R' => map[x.as_str()].1.to_string(),
                _ => unreachable!(),
            };
            if x.ends_with('Z') && found_zs[idx] < 0 {
                found_zs[idx] = (steps + 1) as isize;
            }
        }
        if found_zs.iter().all(|x| *x >= 0) {
            return found_zs.into_iter().reduce(lcm).unwrap();
        }
    }
    unreachable!()
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
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 17873);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE2), 6);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 15746133679061);
    }
}
