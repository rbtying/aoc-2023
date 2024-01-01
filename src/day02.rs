use crate::prelude::*;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (num, line): (u32, &str) = parse_split_once(drop_prefix(line, "Game "), ": ");
        let mut valid = true;
        for view in line.split("; ") {
            for colorseq in view.split(", ") {
                let (ct, color): (u32, &str) = parse_split_once(colorseq, " ");
                valid = valid
                    && match color {
                        "red" if ct > 12 => false,
                        "blue" if ct > 14 => false,
                        "green" if ct > 13 => false,
                        _ => true,
                    };
            }
        }
        if valid {
            sum += num;
        }
    }
    sum
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let line: &str = parse_right(line, ": ");
        let (mut r, mut g, mut b) = (0, 0, 0);

        for view in line.split("; ") {
            for colorseq in view.split(", ") {
                let (ct, color): (u32, &str) = parse_split_once(colorseq, " ");
                match color {
                    "red" => r = r.max(ct),
                    "blue" => b = b.max(ct),
                    "green" => g = g.max(ct),
                    _ => (),
                };
            }
        }
        sum += r * g * b;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2286);
    }
}
