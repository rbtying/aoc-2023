use crate::prelude::*;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    let mut s = 0;
    for line in input.lines() {
        let y = parse_ints(line);
        let p = polynomial_regression(&(0i64..y.len() as i64).collect::<Vec<_>>(), &y, y.len() - 1);
        s += p.eval(y.len() as i64);
    }

    s
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    let mut s = 0;
    for line in input.lines() {
        let y = parse_ints(line);
        let p = polynomial_regression(&(0i64..y.len() as i64).collect::<Vec<_>>(), &y, y.len() - 1);
        s += p.eval(-1);
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
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2);
    }
}
