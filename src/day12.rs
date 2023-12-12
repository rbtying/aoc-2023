use crate::prelude::*;

pub fn part1(input: &str) -> isize {
    let mut s = 0;
    for line in input.lines() {
        let (data, lens) = split1(line, " ");
        let lens = parse_ints(lens);

        s += solve_recursive(
            Box::leak(Box::new(data.chars().collect::<Vec<_>>())),
            Box::leak(Box::new(lens)),
            None,
        );
    }
    s
}

#[memoize::memoize]
fn solve_recursive(s: &'static [char], l: &'static [isize], in_seq: Option<isize>) -> isize {
    match in_seq {
        Some(v) if !l.is_empty() => {
            if s.is_empty() {
                // success
                if [v] == l {
                    1
                } else {
                    0
                }
            } else {
                let mut sum = 0;
                if ".?".find(s[0]).is_some() && v == l[0] {
                    sum += solve_recursive(&s[1..], &l[1..], None);
                }
                if "#?".find(s[0]).is_some() {
                    sum += solve_recursive(&s[1..], l, Some(v + 1));
                }
                sum
            }
        }
        Some(_) => 0,
        None => {
            if s.is_empty() {
                if l.is_empty() {
                    1
                } else {
                    0
                }
            } else {
                let mut sum = 0;
                if ".?".find(s[0]).is_some() {
                    sum += solve_recursive(&s[1..], l, None);
                }
                if "#?".find(s[0]).is_some() {
                    sum += solve_recursive(&s[1..], l, Some(1));
                }
                sum
            }
        }
    }
}

pub fn part2(input: &str) -> isize {
    let mut s = 0;
    for line in input.lines() {
        let (data, lens) = split1(line, " ");
        let lens = parse_ints(lens);

        let mut new_data = data.to_string();
        let mut new_lens = lens.clone();
        for _ in 0..4 {
            new_data.push('?');
            new_data.push_str(data);
            new_lens.extend(lens.iter().copied());
        }

        s += solve_recursive(
            Box::leak(Box::new(new_data.chars().collect::<Vec<_>>())),
            Box::leak(Box::new(new_lens)),
            None,
        );
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 7402);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 525152);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 3384337640277);
    }
}
