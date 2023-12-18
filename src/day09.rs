use crate::prelude::*;

pub fn part1(input: &str) -> i64 {
    let mut s = 0;
    for line in input.lines() {
        let mut values = vec![parse_ints(line)];

        while values.last().unwrap().iter().any(|v| *v != 0) {
            let prev = values.last().unwrap();
            values.push(prev.windows(2).map(|w| w[1] - w[0]).collect());
        }

        for idx in (1..values.len()).rev() {
            let last_val = *values[idx].last().unwrap();
            let last_val2 = *values[idx - 1].last().unwrap();
            values[idx - 1].push(last_val2 + last_val);
        }
        s += values[0].last().unwrap();
    }

    s
}

pub fn part2(input: &str) -> i64 {
    let mut s = 0;
    for line in input.lines() {
        let mut values = vec![parse_ints(line)];

        while values.last().unwrap().iter().any(|v| *v != 0) {
            let prev = values.last().unwrap();
            values.push(prev.windows(2).map(|w| w[1] - w[0]).collect());
        }

        for idx in (1..values.len()).rev() {
            let first_val = *values[idx].first().unwrap();
            let first_val2 = *values[idx - 1].first().unwrap();
            values[idx - 1].insert(0, first_val2 - first_val);
        }
        s += values[0].first().unwrap();
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 114);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 1980437560);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 977);
    }
}
