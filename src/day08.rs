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
    let mut steps = 0;
    let mut iter = instructions.chars();
    loop {
        match iter.next() {
            Some('L') => {
                steps += 1;
                pos = map[pos].0;
            }
            Some('R') => {
                steps += 1;
                pos = map[pos].1;
            }
            None => {
                iter = instructions.chars();
            }
            _ => unreachable!(),
        }
        if pos == "ZZZ" {
            return steps;
        }
    }
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

    let mut steps = 0;
    let mut iter = instructions.chars();
    loop {
        match iter.next() {
            Some('L') => {
                steps += 1;
                for (idx, x) in pos.iter_mut().enumerate() {
                    *x = map[x.as_str()].0.to_string();
                    if x.ends_with('Z') {
                        found_zs[idx] = steps;
                    }
                }
            }
            Some('R') => {
                steps += 1;
                for (idx, x) in pos.iter_mut().enumerate() {
                    *x = map[x.as_str()].1.to_string();
                    if x.ends_with('Z') {
                        found_zs[idx] = steps;
                    }
                }
            }
            None => {
                iter = instructions.chars();
            }
            _ => unreachable!(),
        }
        if found_zs.iter().all(|x| *x >= 0) {
            break found_zs.into_iter().reduce(lcm).unwrap();
        }
    }
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
